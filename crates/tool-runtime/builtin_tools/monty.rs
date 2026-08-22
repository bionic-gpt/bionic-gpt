use bashkit::{ExcType, ExtFunctionResult, MontyException, MontyObject, PythonExternalFnHandler};
use rig::tool::{ToolDyn, ToolError};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

const FUNCTIONS_DIR: &str = "/home/user/functions";
const WEB_FUNCTION_NAME: &str = "web_open_url";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFunctionFile {
    pub path: String,
    pub contents: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FunctionCatalogue {
    pub prompt_section: Option<String>,
    pub files: Vec<RuntimeFunctionFile>,
}

#[derive(Clone)]
struct RuntimeOperation {
    function_name: String,
    description: String,
    parameters: Value,
    executor: OperationExecutor,
}

#[derive(Clone)]
enum OperationExecutor {
    OpenApiTool(Arc<dyn ToolDyn>),
    OpenUrl,
}

#[derive(Clone)]
struct IntegrationInfo {
    name: String,
    slug: String,
    operations: Vec<RuntimeOperation>,
}

#[derive(Clone)]
pub struct RuntimeFunctionRegistry {
    integrations: Vec<IntegrationInfo>,
    functions: HashMap<String, RuntimeOperation>,
}

impl RuntimeFunctionRegistry {
    pub async fn load_for_conversation(
        pool: &db::Pool,
        sub: &str,
        conversation_id: i64,
    ) -> Result<Self, String> {
        Self::load_from_parts(Some(pool), Some(sub), Some(conversation_id)).await
    }

    async fn load_from_parts(
        pool: Option<&db::Pool>,
        sub: Option<&str>,
        conversation_id: Option<i64>,
    ) -> Result<Self, String> {
        let (Some(pool), Some(sub), Some(conversation_id)) = (pool, sub, conversation_id) else {
            return Ok(Self::with_builtin_functions(Vec::new(), HashMap::new()));
        };

        let mut client = pool.get().await.map_err(|err| err.to_string())?;
        let transaction = client.transaction().await.map_err(|err| err.to_string())?;
        db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
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

    pub async fn load_for_team(pool: &db::Pool, sub: &str, team_id: i32) -> Result<Self, String> {
        let mut client = pool.get().await.map_err(|err| err.to_string())?;
        let transaction = client.transaction().await.map_err(|err| err.to_string())?;
        db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
            .await
            .map_err(|err| err.to_string())?;

        let mut integrations = Vec::new();
        let mut functions = HashMap::new();
        let mut used_integration_slugs = HashSet::new();
        let mut used_function_names = HashSet::new();

        let connected = db::queries::connections::connected_integrations()
            .bind(&transaction, &team_id)
            .all()
            .await
            .map_err(|err| err.to_string())?;

        transaction.commit().await.map_err(|err| err.to_string())?;

        let system_specs = crate::system_tool_sources::load_system_openapi_specs(pool).await?;
        for system_spec in system_specs {
            let openapi = match crate::BionicOpenAPI::new(&system_spec.spec.spec) {
                Ok(api) => api,
                Err(err) => {
                    tracing::warn!(
                        "Skipping system integration {} with invalid OpenAPI spec: {}",
                        system_spec.spec.slug,
                        err
                    );
                    continue;
                }
            };
            if openapi.has_api_key_security() && system_spec.api_key.is_none() {
                tracing::warn!(
                    "Skipping system integration {} because its API key is not configured",
                    system_spec.spec.slug
                );
                continue;
            }
            let token_provider = system_spec
                .api_key
                .map(|key| Arc::new(crate::StaticTokenProvider::new(key)) as Arc<_>);
            let tools = match openapi.create_tools(token_provider) {
                Ok(tools) => tools,
                Err(err) => {
                    tracing::warn!(
                        "Skipping system integration {} because tools could not be created: {}",
                        system_spec.spec.slug,
                        err
                    );
                    continue;
                }
            };
            let slug = unique_identifier(&system_spec.spec.slug, &mut used_integration_slugs);
            let mut operations = Vec::new();
            for tool in tools {
                let operation_name = unique_identifier(
                    &format!("{}_{}", slug, tool.name()),
                    &mut used_function_names,
                );
                let operation = RuntimeOperation {
                    function_name: operation_name.clone(),
                    description: tool.description(),
                    parameters: tool.parameters(),
                    executor: OperationExecutor::OpenApiTool(tool),
                };
                functions.insert(operation_name, operation.clone());
                operations.push(operation);
            }
            integrations.push(IntegrationInfo {
                name: system_spec.spec.title,
                slug,
                operations,
            });
        }

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
            let mut operations = Vec::new();

            for tool in tools {
                let operation_name = unique_identifier(
                    &format!("{}_{}", slug, tool.name()),
                    &mut used_function_names,
                );
                let operation = RuntimeOperation {
                    function_name: operation_name.clone(),
                    description: tool.description(),
                    parameters: tool.parameters(),
                    executor: OperationExecutor::OpenApiTool(tool),
                };
                functions.insert(operation_name, operation.clone());
                operations.push(operation);
            }

            integrations.push(IntegrationInfo {
                name: integration.integration_name,
                slug,
                operations,
            });
        }

        Ok(Self::with_builtin_functions(integrations, functions))
    }

    fn with_builtin_functions(
        mut integrations: Vec<IntegrationInfo>,
        mut functions: HashMap<String, RuntimeOperation>,
    ) -> Self {
        let web_operation = RuntimeOperation {
            function_name: WEB_FUNCTION_NAME.to_string(),
            description: "Fetch and read text content from a URL supplied by the user.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {"type": "string", "description": "The URL to fetch"}
                },
                "required": ["url"]
            }),
            executor: OperationExecutor::OpenUrl,
        };

        functions.insert(WEB_FUNCTION_NAME.to_string(), web_operation.clone());
        integrations.push(IntegrationInfo {
            name: "Web Fetch".to_string(),
            slug: "web-fetch".to_string(),
            operations: vec![web_operation],
        });

        Self {
            integrations,
            functions,
        }
    }

    pub fn external_function_names(&self) -> Vec<String> {
        let mut names = self.functions.keys().cloned().collect::<Vec<_>>();
        names.sort();
        names
    }

    pub fn python_external_handler(self: Arc<Self>) -> PythonExternalFnHandler {
        Arc::new(move |name, args, kwargs| {
            let registry = Arc::clone(&self);
            Box::pin(async move {
                registry
                    .execute_external_function(&name, &args, &kwargs)
                    .await
            })
        })
    }

    pub fn function_catalogue(&self) -> FunctionCatalogue {
        let mut prompt = String::from(
            "Available function catalogues:\n\
Use run_bash to list /home/user/functions and cat the relevant <integration>.md file. Call functions from Python with python3 inside run_bash.\n",
        );
        let mut files = Vec::new();

        for integration in &self.integrations {
            prompt.push_str(&format!(
                "- {}: {FUNCTIONS_DIR}/{}.md\n",
                integration.name, integration.slug
            ));
            files.push(RuntimeFunctionFile {
                path: format!("{FUNCTIONS_DIR}/{}.md", integration.slug),
                contents: function_markdown(integration).into_bytes(),
            });
        }

        prompt.truncate(prompt.trim_end().len());
        FunctionCatalogue {
            prompt_section: Some(prompt),
            files,
        }
    }

    async fn execute_external_function(
        &self,
        name: &str,
        args: &[MontyObject],
        kwargs: &[(MontyObject, MontyObject)],
    ) -> ExtFunctionResult {
        let Some(operation) = self.functions.get(name) else {
            return ExtFunctionResult::Error(value_error(format!("Unknown function: {name}")));
        };

        let arguments = match call_arguments_to_json(args, kwargs) {
            Ok(arguments) => arguments,
            Err(err) => return ExtFunctionResult::Error(value_error(err)),
        };

        match &operation.executor {
            OperationExecutor::OpenApiTool(tool) => match tool.call(arguments.to_string()).await {
                Ok(result) => match serde_json::from_str::<Value>(&result) {
                    Ok(value) => ExtFunctionResult::Return(json_to_monty(&value)),
                    Err(_) => ExtFunctionResult::Return(MontyObject::String(result)),
                },
                Err(err) => ExtFunctionResult::Error(value_error(tool_error_to_string(err))),
            },
            OperationExecutor::OpenUrl => {
                let Some(url) = arguments.get("url").and_then(Value::as_str) else {
                    return ExtFunctionResult::Error(value_error(
                        "web_open_url requires a string url argument".to_string(),
                    ));
                };

                match crate::builtin_tools::web::open_url(url.to_string()).await {
                    Ok(content) => ExtFunctionResult::Return(json_to_monty(&json!({
                        "content": content
                    }))),
                    Err(err) => ExtFunctionResult::Error(value_error(err.to_string())),
                }
            }
        }
    }
}

