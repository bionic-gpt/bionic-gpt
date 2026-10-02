use crate::builtin_tools::openapi_tool_adapter::{
    HttpClient, HttpRequestBody, HttpResponse as ToolHttpResponse,
};
use crate::{BionicOpenAPI, ToolDyn, ToolError};
use async_trait::async_trait;
use base64::Engine;
use bashkit::FileSystem;
use reqwest::{Method, StatusCode, Url};
use sandbox::{
    HttpHeader, HttpMethod, HttpRequest, PythonCallArguments, PythonFunction, PythonFunctionError,
    SandboxNetwork,
};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RESPONSE_BYTES: u64 = 20 * 1024 * 1024;

#[derive(Clone)]
pub(crate) struct ConnectorDocument {
    pub name: String,
    pub slug: String,
    pub document: Value,
    pub is_builtin: bool,
}

#[derive(Clone)]
pub(crate) struct ConnectorOperation {
    pub function_name: String,
    pub description: String,
    pub parameters: Value,
    pub operation_id: String,
}

#[derive(Clone)]
pub(crate) struct ConnectorInfo {
    pub name: String,
    pub slug: String,
    pub is_builtin: bool,
    pub operations: Vec<ConnectorOperation>,
}

pub(crate) fn catalogue(documents: &[ConnectorDocument]) -> Vec<ConnectorInfo> {
    let mut documents = documents.to_vec();
    documents.sort_by(|left, right| left.slug.cmp(&right.slug));
    let mut used = HashSet::new();
    documents
        .into_iter()
        .filter_map(|document| {
            let openapi = BionicOpenAPI::new(&document.document).ok()?;
            let mut definitions = openapi.create_tool_definitions().tool_definitions;
            definitions.sort_by(|left, right| left.name.cmp(&right.name));
            let support = supported_operations(&document.document);
            let operations = definitions
                .into_iter()
                .filter(|definition| match support.get(&definition.name) {
                    Some(None) => true,
                    Some(Some(reason)) => {
                        tracing::warn!(
                            connector = %document.slug,
                            operation = %definition.name,
                            %reason,
                            "connector operation is not available as a Python function"
                        );
                        false
                    }
                    None => false,
                })
                .map(|definition| {
                    let function_name =
                        operation_function_name(&document.slug, &definition.name, &mut used);
                    let (parameters, _) = expose_file_parameters(definition.parameters);
                    ConnectorOperation {
                        function_name,
                        description: definition.description,
                        parameters,
                        operation_id: definition.name,
                    }
                })
                .collect();
            Some(ConnectorInfo {
                name: document.name,
                slug: document.slug,
                is_builtin: document.is_builtin,
                operations,
            })
        })
        .collect()
}

fn supported_operations(document: &Value) -> HashMap<String, Option<String>> {
    let mut operations = HashMap::new();
    let Some(paths) = document.get("paths").and_then(Value::as_object) else {
        return operations;
    };
    for path_item in paths.values().filter_map(Value::as_object) {
        let path_parameters = path_item
            .get("parameters")
            .and_then(Value::as_array)
            .filter(|parameters| !parameters.is_empty());
        for method in ["get", "post", "put", "delete", "head", "patch"] {
            let Some(operation) = path_item.get(method).and_then(Value::as_object) else {
                continue;
            };
            let Some(operation_id) = operation.get("operationId").and_then(Value::as_str) else {
                continue;
            };
            let reason = if path_parameters.is_some() {
                Some("path-level parameters are not supported".to_string())
            } else if operation
                .get("parameters")
                .and_then(Value::as_array)
                .is_some_and(|parameters| {
                    parameters.iter().any(|parameter| {
                        parameter.get("$ref").is_some()
                            || matches!(
                                parameter.get("in").and_then(Value::as_str),
                                Some("header" | "cookie")
                            )
                    })
                })
            {
                Some("referenced, header, and cookie parameters are not supported".to_string())
            } else {
                operation
                    .get("requestBody")
                    .and_then(|body| body.get("content"))
                    .and_then(Value::as_object)
                    .filter(|content| {
                        !content.contains_key("application/json")
                            && !content.contains_key("multipart/form-data")
                    })
                    .map(|_| "request body content type is not supported".to_string())
            };
            operations.insert(operation_id.to_string(), reason);
        }
    }
    operations
}

