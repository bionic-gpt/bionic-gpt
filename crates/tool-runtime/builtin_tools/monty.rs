use crate::types::ToolDefinition;
use monty::{MontyRun, RunProgress};
use monty_types::{
    CompileOptions, DictPairs, ExcType, ExtFunctionResult, LimitedTracker, MontyException,
    MontyObject, NameLookupResult, PrintWriter, ResourceLimits,
};
use rig::tool::{ToolDyn, ToolError};
use rig::wasm_compat::WasmBoxedFuture;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

const DEFAULT_TIMEOUT_MS: u64 = 5_000;
const MAX_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_MAX_MEMORY_BYTES: usize = 64 * 1024 * 1024;
const DEFAULT_MAX_ALLOCATIONS: usize = 1_000_000;
const TOOLBOX_INTEGRATIONS_CLASS: &str = "ToolboxIntegrations";
const TOOLBOX_INTEGRATION_CLASS_PREFIX: &str = "ToolboxIntegration:";

#[derive(Debug, Deserialize)]
struct RunPythonArgs {
    code: String,
    timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct SearchToolFunctionsArgs {
    query: String,
}

/// A tool that runs hermetic Python snippets in Monty.
pub struct MontyTool {
    pool: Option<db::Pool>,
    sub: Option<String>,
    conversation_id: Option<i64>,
}

impl MontyTool {
    pub fn new(pool: db::Pool, sub: String, conversation_id: i64) -> Self {
        Self {
            pool: Some(pool),
            sub: Some(sub),
            conversation_id: Some(conversation_id),
        }
    }

    #[cfg(test)]
    fn without_integrations() -> Self {
        Self {
            pool: None,
            sub: None,
            conversation_id: None,
        }
    }
}

impl ToolDyn for MontyTool {
    fn name(&self) -> String {
        get_tool_definition().name
    }

    fn description(&self) -> String {
        get_tool_definition().description
    }

    fn parameters(&self) -> Value {
        get_tool_definition().parameters
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let arguments: RunPythonArgs =
                serde_json::from_str(&args).map_err(ToolError::JsonError)?;

            let timeout = arguments
                .timeout_ms
                .unwrap_or(DEFAULT_TIMEOUT_MS)
                .clamp(100, MAX_TIMEOUT_MS);

            let result = tokio::time::timeout(
                Duration::from_millis(timeout),
                execute_run_python(self, arguments, timeout),
            )
            .await
            .map_err(|_| {
                ToolError::ToolCallError(Box::new(std::io::Error::other(
                    "python execution timed out",
                )))
            })?;

            serde_json::to_string(&result).map_err(ToolError::JsonError)
        })
    }
}

/// A tool that searches Python integration functions available to Monty.
pub struct SearchToolFunctionsTool {
    pool: db::Pool,
    sub: String,
    conversation_id: i64,
}

impl SearchToolFunctionsTool {
    pub fn new(pool: db::Pool, sub: String, conversation_id: i64) -> Self {
        Self {
            pool,
            sub,
            conversation_id,
        }
    }
}

pub async fn preview_integration_functions(
    pool: &db::Pool,
    sub: &str,
    team_id: i32,
) -> Result<Value, String> {
    let registry = IntegrationRegistry::load_for_team(pool, sub, team_id).await?;
    Ok(registry.search_json(""))
}

pub async fn available_integrations_prompt_section(
    pool: &db::Pool,
    sub: &str,
    team_id: i32,
) -> Result<Option<String>, String> {
    let registry = IntegrationRegistry::load_for_team(pool, sub, team_id).await?;
    Ok(registry.prompt_section())
}

impl ToolDyn for SearchToolFunctionsTool {
    fn name(&self) -> String {
        get_search_tool_functions_definition().name
    }

    fn description(&self) -> String {
        get_search_tool_functions_definition().description
    }