pub async fn available_function_catalogue_prompt_section(
    pool: &db::Pool,
    sub: &str,
    team_id: i32,
) -> Result<Option<String>, String> {
    Ok(function_catalogue_for_team(pool, sub, team_id)
        .await?
        .prompt_section)
}

pub async fn function_catalogue_for_team(
    pool: &db::Pool,
    sub: &str,
    team_id: i32,
) -> Result<FunctionCatalogue, String> {
    let registry = RuntimeFunctionRegistry::load_for_team(pool, sub, team_id).await?;
    Ok(registry.function_catalogue())
}

pub async fn function_catalogue_for_conversation(
    pool: &db::Pool,
    sub: &str,
    conversation_id: i64,
) -> Result<FunctionCatalogue, String> {
    let registry =
        RuntimeFunctionRegistry::load_for_conversation(pool, sub, conversation_id).await?;
    Ok(registry.function_catalogue())
}

fn function_markdown(integration: &IntegrationInfo) -> String {
    let mut markdown = format!(
        "# {}\n\nSlug: {}\n\nCall these functions with python3 inside run_bash. Example:\n\n```bash\npython3 -c \"print({}())\"\n```\n\nFunctions:\n",
        integration.name,
        integration.slug,
        integration
            .operations
            .first()
            .map(|operation| operation.function_name.as_str())
            .unwrap_or("<function_name>")
    );

    for operation in &integration.operations {
        let parameters = parameter_names(&operation.parameters);
        let parameter_hint = if parameters.is_empty() {
            "no parameters".to_string()
        } else {
            format!("parameters: {}", parameters.join(", "))
        };
        markdown.push_str(&format!(
            "- {} ({parameter_hint}): {}\n",
            operation.function_name, operation.description
        ));
    }

    markdown
}