pub(crate) fn skill_markdown(connector: &ConnectorInfo) -> String {
    let mut markdown = format!(
        "# {}\n\nUse this connector from `run_python`. Its functions are supplied by Bionic at runtime; do not import a module or add credentials. Functions accept keyword arguments or one positional dictionary.\n\nRead an operation file under `/home/user/skills/{}/operations/` when you need its exact arguments.\n\n## Functions\n\n",
        connector.name, connector.slug
    );
    for operation in &connector.operations {
        let summary = compact_summary(&operation.description, 180);
        markdown.push_str(&format!(
            "- `{}` — {} ([details](operations/{}.md))\n",
            operation.function_name, summary, operation.function_name
        ));
    }
    markdown
}

fn compact_summary(value: &str, max_characters: usize) -> String {
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if value.chars().count() <= max_characters {
        return value;
    }
    let mut summary = value.chars().take(max_characters).collect::<String>();
    if let Some(last_space) = summary.rfind(' ') {
        summary.truncate(last_space);
    }
    summary.push('…');
    summary
}

pub(crate) fn operation_markdown(
    connector: &ConnectorInfo,
    operation: &ConnectorOperation,
) -> String {
    let hints = parameter_hints(&operation.parameters);
    let mut markdown = format!(
        "# {} — `{}`\n\n{}\n\nOpenAPI operation: `{}`\n\n## Arguments\n\n",
        connector.name, operation.function_name, operation.description, operation.operation_id
    );
    if hints.is_empty() {
        markdown.push_str("This function takes no arguments.\n");
    } else {
        for hint in hints {
            markdown.push_str(&format!("- `{hint}`\n"));
        }
    }
    markdown.push_str(&format!(
        "\n## Example\n\n```python\nresult = {}({})\nprint(result)\n```\n\n## Parameter schema\n\n```json\n{}\n```\n",
        operation.function_name,
        example_arguments(&operation.parameters),
        serde_json::to_string_pretty(&operation.parameters).unwrap_or_else(|_| "{}".into())
    ));
    markdown
}

pub(crate) fn build_python_functions(
    documents: &[ConnectorDocument],
    network: Arc<dyn SandboxNetwork>,
    filesystem: Arc<dyn FileSystem>,
) -> Vec<Arc<dyn PythonFunction>> {
    let catalogue = catalogue(documents);
    let client: Arc<dyn HttpClient> = Arc::new(MediatedHttpClient { network });
    let mut functions = Vec::new();
    for (document, connector) in documents_for_catalogue(documents, &catalogue) {
        let Ok(openapi) = BionicOpenAPI::new(&document.document) else {
            continue;
        };
        let tools = openapi.create_tools_with_http_client(
            &format!("https://{}.connectors.invalid", connector.slug),
            client.clone(),
        );
        let mut tools = tools
            .into_iter()
            .map(|tool| (tool.name(), tool))
            .collect::<Vec<_>>();
        tools.sort_by(|left, right| left.0.cmp(&right.0));
        for operation in &connector.operations {
            let Some((_, tool)) = tools
                .iter()
                .find(|(operation_id, _)| operation_id == &operation.operation_id)
            else {
                continue;
            };
            let (_, file_parameters) = expose_file_parameters(tool.parameters());
            functions.push(Arc::new(ConnectorPythonFunction {
                name: operation.function_name.clone(),
                tool: tool.clone(),
                file_parameters,
                filesystem: filesystem.clone(),
                output_sequence: AtomicUsize::new(0),
            }) as Arc<dyn PythonFunction>);
        }
    }
    functions.sort_by(|left, right| left.name().cmp(right.name()));
    functions
}