    fn parameters(&self) -> Value {
        get_search_tool_functions_definition().parameters
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let arguments: SearchToolFunctionsArgs =
                serde_json::from_str(&args).map_err(ToolError::JsonError)?;

            let registry = IntegrationRegistry::load_from_parts(
                Some(&self.pool),
                Some(&self.sub),
                Some(self.conversation_id),
            )
            .await
            .map_err(|err| ToolError::ToolCallError(Box::new(std::io::Error::other(err))))?;

            let result = registry.search_json(&arguments.query);
            serde_json::to_string(&result).map_err(ToolError::JsonError)
        })
    }
}

pub fn get_tool_definition() -> ToolDefinition {
    ToolDefinition {
        name: "run_python".to_string(),
        description: "Run a short, hermetic Python snippet with Monty. Use this for calculations, data shaping, small programs, and configured integration functions. A preloaded global object named toolbox is available; do not import it. Discover integrations with toolbox.integrations.list() or toolbox.integrations.describe(...), and call functions as toolbox.integrations.<integration>.<operation>(**kwargs). The sandbox has no access to the host filesystem, environment variables, network, or third-party Python packages. Return values and print output are captured.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string",
                    "description": "Python source code to execute."
                },
                "timeout_ms": {
                    "type": "integer",
                    "minimum": 100,
                    "maximum": MAX_TIMEOUT_MS,
                    "description": "Optional execution timeout in milliseconds. Defaults to 5000."
                }
            },
            "required": ["code"]
        }),
    }
}

pub fn get_search_tool_functions_definition() -> ToolDefinition {
    ToolDefinition {
        name: "search_tool_functions".to_string(),
        description: "Search available integration functions callable from run_python. Use this to discover functions for current, external, account-specific, or connected-system information before writing Python code.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search text describing the needed integration, function, data, or action."
                }
            },
            "required": ["query"]
        }),
    }
}

async fn execute_run_python(tool: &MontyTool, arguments: RunPythonArgs, timeout: u64) -> Value {
    if arguments.code.trim().is_empty() {
        return json!({"error": "code is required"});
    }

    let runner = match MontyRun::new(arguments.code, "tool.py", vec![], CompileOptions::default()) {
        Ok(runner) => runner,
        Err(err) => return json!({"error": err.to_string()}),
    };

    let started = Instant::now();
    let mut stdout = String::new();
    let registry = match IntegrationRegistry::load(tool).await {
        Ok(registry) => registry,
        Err(err) => return json!({"error": err}),
    };

    let limits = ResourceLimits::new()
        .max_duration(Duration::from_millis(timeout))
        .max_memory(DEFAULT_MAX_MEMORY_BYTES)
        .max_allocations(DEFAULT_MAX_ALLOCATIONS);

    let mut progress = match runner.start(
        vec![],
        LimitedTracker::new(limits),
        PrintWriter::collect_string(&mut stdout),
    ) {
        Ok(progress) => progress,
        Err(err) => {
            return json!({
                "stdout": stdout,
                "stderr": "",
                "error": err.to_string(),
                "duration_ms": started.elapsed().as_millis()
            });
        }
    };

    let result = loop {
        progress = match progress {
            RunProgress::Complete(result) => break result,
            RunProgress::NameLookup(lookup) => {
                let resolved = if lookup.name == "toolbox" {
                    NameLookupResult::Value(registry.toolbox_object())
                } else {
                    NameLookupResult::Undefined
                };
                match lookup.resume(resolved, PrintWriter::collect_string(&mut stdout)) {
                    Ok(progress) => progress,
                    Err(err) => {
                        return monty_error(stdout, started, err);
                    }
                }
            }
            RunProgress::FunctionCall(call) => {
                let result = registry.execute_function_call(&call);
                match call.resume(result, PrintWriter::collect_string(&mut stdout)) {
                    Ok(progress) => progress,
                    Err(err) => {
                        return monty_error(stdout, started, err);
                    }
                }
            }
            RunProgress::OsCall(call) => {
                let err = MontyException::new(
                    ExcType::RuntimeError,
                    Some("OS access is disabled in this Python sandbox".to_string()),
                );
                match call.resume(err, PrintWriter::collect_string(&mut stdout)) {
                    Ok(progress) => progress,
                    Err(err) => {
                        return monty_error(stdout, started, err);
                    }
                }
            }
            RunProgress::ResolveFutures(futures) => {
                let err = MontyException::new(
                    ExcType::RuntimeError,
                    Some("async external futures are not supported by this tool".to_string()),
                );
                let pending_results = futures
                    .pending_call_ids()
                    .iter()
                    .map(|id| (*id, ExtFunctionResult::Error(err.clone())))
                    .collect();
                match futures.resume(pending_results, PrintWriter::collect_string(&mut stdout)) {
                    Ok(progress) => progress,
                    Err(err) => {
                        return monty_error(stdout, started, err);
                    }
                }
            }
        };
    };

    json!({
        "stdout": stdout,
        "stderr": "",
        "result": serde_json::to_value(&result).unwrap_or_else(|_| json!({"repr": result.to_string()})),
        "repr": result.to_string(),
        "duration_ms": started.elapsed().as_millis()
    })
}