fn parameter_names(parameters: &Value) -> Vec<String> {
    let mut names = parameters
        .get("properties")
        .and_then(Value::as_object)
        .map(|properties| properties.keys().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    names.sort();
    names
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
        "function".to_string()
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
            _ => return Err("positional function arguments must be a dict".to_string()),
        }
    }

    if args.len() > 1 {
        return Err("functions accept at most one positional dict".to_string());
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
                    _ => return Err("dict keys passed to functions must be strings".to_string()),
                };
                object.insert(key, monty_to_json(value)?);
            }
            Ok(Value::Object(object))
        }
        _ => Err(format!(
            "unsupported argument type for function call: {}",
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

fn tool_error_to_string(error: ToolError) -> String {
    match error {
        ToolError::JsonError(err) => err.to_string(),
        ToolError::ToolCallError(err) => err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_catalogue_includes_web_by_default() {
        let registry = RuntimeFunctionRegistry::with_builtin_functions(Vec::new(), HashMap::new());
        let catalogue = registry.function_catalogue();

        let prompt = catalogue.prompt_section.expect("expected prompt section");
        assert!(prompt.contains("- Web Fetch: /home/user/functions/web-fetch.md"));
        assert!(prompt.contains("python3"));
        assert!(!prompt.contains("run_python"));

        let web_file = catalogue
            .files
            .iter()
            .find(|file| file.path == "/home/user/functions/web-fetch.md")
            .expect("expected web catalogue file");
        let markdown = String::from_utf8(web_file.contents.clone()).unwrap();
        assert!(markdown.contains("web_open_url"));
        assert!(markdown.contains("parameters: url"));
    }

    #[test]
    fn function_catalogue_summarizes_integrations_without_schemas() {
        let operation = RuntimeOperation {
            function_name: "enterprise_email_api_listemails".to_string(),
            description: "List recent enterprise email messages".to_string(),
            parameters: json!({"type": "object"}),
            executor: OperationExecutor::OpenUrl,
        };
        let registry = RuntimeFunctionRegistry::with_builtin_functions(
            vec![IntegrationInfo {
                name: "Enterprise Email API".to_string(),
                slug: "enterprise_email_api".to_string(),
                operations: vec![operation],
            }],
            HashMap::new(),
        );

        let catalogue = registry.function_catalogue();
        let prompt = catalogue.prompt_section.expect("expected prompt");

        assert!(
            prompt.contains("- Enterprise Email API: /home/user/functions/enterprise_email_api.md")
        );
        assert!(prompt.contains("- Web Fetch: /home/user/functions/web-fetch.md"));
        assert!(!prompt.contains("parameters"));

        let markdown = String::from_utf8(
            catalogue
                .files
                .iter()
                .find(|file| file.path == "/home/user/functions/enterprise_email_api.md")
                .expect("expected integration catalogue")
                .contents
                .clone(),
        )
        .unwrap();
        assert!(markdown.contains("enterprise_email_api_listemails"));
        assert!(!markdown.contains("\"type\""));
    }

    #[test]
    fn call_arguments_accept_keyword_args() {
        let args = call_arguments_to_json(
            &[],
            &[(
                MontyObject::String("url".to_string()),
                MontyObject::String("https://example.com".to_string()),
            )],
        )
        .unwrap();

        assert_eq!(args["url"], "https://example.com");
    }
}