fn documents_for_catalogue<'a>(
    documents: &'a [ConnectorDocument],
    catalogue: &'a [ConnectorInfo],
) -> Vec<(&'a ConnectorDocument, &'a ConnectorInfo)> {
    catalogue
        .iter()
        .filter_map(|connector| {
            documents
                .iter()
                .find(|document| document.slug == connector.slug)
                .map(|document| (document, connector))
        })
        .collect()
}

struct ConnectorPythonFunction {
    name: String,
    tool: Arc<dyn ToolDyn>,
    file_parameters: Vec<FileParameterMapping>,
    filesystem: Arc<dyn FileSystem>,
    output_sequence: AtomicUsize,
}

#[async_trait]
impl PythonFunction for ConnectorPythonFunction {
    fn name(&self) -> &str {
        &self.name
    }

    async fn call(&self, arguments: PythonCallArguments) -> Result<Value, PythonFunctionError> {
        let mut arguments = merge_arguments(arguments)?;
        materialize_file_parameters(
            &mut arguments,
            &self.file_parameters,
            self.filesystem.as_ref(),
        )
        .await?;
        let result = self
            .tool
            .call(arguments.to_string())
            .await
            .map_err(|error| PythonFunctionError(tool_error(error)))?;
        let value: Value = serde_json::from_str(&result).unwrap_or(Value::String(result));
        let sequence = self.output_sequence.fetch_add(1, Ordering::Relaxed);
        persist_binary_response(&self.name, sequence, value, self.filesystem.as_ref()).await
    }
}

fn merge_arguments(arguments: PythonCallArguments) -> Result<Value, PythonFunctionError> {
    if arguments.positional.len() > 1 {
        return Err(PythonFunctionError(
            "connector functions accept at most one positional dictionary".into(),
        ));
    }
    let mut object = match arguments.positional.into_iter().next() {
        Some(Value::Object(object)) => object,
        Some(_) => {
            return Err(PythonFunctionError(
                "the positional connector argument must be a dictionary".into(),
            ))
        }
        None => Map::new(),
    };
    object.extend(arguments.keyword);
    Ok(Value::Object(object))
}

#[derive(Clone)]
struct FileParameterMapping {
    api_parameter: String,
    path_parameter: String,
    multiple: bool,
    required: bool,
}

fn expose_file_parameters(mut parameters: Value) -> (Value, Vec<FileParameterMapping>) {
    let required_parameters = parameters
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect::<HashSet<_>>();
    let Some(properties) = parameters
        .get_mut("properties")
        .and_then(Value::as_object_mut)
    else {
        return (parameters, Vec::new());
    };
    let file_parameters = properties
        .iter()
        .filter_map(|(name, schema)| {
            let multiple = schema.get("type").and_then(Value::as_str) == Some("array");
            let format = if multiple {
                schema
                    .get("items")
                    .and_then(|items| items.get("format"))
                    .and_then(Value::as_str)
            } else {
                schema.get("format").and_then(Value::as_str)
            };
            matches!(format, Some("byte" | "binary")).then_some((name.clone(), multiple))
        })
        .collect::<Vec<_>>();
    let mut mappings = Vec::new();
    for (index, (api_parameter, multiple)) in file_parameters.into_iter().enumerate() {
        let path_parameter = match (index, multiple) {
            (0, false) => "file_path".to_string(),
            (0, true) => "file_paths".to_string(),
            (_, false) => format!("{api_parameter}_file_path"),
            (_, true) => format!("{api_parameter}_file_paths"),
        };
        properties.remove(&api_parameter);
        properties.insert(
            path_parameter.clone(),
            if multiple {
                json!({"type":"array","items":{"type":"string"},"description":"VFS paths under /home/user/attachments, /home/user/work, or /home/user/output."})
            } else {
                json!({"type":"string","description":"A VFS path under /home/user/attachments, /home/user/work, or /home/user/output."})
            },
        );
        mappings.push(FileParameterMapping {
            required: required_parameters.contains(&api_parameter),
            api_parameter,
            path_parameter,
            multiple,
        });
    }
    if let Some(required) = parameters.get_mut("required").and_then(Value::as_array_mut) {
        for required_name in required {
            if let Some(mapping) = mappings
                .iter()
                .find(|mapping| required_name.as_str() == Some(&mapping.api_parameter))
            {
                *required_name = Value::String(mapping.path_parameter.clone());
            }
        }
    }
    (parameters, mappings)
}