fn monty_error(stdout: String, started: Instant, err: MontyException) -> Value {
    json!({
        "stdout": stdout,
        "stderr": "",
        "error": err.to_string(),
        "duration_ms": started.elapsed().as_millis()
    })
}

#[derive(Clone)]
struct IntegrationOperation {
    operation_name: String,
    path: String,
    description: String,
    parameters: Value,
    tool: Arc<dyn ToolDyn>,
}

struct IntegrationInfo {
    name: String,
    slug: String,
    operations: Vec<IntegrationOperation>,
}

struct IntegrationRegistry {
    integrations: Vec<IntegrationInfo>,
    functions: HashMap<String, IntegrationOperation>,
}

impl IntegrationRegistry {
    async fn load(tool: &MontyTool) -> Result<Self, String> {
        Self::load_from_parts(tool.pool.as_ref(), tool.sub.as_ref(), tool.conversation_id).await
    }

    async fn load_from_parts(
        pool: Option<&db::Pool>,
        sub: Option<&String>,
        conversation_id: Option<i64>,
    ) -> Result<Self, String> {
        let (Some(pool), Some(sub), Some(conversation_id)) = (pool, sub, conversation_id) else {
            return Ok(Self {
                integrations: Vec::new(),
                functions: HashMap::new(),
            });
        };

        let mut client = pool.get().await.map_err(|err| err.to_string())?;
        let transaction = client.transaction().await.map_err(|err| err.to_string())?;
        db::authz::set_row_level_security_user_id(&transaction, sub.clone())
            .await
            .map_err(|err| err.to_string())?;

        let row = transaction
            .query_one(
                "SELECT team_id FROM llm.conversations WHERE id = $1",
                &[&conversation_id],
            )
            .await
            .map_err(|err| err.to_string())?;
        let team_id: i32 = row.get(0);

        transaction.commit().await.map_err(|err| err.to_string())?;

        Self::load_for_team(pool, sub, team_id).await
    }

    async fn load_for_team(pool: &db::Pool, sub: &str, team_id: i32) -> Result<Self, String> {
        let mut client = pool.get().await.map_err(|err| err.to_string())?;
        let transaction = client.transaction().await.map_err(|err| err.to_string())?;
        db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
            .await
            .map_err(|err| err.to_string())?;

        let mut integrations = Vec::new();
        let mut functions = HashMap::new();
        let mut used_integration_slugs = HashSet::new();

        let connected = db::queries::connections::connected_integrations()
            .bind(&transaction, &team_id)
            .all()
            .await
            .map_err(|err| err.to_string())?;

        transaction.commit().await.map_err(|err| err.to_string())?;

        for integration in connected {
            let Some(definition) = integration.definition.as_ref() else {
                continue;
            };
            let openapi = match crate::BionicOpenAPI::new(definition) {
                Ok(api) => api,
                Err(err) => {
                    tracing::warn!(
                        "Skipping integration {} with invalid OpenAPI spec: {}",
                        integration.integration_name,
                        err
                    );
                    continue;
                }
            };

            let token_provider = token_provider_for_connected_integration(
                pool.clone(),
                sub.to_string(),
                &integration,
                &openapi,
            );
            let tools = match openapi.create_tools(token_provider) {
                Ok(tools) => tools,
                Err(err) => {
                    tracing::warn!(
                        "Skipping integration {} because tools could not be created: {}",
                        integration.integration_name,
                        err
                    );
                    continue;
                }
            };

            let slug =
                unique_identifier(&integration.integration_name, &mut used_integration_slugs);
            let mut used_operation_names = HashSet::new();
            let mut operations = Vec::new();

            for tool in tools {
                let operation_name = unique_identifier(&tool.name(), &mut used_operation_names);
                let path = format!("toolbox.integrations.{slug}.{operation_name}");
                let operation = IntegrationOperation {
                    operation_name,
                    path,
                    description: tool.description(),
                    parameters: tool.parameters(),
                    tool,
                };
                functions.insert(
                    format!("{}.{}", slug, operation.operation_name),
                    operation.clone(),
                );
                operations.push(operation);
            }

            integrations.push(IntegrationInfo {
                name: integration.integration_name,
                slug,
                operations,
            });
        }

        Ok(Self {
            integrations,
            functions,
        })
    }

