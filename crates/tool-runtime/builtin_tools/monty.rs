use crate::{ToolDyn, ToolError};
use base64::Engine;
use bashkit::{
    ExcType, ExtFunctionResult, FileSystem, MontyException, MontyObject, PythonExternalFnHandler,
};
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
    byte_parameters: Vec<FileParameterMapping>,
    executor: OperationExecutor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FileParameterMapping {
    api_parameter: String,
    path_parameter: String,
    multiple: bool,
}

#[derive(Clone)]
enum OperationExecutor {
    OpenApiTool(Arc<dyn ToolDyn>),
    OpenUrl,
    ScheduledTask(crate::scheduled_tasks::Operation),
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
    scheduled_context: Option<crate::scheduled_tasks::Context>,
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
                "SELECT user_id, team_id, project_id FROM llm.conversations WHERE id = $1 AND user_id = current_app_user() AND team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user())",
                &[&conversation_id],
            )
            .await
            .map_err(|err| err.to_string())?;
        let model_id: i32 = transaction
            .query_one(
                "SELECT model_id FROM llm.chats WHERE conversation_id = $1 ORDER BY id DESC LIMIT 1",
                &[&conversation_id],
            )
            .await
            .map_err(|err| err.to_string())?
            .get(0);
        let team_id: i32 = row.get(1);
        let project_id: Option<i32> = row.get(2);

        transaction.commit().await.map_err(|err| err.to_string())?;

        let mut registry = Self::load_for_team(pool, sub, team_id).await?;
        registry.add_scheduled_task_functions(crate::scheduled_tasks::Context {
            pool: pool.clone(),
            sub: sub.to_string(),
            conversation_id,
            model_id,
            team_id,
            project_id,
        });
        Ok(registry)
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
        let server_overrides = crate::system_tool_sources::openapi_server_overrides();
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
            let base_url_override = server_overrides
                .get(&system_spec.spec.slug)
                .map(String::as_str);
            let tools = match openapi.create_tools_with_base_url(token_provider, base_url_override)
            {
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
                let operation = runtime_operation(operation_name.clone(), tool);
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
                let operation = runtime_operation(operation_name.clone(), tool);
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
            byte_parameters: Vec::new(),
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
            scheduled_context: None,
        }
    }

    fn add_scheduled_task_functions(&mut self, context: crate::scheduled_tasks::Context) {
        self.scheduled_context = Some(context.clone());
        let definitions = [
            (
                "create_scheduled_task",
                "Create a prompt that runs on a cron schedule.",
                crate::scheduled_tasks::Operation::Create,
                json!({"type":"object","properties":{"name":{"type":"string"},"cron":{"type":"string"},"timezone":{"type":"string"},"prompt":{"type":"string"}},"required":["name","cron","timezone","prompt"]}),
            ),
            (
                "list_scheduled_tasks",
                "List scheduled tasks owned by the current user.",
                crate::scheduled_tasks::Operation::List,
                json!({"type":"object","properties":{}}),
            ),
            (
                "update_scheduled_task",
                "Update a scheduled task owned by the current user.",
                crate::scheduled_tasks::Operation::Update,
                json!({"type":"object","properties":{"task_id":{"type":"integer"},"name":{"type":"string"},"cron":{"type":"string"},"timezone":{"type":"string"},"prompt":{"type":"string"},"enabled":{"type":"boolean"}},"required":["task_id"]}),
            ),
            (
                "delete_scheduled_task",
                "Delete a scheduled task owned by the current user.",
                crate::scheduled_tasks::Operation::Delete,
                json!({"type":"object","properties":{"task_id":{"type":"integer"}},"required":["task_id"]}),
            ),
        ];
        for (name, description, operation, parameters) in definitions {
            let runtime_operation = RuntimeOperation {
                function_name: name.to_string(),
                description: description.to_string(),
                parameters,
                byte_parameters: Vec::new(),
                executor: OperationExecutor::ScheduledTask(operation),
            };
            self.functions
                .insert(name.to_string(), runtime_operation.clone());
        }
        self.integrations.push(IntegrationInfo {
            name: "Scheduled Tasks".to_string(),
            slug: "scheduled-tasks".to_string(),
            operations: self
                .functions
                .values()
                .filter(|operation| {
                    matches!(&operation.executor, OperationExecutor::ScheduledTask(_))
                })
                .cloned()
                .collect(),
        });
    }

    pub fn external_function_names(&self) -> Vec<String> {
        let mut names = self.functions.keys().cloned().collect::<Vec<_>>();
        names.sort();
        names
    }

    pub fn python_external_handler(self: Arc<Self>) -> PythonExternalFnHandler {
        self.python_external_handler_with_optional_fs(None)
    }

    pub fn python_external_handler_with_fs(
        self: Arc<Self>,
        fs: Arc<dyn FileSystem>,
    ) -> PythonExternalFnHandler {
        self.python_external_handler_with_optional_fs(Some(fs))
    }

    fn python_external_handler_with_optional_fs(
        self: Arc<Self>,
        fs: Option<Arc<dyn FileSystem>>,
    ) -> PythonExternalFnHandler {
        Arc::new(move |name, args, kwargs| {
            let registry = Arc::clone(&self);
            let fs = fs.clone();
            Box::pin(async move {
                registry
                    .execute_external_function(&name, &args, &kwargs, fs)
                    .await
            })
        })
    }

    pub fn function_catalogue(&self) -> FunctionCatalogue {
        let mut prompt = String::from(
            "Available function catalogues:\n\
Use read_file or run_bash to inspect `/home/user/functions`, then read the relevant `.md` file before calling an integration with run_python. The file contains the exact function names, parameters, and usage examples.\n",
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
        fs: Option<Arc<dyn FileSystem>>,
    ) -> ExtFunctionResult {
        let Some(operation) = self.functions.get(name) else {
            return ExtFunctionResult::Error(value_error(format!("Unknown function: {name}")));
        };

        let mut arguments = match call_arguments_to_json(args, kwargs) {
            Ok(arguments) => arguments,
            Err(err) => return ExtFunctionResult::Error(value_error(err)),
        };

        if !operation.byte_parameters.is_empty() {
            let Some(ref fs) = fs else {
                return ExtFunctionResult::Error(value_error(
                    "file-backed functions require a Bashkit filesystem".to_string(),
                ));
            };
            for mapping in &operation.byte_parameters {
                let paths = if mapping.multiple {
                    let Some(paths) = arguments
                        .get(&mapping.path_parameter)
                        .and_then(Value::as_array)
                    else {
                        return ExtFunctionResult::Error(value_error(format!(
                            "{} must be a list of VFS attachment or output paths",
                            mapping.path_parameter
                        )));
                    };
                    let mut values = Vec::with_capacity(paths.len());
                    for path in paths {
                        let Some(path) = path.as_str() else {
                            return ExtFunctionResult::Error(value_error(format!(
                                "{} entries must be VFS attachment or output paths",
                                mapping.path_parameter
                            )));
                        };
                        values.push(path.to_string());
                    }
                    values
                } else {
                    let Some(path) = arguments
                        .get(&mapping.path_parameter)
                        .and_then(Value::as_str)
                    else {
                        return ExtFunctionResult::Error(value_error(format!(
                            "{} must be a VFS attachment or output path",
                            mapping.path_parameter
                        )));
                    };
                    vec![path.to_string()]
                };

                let mut files = Vec::with_capacity(paths.len());
                for path in paths {
                    if !is_allowed_file_path(&path) {
                        return ExtFunctionResult::Error(value_error(format!(
                            "{} must refer to /home/user/attachments or /home/user/output",
                            mapping.path_parameter
                        )));
                    }
                    let bytes = match fs.read_file(std::path::Path::new(&path)).await {
                        Ok(bytes) => bytes,
                        Err(err) => {
                            return ExtFunctionResult::Error(value_error(format!(
                                "failed to read {path}: {err}"
                            )))
                        }
                    };
                    let Some(filename) = std::path::Path::new(&path)
                        .file_name()
                        .and_then(|name| name.to_str())
                    else {
                        return ExtFunctionResult::Error(value_error(format!(
                            "failed to determine a filename for {path}"
                        )));
                    };
                    files.push(json!({
                        "__bionic_file": true,
                        "content_base64": base64::engine::general_purpose::STANDARD.encode(bytes),
                        "filename": filename,
                    }));
                }
                arguments[&mapping.api_parameter] = if mapping.multiple {
                    Value::Array(files)
                } else {
                    files.into_iter().next().unwrap_or(Value::Null)
                };
                arguments
                    .as_object_mut()
                    .unwrap()
                    .remove(&mapping.path_parameter);
            }
        }

        match &operation.executor {
            OperationExecutor::OpenApiTool(tool) => match tool.call(arguments.to_string()).await {
                Ok(result) => match serde_json::from_str::<Value>(&result) {
                    Ok(mut value) => {
                        if let Some(fs) = fs.as_ref() {
                            match persist_binary_result(
                                fs,
                                &value,
                                &arguments,
                                &operation.function_name,
                            )
                            .await
                            {
                                Ok(Some(path)) => {
                                    if let Some(object) = value.as_object_mut() {
                                        object.insert("path".to_string(), Value::String(path));
                                    }
                                }
                                Ok(None) => {}
                                Err(error) => return ExtFunctionResult::Error(value_error(error)),
                            }
                        }
                        ExtFunctionResult::Return(json_to_monty(&value))
                    }
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
            OperationExecutor::ScheduledTask(operation) => {
                let Some(context) = self.scheduled_context.as_ref() else {
                    return ExtFunctionResult::Error(value_error(
                        "scheduled tasks require an active conversation".to_string(),
                    ));
                };
                match crate::scheduled_tasks::execute(context, *operation, arguments).await {
                    Ok(value) => ExtFunctionResult::Return(json_to_monty(&value)),
                    Err(error) => ExtFunctionResult::Error(value_error(error)),
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

pub async fn available_function_catalogue_prompt_section_for_conversation(
    pool: &db::Pool,
    sub: &str,
    conversation_id: i64,
) -> Result<Option<String>, String> {
    Ok(
        function_catalogue_for_conversation(pool, sub, conversation_id)
            .await?
            .prompt_section,
    )
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
    let example = integration
        .operations
        .first()
        .map(operation_example)
        .unwrap_or_else(|| "print(<function_name>())".to_string());
    let mut markdown = format!(
        "# {}\n\nSlug: {}\n\nCall these functions directly by name with run_python. These markdown files are documentation, not Python modules; do not use `from functions import ...`. Example:\n\n```python\n{}\n```\n\nFunctions:\n",
        integration.name,
        integration.slug,
        example
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

fn runtime_operation(function_name: String, tool: Arc<dyn ToolDyn>) -> RuntimeOperation {
    let original_parameters = tool.parameters();
    let (parameters, byte_parameters) = expose_file_parameters(original_parameters);
    RuntimeOperation {
        function_name,
        description: tool.description(),
        parameters,
        byte_parameters,
        executor: OperationExecutor::OpenApiTool(tool),
    }
}

fn expose_file_parameters(mut parameters: Value) -> (Value, Vec<FileParameterMapping>) {
    let mut mappings = Vec::new();
    let Some(properties) = parameters
        .get_mut("properties")
        .and_then(Value::as_object_mut)
    else {
        return (parameters, mappings);
    };
    let file_parameters = properties
        .iter()
        .filter_map(|(name, schema)| {
            let direct_format = schema.get("format").and_then(Value::as_str);
            let multiple = schema.get("type").and_then(Value::as_str) == Some("array");
            let item_format = schema
                .get("items")
                .and_then(|items| items.get("format"))
                .and_then(Value::as_str);
            (matches!(direct_format, Some("byte" | "binary"))
                || multiple && matches!(item_format, Some("byte" | "binary")))
            .then_some((name.clone(), multiple))
        })
        .collect::<Vec<_>>();
    for (index, (api_name, multiple)) in file_parameters.into_iter().enumerate() {
        let path_name = if index == 0 {
            if multiple {
                "file_paths".to_string()
            } else {
                "file_path".to_string()
            }
        } else if multiple {
            format!("{api_name}_file_paths")
        } else {
            format!("{api_name}_file_path")
        };
        properties.remove(&api_name);
        let path_schema = if multiple {
            json!({
                "type": "array",
                "items": {"type": "string"},
                "description": "Paths to documents in /home/user/attachments or /home/user/output."
            })
        } else {
            json!({
                "type": "string",
                "description": "Path to the document in /home/user/attachments or /home/user/output."
            })
        };
        properties.insert(path_name.clone(), path_schema);
        mappings.push(FileParameterMapping {
            api_parameter: api_name,
            path_parameter: path_name,
            multiple,
        });
    }
    if let Some(required) = parameters.get_mut("required").and_then(Value::as_array_mut) {
        for value in required {
            if let Some(name) = value.as_str() {
                if let Some(mapping) = mappings
                    .iter()
                    .find(|mapping| mapping.api_parameter == name)
                {
                    *value = Value::String(mapping.path_parameter.clone());
                }
            }
        }
    }
    (parameters, mappings)
}

fn is_allowed_file_path(path: &str) -> bool {
    path.starts_with("/home/user/attachments/") || path.starts_with("/home/user/output/")
}

async fn persist_binary_result(
    fs: &Arc<dyn FileSystem>,
    value: &Value,
    arguments: &Value,
    function_name: &str,
) -> Result<Option<String>, String> {
    if value.get("__bionic_binary") != Some(&Value::Bool(true)) {
        return Ok(None);
    }

    let encoded = value
        .get("content_base64")
        .and_then(Value::as_str)
        .ok_or_else(|| "binary tool response did not include content_base64".to_string())?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|error| format!("invalid binary tool response: {error}"))?;
    let content_type = value
        .get("content_type")
        .and_then(Value::as_str)
        .unwrap_or("application/octet-stream");
    let filename = if content_type == "application/pdf" {
        "document.pdf"
    } else {
        "output.bin"
    };
    let output_dir = arguments
        .as_object()
        .and_then(|arguments| arguments.values().find_map(Value::as_str))
        .filter(|path| path.starts_with("/home/user/output/"))
        .and_then(|path| std::path::Path::new(path).parent())
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|| format!("/home/user/output/{function_name}"));
    let output_path = format!("{output_dir}/{filename}");
    fs.write_file(std::path::Path::new(&output_path), &bytes)
        .await
        .map_err(|error| format!("failed to persist binary tool response: {error}"))?;
    Ok(Some(output_path))
}

fn operation_example(operation: &RuntimeOperation) -> String {
    if let Some(parameter) = file_path_parameter_name(&operation.parameters) {
        return format!(
            "print({}(**{{'{}': '/home/user/attachments/<document>'}}))  # use /home/user/output for generated files",
            operation.function_name, parameter
        );
    }

    if operation
        .parameters
        .get("properties")
        .and_then(Value::as_object)
        .is_some_and(|properties| properties.contains_key("file_paths"))
    {
        return format!(
            "print({}(**{{'file_paths': ['/home/user/output/main.typ']}}))",
            operation.function_name
        );
    }

    format!("print({}())", operation.function_name)
}

fn file_path_parameter_name(parameters: &Value) -> Option<&str> {
    parameters
        .get("properties")
        .and_then(Value::as_object)
        .and_then(|properties| properties.contains_key("file_path").then_some("file_path"))
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
        assert!(prompt.contains("read the relevant `.md` file before calling an integration"));
        assert!(prompt.contains("with run_python"));

        let web_file = catalogue
            .files
            .iter()
            .find(|file| file.path == "/home/user/functions/web-fetch.md")
            .expect("expected web catalogue file");
        let markdown = String::from_utf8(web_file.contents.clone()).unwrap();
        assert!(markdown.contains("web_open_url"));
        assert!(markdown.contains("parameters: url"));
        assert!(markdown.contains("do not use `from functions import ...`"));
    }

    #[test]
    fn function_catalogue_summarizes_integrations_without_schemas() {
        let operation = RuntimeOperation {
            function_name: "enterprise_email_api_listemails".to_string(),
            description: "List recent enterprise email messages".to_string(),
            parameters: json!({"type": "object"}),
            byte_parameters: Vec::new(),
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
    fn function_catalogue_documents_base64_file_calls() {
        let operation = RuntimeOperation {
            function_name: "document_conversion_api_extractdocument".to_string(),
            description: "Convert a document".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "file_path": {"type": "string"}
                }
            }),
            byte_parameters: vec![FileParameterMapping {
                api_parameter: "files".to_string(),
                path_parameter: "file_path".to_string(),
                multiple: false,
            }],
            executor: OperationExecutor::OpenUrl,
        };
        let registry = RuntimeFunctionRegistry::with_builtin_functions(
            vec![IntegrationInfo {
                name: "Document Conversion API".to_string(),
                slug: "document_conversion_api".to_string(),
                operations: vec![operation],
            }],
            HashMap::new(),
        );

        let file = registry
            .function_catalogue()
            .files
            .into_iter()
            .find(|file| file.path.ends_with("document_conversion_api.md"))
            .expect("expected document conversion catalogue");
        let markdown = String::from_utf8(file.contents).unwrap();
        assert!(markdown.contains("document_conversion_api_extractdocument"));
        assert!(markdown.contains("file_path': '/home/user/attachments/<document>'"));
    }

    #[test]
    fn byte_parameters_are_exposed_as_vfs_paths() {
        let (parameters, mappings) = expose_file_parameters(json!({
            "type": "object",
            "properties": {
                "files": {"type": "string", "format": "byte"},
                "mode": {"type": "string"}
            },
            "required": ["files"]
        }));

        assert_eq!(
            mappings,
            vec![FileParameterMapping {
                api_parameter: "files".to_string(),
                path_parameter: "file_path".to_string(),
                multiple: false,
            }]
        );
        assert!(parameters["properties"].get("files").is_none());
        assert!(parameters["properties"].get("file_path").is_some());
        assert_eq!(parameters["required"], json!(["file_path"]));
    }

    #[test]
    fn binary_parameters_are_exposed_as_vfs_paths() {
        let (parameters, mappings) = expose_file_parameters(json!({
            "type": "object",
            "properties": {
                "file": {"type": "string", "format": "binary"}
            },
            "required": ["file"]
        }));

        assert_eq!(
            mappings,
            vec![FileParameterMapping {
                api_parameter: "file".to_string(),
                path_parameter: "file_path".to_string(),
                multiple: false,
            }]
        );
        assert!(parameters["properties"].get("file").is_none());
        assert!(parameters["properties"].get("file_path").is_some());
        assert_eq!(parameters["required"], json!(["file_path"]));
    }

    #[test]
    fn array_binary_parameters_are_exposed_as_vfs_paths() {
        let (parameters, mappings) = expose_file_parameters(json!({
            "type": "object",
            "properties": {
                "files": {
                    "type": "array",
                    "items": {"type": "string", "format": "binary"}
                }
            },
            "required": ["files"]
        }));

        assert_eq!(
            mappings,
            vec![FileParameterMapping {
                api_parameter: "files".to_string(),
                path_parameter: "file_paths".to_string(),
                multiple: true,
            }]
        );
        assert!(parameters["properties"].get("files").is_none());
        assert_eq!(
            parameters["properties"]["file_paths"]["items"]["type"],
            "string"
        );
        assert_eq!(parameters["required"], json!(["file_paths"]));
    }

    #[test]
    fn function_catalogue_documents_multiple_file_paths() {
        let operation = RuntimeOperation {
            function_name: "typst_compiledocument".to_string(),
            description: "Compile a Typst document".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "file_paths": {
                        "type": "array",
                        "items": {"type": "string"}
                    }
                },
                "required": ["file_paths"]
            }),
            byte_parameters: vec![FileParameterMapping {
                api_parameter: "files".to_string(),
                path_parameter: "file_paths".to_string(),
                multiple: true,
            }],
            executor: OperationExecutor::OpenUrl,
        };
        let registry = RuntimeFunctionRegistry::with_builtin_functions(
            vec![IntegrationInfo {
                name: "Typst Compilation API".to_string(),
                slug: "typst".to_string(),
                operations: vec![operation],
            }],
            HashMap::new(),
        );

        let markdown = String::from_utf8(
            registry
                .function_catalogue()
                .files
                .into_iter()
                .find(|file| file.path.ends_with("typst.md"))
                .unwrap()
                .contents,
        )
        .unwrap();
        assert!(markdown.contains("file_paths': ['/home/user/output/main.typ']"));
        assert!(markdown.contains("parameters: file_paths"));
    }

    #[test]
    fn file_backed_functions_accept_only_attachment_and_output_paths() {
        assert!(is_allowed_file_path("/home/user/attachments/main.typ"));
        assert!(is_allowed_file_path("/home/user/output/draft/main.typ"));
        assert!(!is_allowed_file_path("/home/user/skills/main.typ"));
        assert!(!is_allowed_file_path("/tmp/main.typ"));
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