async fn materialize_file_parameters(
    arguments: &mut Value,
    mappings: &[FileParameterMapping],
    filesystem: &dyn FileSystem,
) -> Result<(), PythonFunctionError> {
    for mapping in mappings {
        let Some(raw_value) = arguments.get(&mapping.path_parameter) else {
            if mapping.required {
                return Err(PythonFunctionError(format!(
                    "{} is required",
                    mapping.path_parameter
                )));
            }
            continue;
        };
        let values = if mapping.multiple {
            raw_value
                .as_array()
                .ok_or_else(|| {
                    PythonFunctionError(format!(
                        "{} must be a list of VFS paths",
                        mapping.path_parameter
                    ))
                })?
                .iter()
                .map(|value| {
                    value.as_str().map(str::to_string).ok_or_else(|| {
                        PythonFunctionError(format!(
                            "{} entries must be strings",
                            mapping.path_parameter
                        ))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            vec![raw_value
                .as_str()
                .ok_or_else(|| {
                    PythonFunctionError(format!("{} must be a VFS path", mapping.path_parameter))
                })?
                .to_string()]
        };
        let mut files = Vec::new();
        for path in values {
            if !allowed_file_path(&path) {
                return Err(PythonFunctionError(format!(
                    "{path} must be under /home/user/attachments, /home/user/work, or /home/user/output"
                )));
            }
            let bytes = filesystem
                .read_file(Path::new(&path))
                .await
                .map_err(|error| PythonFunctionError(format!("failed to read {path}: {error}")))?;
            let filename = Path::new(&path)
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| PythonFunctionError(format!("invalid file path: {path}")))?;
            files.push(json!({
                "__bionic_file": true,
                "filename": filename,
                "content_base64": base64::engine::general_purpose::STANDARD.encode(bytes),
            }));
        }
        arguments[&mapping.api_parameter] = if mapping.multiple {
            Value::Array(files)
        } else {
            files.into_iter().next().unwrap_or(Value::Null)
        };
        arguments
            .as_object_mut()
            .expect("connector arguments are an object")
            .remove(&mapping.path_parameter);
    }
    Ok(())
}

fn allowed_file_path(path: &str) -> bool {
    [
        "/home/user/attachments/",
        "/home/user/work/",
        "/home/user/output/",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
}

async fn persist_binary_response(
    function_name: &str,
    sequence: usize,
    mut value: Value,
    filesystem: &dyn FileSystem,
) -> Result<Value, PythonFunctionError> {
    if value.get("__bionic_binary") != Some(&Value::Bool(true)) {
        return Ok(value);
    }
    let encoded = value
        .get("content_base64")
        .and_then(Value::as_str)
        .ok_or_else(|| PythonFunctionError("binary response is missing content".into()))?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|error| PythonFunctionError(format!("invalid binary response: {error}")))?;
    let content_type = value
        .get("content_type")
        .and_then(Value::as_str)
        .unwrap_or("application/octet-stream");
    let extension = if content_type == "application/pdf" {
        "pdf"
    } else {
        "bin"
    };
    let directory = format!("/home/user/output/{function_name}");
    let filename = if sequence == 0 {
        format!("output.{extension}")
    } else {
        format!("output-{}.{extension}", sequence + 1)
    };
    let path = format!("{directory}/{filename}");
    filesystem
        .mkdir(Path::new(&directory), true)
        .await
        .map_err(|error| PythonFunctionError(error.to_string()))?;
    filesystem
        .write_file(Path::new(&path), &bytes)
        .await
        .map_err(|error| PythonFunctionError(error.to_string()))?;
    if let Some(object) = value.as_object_mut() {
        object.remove("content_base64");
        object.insert("path".into(), Value::String(path));
        object.insert("size".into(), json!(bytes.len()));
    }
    Ok(value)
}

fn tool_error(error: ToolError) -> String {
    match error {
        ToolError::JsonError(error) => error.to_string(),
        ToolError::ToolCallError(error) => error.to_string(),
    }
}

struct MediatedHttpClient {
    network: Arc<dyn SandboxNetwork>,
}

#[async_trait]
impl HttpClient for MediatedHttpClient {
    async fn send(
        &self,
        method: Method,
        url: Url,
        headers: Vec<(String, String)>,
        body: Option<HttpRequestBody>,
    ) -> Result<ToolHttpResponse, String> {
        let method = method_to_sandbox(method)?;
        let mut headers = headers
            .into_iter()
            .map(|(name, value)| HttpHeader { name, value })
            .collect::<Vec<_>>();
        let body = match body {
            Some(HttpRequestBody::Json(value)) => {
                headers.push(HttpHeader {
                    name: "Content-Type".into(),
                    value: "application/json".into(),
                });
                Some(serde_json::to_vec(&value).map_err(|error| error.to_string())?)
            }
            Some(HttpRequestBody::Multipart { fields, files }) => {
                let boundary = multipart_boundary(&fields, &files);
                headers.push(HttpHeader {
                    name: "Content-Type".into(),
                    value: format!("multipart/form-data; boundary={boundary}"),
                });
                Some(encode_multipart(&boundary, fields, files))
            }
            None => None,
        };
        let response = self
            .network
            .request(HttpRequest {
                method,
                url: url.to_string(),
                headers,
                body,
                timeout: REQUEST_TIMEOUT,
                connect_timeout: Some(Duration::from_secs(10)),
                max_response_bytes: MAX_RESPONSE_BYTES,
            })
            .await
            .map_err(|error| error.to_string())?;
        let content_type = response
            .headers
            .iter()
            .find(|header| header.name.eq_ignore_ascii_case("content-type"))
            .map(|header| header.value.clone());
        Ok(ToolHttpResponse {
            status: StatusCode::from_u16(response.status).map_err(|error| error.to_string())?,
            body: response.body,
            content_type,
        })
    }
}

fn method_to_sandbox(method: Method) -> Result<HttpMethod, String> {
    match method {
        Method::GET => Ok(HttpMethod::Get),
        Method::POST => Ok(HttpMethod::Post),
        Method::PUT => Ok(HttpMethod::Put),
        Method::DELETE => Ok(HttpMethod::Delete),
        Method::HEAD => Ok(HttpMethod::Head),
        Method::PATCH => Ok(HttpMethod::Patch),
        _ => Err(format!("unsupported connector HTTP method: {method}")),
    }
}

fn encode_multipart(
    boundary: &str,
    fields: Vec<(String, String)>,
    files: Vec<(String, String, Vec<u8>)>,
) -> Vec<u8> {
    let mut body = Vec::new();
    for (name, value) in fields {
        let name = multipart_quoted(&name);
        body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n").as_bytes());
    }
    for (name, filename, bytes) in files {
        let name = multipart_quoted(&name);
        let filename = multipart_quoted(&filename);
        body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"; filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n").as_bytes());
        body.extend_from_slice(&bytes);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    body
}

fn multipart_boundary(fields: &[(String, String)], files: &[(String, String, Vec<u8>)]) -> String {
    let mut contents = Vec::new();
    for (name, value) in fields {
        contents.extend_from_slice(name.as_bytes());
        contents.extend_from_slice(value.as_bytes());
    }
    for (name, filename, bytes) in files {
        contents.extend_from_slice(name.as_bytes());
        contents.extend_from_slice(filename.as_bytes());
        contents.extend_from_slice(bytes);
    }
    let digest = md5::compute(contents);
    format!("bionic-{digest:x}")
}

fn multipart_quoted(value: &str) -> String {
    value
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace('"', "%22")
}

fn operation_function_name(slug: &str, operation_id: &str, used: &mut HashSet<String>) -> String {
    let prefix = sanitize_identifier(slug);
    let operation = sanitize_identifier(operation_id);
    let operation = operation
        .strip_prefix(&format!("{prefix}_"))
        .unwrap_or(&operation);
    unique_identifier(&format!("{prefix}_{operation}"), used)
}

fn unique_identifier(value: &str, used: &mut HashSet<String>) -> String {
    let base = sanitize_identifier(value);
    let mut candidate = base.clone();
    let mut suffix = 2;
    while !used.insert(candidate.clone()) {
        candidate = format!("{base}_{suffix}");
        suffix += 1;
    }
    candidate
}

fn sanitize_identifier(value: &str) -> String {
    let mut output = String::new();
    let characters = value.chars().collect::<Vec<_>>();
    for (index, character) in characters.iter().copied().enumerate() {
        if character.is_ascii_alphanumeric() || character == '_' {
            if character.is_ascii_uppercase()
                && !output.ends_with('_')
                && index > 0
                && (characters[index - 1].is_ascii_lowercase()
                    || characters
                        .get(index + 1)
                        .is_some_and(|next| next.is_ascii_lowercase()))
            {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
        } else if !output.ends_with('_') {
            output.push('_');
        }
    }
    let mut output = output.trim_matches('_').to_string();
    if output.is_empty() {
        output = "function".into();
    }
    if output.starts_with(|character: char| character.is_ascii_digit())
        || is_python_keyword(&output)
    {
        output.insert(0, '_');
    }
    output
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

fn parameter_hints(parameters: &Value) -> Vec<String> {
    let required = parameters
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect::<HashSet<_>>();
    parameters
        .get("properties")
        .and_then(Value::as_object)
        .map(|properties| {
            properties
                .iter()
                .map(|(name, schema)| {
                    let presence = if required.contains(name.as_str()) {
                        "required"
                    } else {
                        "optional"
                    };
                    format!("{name}: {} ({presence})", schema_type_hint(schema))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn schema_type_hint(schema: &Value) -> String {
    match schema.get("type").and_then(Value::as_str) {
        Some("array") => format!(
            "list[{}]",
            schema
                .get("items")
                .map(schema_type_hint)
                .unwrap_or_else(|| "value".into())
        ),
        Some(value) => value.to_string(),
        None if schema.get("properties").is_some() || schema.get("$ref").is_some() => {
            "object".into()
        }
        None => "value".into(),
    }
}

fn example_arguments(parameters: &Value) -> String {
    let required = parameters
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>();
    if required.iter().all(|name| is_python_identifier(name)) {
        required
            .into_iter()
            .map(|name| format!("{name}=..."))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        format!(
            "{{{}}}",
            required
                .into_iter()
                .map(|name| format!("{name:?}: ..."))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn is_python_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
        && !is_python_keyword(&value.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bashkit::InMemoryFs;
    use std::sync::Mutex;

    #[derive(Default)]
    struct RecordingNetwork(Mutex<Vec<HttpRequest>>);

    #[async_trait]
    impl SandboxNetwork for RecordingNetwork {
        async fn request(
            &self,
            request: HttpRequest,
        ) -> Result<sandbox::HttpResponse, sandbox::NetworkError> {
            self.0.lock().unwrap().push(request);
            Ok(sandbox::HttpResponse {
                status: 200,
                headers: vec![HttpHeader {
                    name: "Content-Type".into(),
                    value: "application/json".into(),
                }],
                body: br#"{"records":[{"Id":"001"}]}"#.to_vec(),
            })
        }
    }

    #[test]
    fn operation_names_strip_duplicate_connector_prefix() {
        let mut used = HashSet::new();
        assert_eq!(
            operation_function_name("salesforce", "salesforce.sobjects.lead.create", &mut used),
            "salesforce_sobjects_lead_create"
        );
        assert_eq!(
            operation_function_name("salesforce", "salesforce.query", &mut used),
            "salesforce_query"
        );
        assert_eq!(
            operation_function_name("typst", "compileDocument", &mut used),
            "typst_compile_document"
        );
    }

    #[test]
    fn file_parameters_are_exposed_as_vfs_paths() {
        let (parameters, mappings) = expose_file_parameters(json!({
            "type":"object",
            "properties":{"file":{"type":"string","format":"binary"}},
            "required":["file"]
        }));
        assert_eq!(parameters["required"], json!(["file_path"]));
        assert!(parameters["properties"].get("file").is_none());
        assert_eq!(mappings[0].path_parameter, "file_path");
        assert!(mappings[0].required);
    }

    #[tokio::test]
    async fn optional_file_parameters_may_be_omitted() {
        let (_, mappings) = expose_file_parameters(json!({
            "type":"object",
            "properties":{"file":{"type":"string","format":"binary"}}
        }));
        let mut arguments = json!({});
        materialize_file_parameters(&mut arguments, &mappings, &InMemoryFs::new())
            .await
            .unwrap();
        assert_eq!(arguments, json!({}));
    }

    #[test]
    fn examples_use_a_dictionary_for_non_identifier_parameters() {
        assert_eq!(
            example_arguments(&json!({"required":["filter[name]"]})),
            r#"{"filter[name]": ...}"#
        );
    }

    #[tokio::test]
    async fn generated_function_uses_the_mediated_virtual_origin() {
        let document = salesforce_document();
        let network = Arc::new(RecordingNetwork::default());
        let functions =
            build_python_functions(&[document], network.clone(), Arc::new(InMemoryFs::new()));
        let function = functions
            .iter()
            .find(|function| function.name() == "salesforce_query")
            .unwrap();
        let result = function
            .call(PythonCallArguments {
                positional: Vec::new(),
                keyword: Map::from_iter([("q".into(), Value::String("SELECT Id".into()))]),
            })
            .await
            .unwrap();
        assert_eq!(result["records"][0]["Id"], "001");
        let requests = network.0.lock().unwrap();
        assert_eq!(
            requests[0].url,
            "https://salesforce.connectors.invalid/query?q=SELECT+Id"
        );
    }

    #[test]
    fn generated_skill_links_to_operation_docs_without_exposing_openapi() {
        let connectors = catalogue(&[salesforce_document()]);
        let skill = skill_markdown(&connectors[0]);
        assert!(skill.contains("`salesforce_query`"));
        assert!(skill.contains("operations/salesforce_query.md"));
        assert!(!skill.contains("openapi.json"));
    }

    #[test]
    fn unsupported_header_parameter_operations_are_not_exposed() {
        let mut document = salesforce_document();
        document.document["paths"]["/query"]["get"]["parameters"] = json!([{
            "name":"X-Custom",
            "in":"header",
            "schema":{"type":"string"}
        }]);
        assert!(catalogue(&[document])[0].operations.is_empty());
    }

    fn salesforce_document() -> ConnectorDocument {
        ConnectorDocument {
            name: "Salesforce".into(),
            slug: "salesforce".into(),
            is_builtin: false,
            document: json!({
                "openapi":"3.0.0",
                "info":{"title":"Salesforce","version":"1"},
                "servers":[{"url":"https://upstream.example"}],
                "paths":{"/query":{"get":{
                    "operationId":"salesforce.query",
                    "parameters":[{"name":"q","in":"query","required":true,"schema":{"type":"string"}}],
                    "responses":{"200":{"description":"ok"}}
                }}}
            }),
        }
    }
}