    fn toolbox_object(&self) -> MontyObject {
        dataclass(
            "Toolbox",
            1,
            vec![("integrations", self.integrations_object())],
        )
    }

    fn integrations_object(&self) -> MontyObject {
        let mut attrs = Vec::new();

        for integration in &self.integrations {
            attrs.push((
                integration.slug.as_str(),
                dataclass(
                    &format!("{TOOLBOX_INTEGRATION_CLASS_PREFIX}{}", integration.slug),
                    stable_type_id(&integration.slug),
                    vec![],
                ),
            ));
        }

        dataclass(TOOLBOX_INTEGRATIONS_CLASS, 2, attrs)
    }

    fn execute_function_call<T: monty_types::ResourceTracker>(
        &self,
        call: &monty::FunctionCall<T>,
    ) -> ExtFunctionResult {
        let Some((receiver_name, receiver_args)) = method_receiver(&call.args) else {
            return ExtFunctionResult::NotFound(call.function_name.clone());
        };

        if receiver_name == TOOLBOX_INTEGRATIONS_CLASS {
            if call.function_name == "list" {
                return ExtFunctionResult::Return(json_to_monty(&self.list_json()));
            }

            if call.function_name == "describe" {
                return match self.describe_json(receiver_args, &call.kwargs) {
                    Ok(value) => ExtFunctionResult::Return(json_to_monty(&value)),
                    Err(err) => ExtFunctionResult::Error(value_error(err)),
                };
            }

            return ExtFunctionResult::NotFound(call.function_name.clone());
        }

        let Some(integration_slug) = receiver_name.strip_prefix(TOOLBOX_INTEGRATION_CLASS_PREFIX)
        else {
            return ExtFunctionResult::NotFound(call.function_name.clone());
        };

        let Some(operation) = self
            .functions
            .get(&format!("{integration_slug}.{}", call.function_name))
        else {
            return ExtFunctionResult::NotFound(call.function_name.clone());
        };

        let args = match call_arguments_to_json(receiver_args, &call.kwargs) {
            Ok(args) => args,
            Err(err) => return ExtFunctionResult::Error(value_error(err)),
        };

        match block_on_tool_call(operation.tool.clone(), args.to_string()) {
            Ok(result) => match serde_json::from_str::<Value>(&result) {
                Ok(value) => ExtFunctionResult::Return(json_to_monty(&value)),
                Err(_) => ExtFunctionResult::Return(MontyObject::String(result)),
            },
            Err(err) => ExtFunctionResult::Error(value_error(err.to_string())),
        }
    }

    fn list_json(&self) -> Value {
        Value::Array(
            self.integrations
                .iter()
                .map(|integration| {
                    json!({
                        "name": integration.name,
                        "slug": integration.slug,
                        "operations": integration.operations.iter().map(|operation| {
                            json!({
                                "name": operation.operation_name,
                                "path": operation.path,
                                "description": operation.description,
                                "parameters": operation.parameters,
                            })
                        }).collect::<Vec<_>>()
                    })
                })
                .collect(),
        )
    }

    fn search_json(&self, query: &str) -> Value {
        let query_terms = search_terms(query);
        let mut matches = Vec::new();

        for integration in &self.integrations {
            for operation in &integration.operations {
                if query_terms.is_empty() || operation_matches(integration, operation, &query_terms)
                {
                    matches.push(json!({
                        "path": operation.path,
                        "integration": integration.slug,
                        "operation": operation.operation_name,
                        "description": operation.description,
                        "parameters": operation.parameters,
                    }));
                }
            }
        }

        Value::Array(matches)
    }

    fn prompt_section(&self) -> Option<String> {
        if self.integrations.is_empty() {
            return None;
        }

        let mut prompt = String::from(
            "Available integrations:\n\
Use run_python to inspect and call integrations. Inside Python, use toolbox.integrations.list()/describe() or search_tool_functions to inspect callable functions.\n",
        );

        for integration in &self.integrations {
            let operations = integration
                .operations
                .iter()
                .map(|operation| operation.operation_name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            prompt.push_str(&format!(
                "- {} ({}): {}\n",
                integration.name, integration.slug, operations
            ));
        }

        prompt.truncate(prompt.trim_end().len());
        Some(prompt)
    }

    fn describe_json(
        &self,
        args: &[MontyObject],
        kwargs: &[(MontyObject, MontyObject)],
    ) -> Result<Value, String> {
        let input = call_arguments_to_json(args, kwargs)?;
        let integration = input.get("integration").and_then(Value::as_str);
        let operation = input.get("operation").and_then(Value::as_str);

        let Some(integration_slug) = integration else {
            return Ok(self.list_json());
        };

        let Some(info) = self
            .integrations
            .iter()
            .find(|info| info.slug == integration_slug || info.name == integration_slug)
        else {
            return Err(format!("Unknown integration: {integration_slug}"));
        };

        if let Some(operation_name) = operation {
            let Some(op) = info
                .operations
                .iter()
                .find(|op| op.operation_name == operation_name)
            else {
                return Err(format!(
                    "Unknown operation for integration {integration_slug}: {operation_name}"
                ));
            };

            return Ok(json!({
                "integration": info.slug,
                "operation": op.operation_name,
                "path": op.path,
                "description": op.description,
                "parameters": op.parameters,
            }));
        }

        Ok(json!({
            "name": info.name,
            "slug": info.slug,
            "operations": info.operations.iter().map(|operation| {
                json!({
                    "name": operation.operation_name,
                    "path": operation.path,
                    "description": operation.description,
                    "parameters": operation.parameters,
                })
            }).collect::<Vec<_>>()
        }))
    }
}

fn method_receiver(args: &[MontyObject]) -> Option<(&str, &[MontyObject])> {
    let Some(MontyObject::Dataclass { name, .. }) = args.first() else {
        return None;
    };
    Some((name.as_str(), &args[1..]))
}

fn block_on_tool_call(tool: Arc<dyn ToolDyn>, args: String) -> Result<String, ToolError> {
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        tokio::task::block_in_place(|| handle.block_on(tool.call(args)))
    } else {
        futures::executor::block_on(tool.call(args))
    }
}

fn token_provider_for_connected_integration(
    pool: db::Pool,
    sub: String,
    integration: &db::ConnectedIntegration,
    bionic_api: &crate::BionicOpenAPI,
) -> Option<Arc<dyn crate::TokenProvider>> {
    if let Some(conn_id) = integration.oauth2_connection_id {
        let token = integration.bearer_token.clone();
        if let Some(config) = bionic_api.get_oauth2_config() {
            return Some(Arc::new(crate::tool_auth::OAuth2TokenProvider::new(
                pool,
                sub,
                conn_id,
                token,
                integration.refresh_token.clone(),
                integration.expires_at,
                config,
            )));
        }
        return token.map(|token| Arc::new(crate::StaticTokenProvider::new(token)) as Arc<_>);
    }

    integration
        .bearer_token
        .clone()
        .map(|token| Arc::new(crate::StaticTokenProvider::new(token)) as Arc<_>)
}

fn dataclass(name: &str, type_id: u64, attrs: Vec<(&str, MontyObject)>) -> MontyObject {
    MontyObject::Dataclass {
        name: name.to_string(),
        type_id,
        field_names: attrs.iter().map(|(name, _)| (*name).to_string()).collect(),
        attrs: attrs
            .into_iter()
            .map(|(key, value)| (MontyObject::String(key.to_string()), value))
            .collect::<DictPairs>(),
        frozen: true,
    }
}

fn stable_type_id(value: &str) -> u64 {
    value.bytes().fold(10_000_u64, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(byte as u64)
    })
}

fn unique_identifier(value: &str, used: &mut HashSet<String>) -> String {
    let base = sanitize_identifier(value);
    let mut candidate = base.clone();
    let mut suffix = 2;
    while used.contains(&candidate) {
        candidate = format!("{base}_{suffix}");
        suffix += 1;
    }
    used.insert(candidate.clone());
    candidate
}

fn sanitize_identifier(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    while out.contains("__") {
        out = out.replace("__", "_");
    }
    let out = out.trim_matches('_').to_string();
    let mut out = if out.is_empty() {
        "integration".to_string()
    } else {
        out
    };
    if out.chars().next().is_some_and(|ch| ch.is_ascii_digit()) || is_python_keyword(&out) {
        out = format!("_{out}");
    }
    out
}

fn is_python_keyword(value: &str) -> bool {
    matches!(
        value,
        "false"
            | "none"
            | "true"
            | "and"
            | "as"
            | "assert"
            | "async"
            | "await"
            | "break"
            | "class"
            | "continue"
            | "def"
            | "del"
            | "elif"
            | "else"
            | "except"
            | "finally"
            | "for"
            | "from"
            | "global"
            | "if"
            | "import"
            | "in"
            | "is"
            | "lambda"
            | "nonlocal"
            | "not"
            | "or"
            | "pass"
            | "raise"
            | "return"
            | "try"
            | "while"
            | "with"
            | "yield"
    )
}

fn call_arguments_to_json(
    args: &[MontyObject],
    kwargs: &[(MontyObject, MontyObject)],
) -> Result<Value, String> {
    let mut object = Map::new();

    if let Some(first) = args.first() {
        match monty_to_json(first)? {
            Value::Object(map) => object.extend(map),
            _ => return Err("positional integration arguments must be a dict".to_string()),
        }
    }

    if args.len() > 1 {
        return Err("integration functions accept at most one positional dict".to_string());
    }

    for (key, value) in kwargs {
        let key = match key {
            MontyObject::String(key) => key.clone(),
            _ => return Err("keyword argument names must be strings".to_string()),
        };
        object.insert(key, monty_to_json(value)?);
    }

    Ok(Value::Object(object))
}

fn search_terms(query: &str) -> Vec<String> {
    query
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|term| !term.is_empty())
        .map(|term| term.to_ascii_lowercase())
        .filter(|term| !is_stop_word(term))
        .collect()
}

fn is_stop_word(term: &str) -> bool {
    matches!(
        term,
        "a" | "an"
            | "and"
            | "are"
            | "for"
            | "from"
            | "i"
            | "me"
            | "my"
            | "of"
            | "please"
            | "show"
            | "summarize"
            | "summary"
            | "the"
            | "to"
            | "with"
    )
}

fn operation_matches(
    integration: &IntegrationInfo,
    operation: &IntegrationOperation,
    query_terms: &[String],
) -> bool {
    let haystack = format!(
        "{} {} {} {} {}",
        integration.name,
        integration.slug,
        operation.operation_name,
        operation.description,
        operation.parameters
    )
    .to_ascii_lowercase();

    query_terms.iter().any(|term| {
        equivalent_terms(term)
            .iter()
            .any(|candidate| haystack.contains(candidate))
    })
}

fn equivalent_terms(term: &str) -> Vec<&str> {
    match term {
        "inbox" | "mailbox" | "mail" | "email" | "emails" | "message" | "messages" => {
            vec!["inbox", "mailbox", "mail", "email", "message", "messages"]
        }
        "calendar" | "calendars" | "meeting" | "meetings" | "event" | "events" => {
            vec!["calendar", "meeting", "event"]
        }
        "crm" | "customer" | "customers" | "account" | "accounts" => {
            vec!["crm", "customer", "account"]
        }
        "ticket" | "tickets" | "issue" | "issues" => vec!["ticket", "issue"],
        _ => vec![term],
    }
}

fn monty_to_json(value: &MontyObject) -> Result<Value, String> {
    match value {
        MontyObject::None => Ok(Value::Null),
        MontyObject::Bool(value) => Ok(Value::Bool(*value)),
        MontyObject::Int(value) => Ok(json!(value)),
        MontyObject::BigInt(value) => Ok(Value::String(value.to_string())),
        MontyObject::Float(value) => Ok(json!(value)),
        MontyObject::String(value) | MontyObject::Path(value) => Ok(Value::String(value.clone())),
        MontyObject::Bytes(value) => Ok(Value::String(String::from_utf8_lossy(value).to_string())),
        MontyObject::List(values) | MontyObject::Tuple(values) => values
            .iter()
            .map(monty_to_json)
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        MontyObject::Dict(pairs) => {
            let mut object = Map::new();
            for (key, value) in pairs {
                let key = match key {
                    MontyObject::String(key) => key.clone(),
                    _ => return Err("dict keys passed to integrations must be strings".to_string()),
                };
                object.insert(key, monty_to_json(value)?);
            }
            Ok(Value::Object(object))
        }
        _ => Err(format!(
            "unsupported argument type for integration call: {}",
            value.type_name()
        )),
    }
}

fn json_to_monty(value: &Value) -> MontyObject {
    match value {
        Value::Null => MontyObject::None,
        Value::Bool(value) => MontyObject::Bool(*value),
        Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                MontyObject::Int(value)
            } else if let Some(value) = value.as_f64() {
                MontyObject::Float(value)
            } else {
                MontyObject::String(value.to_string())
            }
        }
        Value::String(value) => MontyObject::String(value.clone()),
        Value::Array(values) => MontyObject::List(values.iter().map(json_to_monty).collect()),
        Value::Object(object) => MontyObject::Dict(
            object
                .iter()
                .map(|(key, value)| (MontyObject::String(key.clone()), json_to_monty(value)))
                .collect(),
        ),
    }
}

fn value_error(error: String) -> MontyException {
    MontyException::new(ExcType::ValueError, Some(error))
}

#[cfg(test)]
fn execute_run_python_without_integrations(arguments: RunPythonArgs) -> Value {
    futures::executor::block_on(execute_run_python(
        &MontyTool::without_integrations(),
        arguments,
        DEFAULT_TIMEOUT_MS,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tool_definition() {
        let tool = get_tool_definition();
        assert_eq!(tool.name, "run_python");
        assert!(tool.description.contains("toolbox.integrations.list()"));
        assert!(tool.description.contains("do not import it"));
        assert!(!tool.description.contains("Bionic"));
        assert!(!tool.description.contains("bionic"));
    }

    #[test]
    fn test_get_search_tool_functions_definition() {
        let tool = get_search_tool_functions_definition();
        assert_eq!(tool.name, "search_tool_functions");
        assert!(!tool.description.contains("Bionic"));
        assert!(!tool.description.contains("bionic"));
    }

    #[test]
    fn test_rejects_empty_code() {
        let result = execute_run_python_without_integrations(RunPythonArgs {
            code: "   ".to_string(),
            timeout_ms: None,
        });
        assert_eq!(result["error"], "code is required");
    }

    #[test]
    fn test_toolbox_integrations_list_without_integrations() {
        let result = execute_run_python_without_integrations(RunPythonArgs {
            code: "toolbox.integrations.list()".to_string(),
            timeout_ms: None,
        });
        assert_eq!(result["stdout"], "");
        assert_eq!(result["result"]["List"], json!([]));
    }

    #[test]
    fn test_search_tool_functions_matches_keywords() {
        let registry = IntegrationRegistry {
            integrations: vec![IntegrationInfo {
                name: "Coin Market".to_string(),
                slug: "coin_market".to_string(),
                operations: vec![IntegrationOperation {
                    operation_name: "get_quote".to_string(),
                    path: "toolbox.integrations.coin_market.get_quote".to_string(),
                    description: "Get bitcoin price and crypto market quote".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "symbol": {"type": "string"}
                        }
                    }),
                    tool: Arc::new(crate::builtin_tools::time_date::TimeDateTool),
                }],
            }],
            functions: HashMap::new(),
        };

        let result = registry.search_json("bitcoin price");
        assert_eq!(
            result[0]["path"],
            "toolbox.integrations.coin_market.get_quote"
        );
        assert_eq!(result[0]["integration"], "coin_market");
        assert_eq!(result[0]["operation"], "get_quote");
    }

    #[test]
    fn test_search_tool_functions_matches_inbox_synonym() {
        let registry = IntegrationRegistry {
            integrations: vec![IntegrationInfo {
                name: "Enterprise Email API".to_string(),
                slug: "enterprise_email_api".to_string(),
                operations: vec![IntegrationOperation {
                    operation_name: "listEmails".to_string(),
                    path: "toolbox.integrations.enterprise_email_api.listEmails".to_string(),
                    description: "List recent enterprise email messages".to_string(),
                    parameters: json!({"type": "object"}),
                    tool: Arc::new(crate::builtin_tools::time_date::TimeDateTool),
                }],
            }],
            functions: HashMap::new(),
        };

        let result = registry.search_json("summarize my inbox");
        assert_eq!(
            result[0]["path"],
            "toolbox.integrations.enterprise_email_api.listEmails"
        );
        assert_eq!(result[0]["integration"], "enterprise_email_api");
        assert_eq!(result[0]["operation"], "listEmails");
    }

    #[test]
    fn test_search_tool_functions_no_match_returns_empty_list() {
        let registry = IntegrationRegistry {
            integrations: vec![IntegrationInfo {
                name: "Enterprise Email API".to_string(),
                slug: "enterprise_email_api".to_string(),
                operations: vec![IntegrationOperation {
                    operation_name: "listEmails".to_string(),
                    path: "toolbox.integrations.enterprise_email_api.listEmails".to_string(),
                    description: "List recent enterprise email messages".to_string(),
                    parameters: json!({"type": "object"}),
                    tool: Arc::new(crate::builtin_tools::time_date::TimeDateTool),
                }],
            }],
            functions: HashMap::new(),
        };

        let result = registry.search_json("weather forecast");
        assert_eq!(result, json!([]));
    }

    #[test]
    fn test_prompt_section_summarizes_integrations_without_schemas_or_paths() {
        let registry = IntegrationRegistry {
            integrations: vec![IntegrationInfo {
                name: "Enterprise Email API".to_string(),
                slug: "enterprise_email_api".to_string(),
                operations: vec![IntegrationOperation {
                    operation_name: "listEmails".to_string(),
                    path: "toolbox.integrations.enterprise_email_api.listEmails".to_string(),
                    description: "List recent enterprise email messages".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "limit": {"type": "integer"}
                        }
                    }),
                    tool: Arc::new(crate::builtin_tools::time_date::TimeDateTool),
                }],
            }],
            functions: HashMap::new(),
        };

        let prompt = registry.prompt_section().unwrap();
        assert!(prompt.contains("Available integrations:"));
        assert!(prompt.contains("- Enterprise Email API (enterprise_email_api): listEmails"));
        assert!(prompt.contains("run_python"));
        assert!(prompt.contains("search_tool_functions"));
        assert!(!prompt.contains("List recent enterprise email messages"));
        assert!(!prompt.contains("toolbox.integrations.enterprise_email_api.listEmails"));
        assert!(!prompt.contains("properties"));
        assert!(!prompt.contains("limit"));
    }

    #[test]
    fn test_preview_integration_functions_without_context_returns_empty_list() {
        let registry = IntegrationRegistry {
            integrations: Vec::new(),
            functions: HashMap::new(),
        };
        assert_eq!(registry.search_json(""), json!([]));
        assert_eq!(registry.prompt_section(), None);
    }

    #[test]
    fn test_sanitize_identifier() {
        let mut used = HashSet::new();
        assert_eq!(unique_identifier("My API", &mut used), "my_api");
        assert_eq!(unique_identifier("My API", &mut used), "my_api_2");
        assert_eq!(unique_identifier("123", &mut used), "_123");
    }
}
