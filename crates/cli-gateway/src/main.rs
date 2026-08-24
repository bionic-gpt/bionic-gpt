use axum::{
    body::{to_bytes, Body},
    http::{header, Method, Request, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use futures_util::stream;
use multer::Multipart;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    convert::Infallible,
    env,
    net::SocketAddr,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Duration,
};
use tempfile::TempDir;
use tokio::{fs, process::Command, time::timeout};
use tracing::{debug, error, info};

const DEFAULT_SPEC: &str = "/etc/cli-gateway/openapi.yaml";
const DEFAULT_BIND: &str = "0.0.0.0:8080";
const MAX_REQUEST_BYTES: usize = 128 * 1024 * 1024;
const DEFAULT_TIMEOUT_MS: u64 = 30_000;

#[derive(Clone)]
struct AppState {
    spec: Arc<LoadedSpec>,
}

#[derive(Clone, Debug)]
struct Operation {
    operation_id: String,
    executable: String,
    args: Vec<String>,
    output: Option<PathBuf>,
    timeout: Duration,
    response_content_type: Option<String>,
}

#[derive(Debug)]
struct LoadedSpec {
    yaml: String,
    json: String,
    operations: HashMap<(Method, String), Operation>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let spec_path = env::var("CLI_GATEWAY_SPEC").unwrap_or_else(|_| DEFAULT_SPEC.to_string());
    let spec = LoadedSpec::load(&spec_path).await?;
    let state = AppState {
        spec: Arc::new(spec),
    };
    let address: SocketAddr = env::var("CLI_GATEWAY_BIND")
        .unwrap_or_else(|_| DEFAULT_BIND.to_string())
        .parse()?;

    let app = app_router(state);

    info!(%address, %spec_path, "cli gateway listening");
    axum::serve(tokio::net::TcpListener::bind(address).await?, app).await?;
    Ok(())
}

fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/openapi.yaml", get(openapi_yaml))
        .route("/openapi.json", get(openapi_json))
        .fallback(operation)
        .with_state(state)
}

impl LoadedSpec {
    async fn load(path: &str) -> Result<Self, String> {
        let yaml = fs::read_to_string(path)
            .await
            .map_err(|error| format!("failed to read OpenAPI spec {path}: {error}"))?;
        Self::from_yaml(&yaml)
    }

    fn from_yaml(yaml: &str) -> Result<Self, String> {
        let value: serde_yaml::Value =
            serde_yaml::from_str(yaml).map_err(|error| format!("invalid OpenAPI YAML: {error}"))?;
        let json_value: Value = serde_json::to_value(value)
            .map_err(|error| format!("invalid OpenAPI document: {error}"))?;
        let version = json_value
            .get("openapi")
            .and_then(Value::as_str)
            .ok_or_else(|| "OpenAPI document must contain an openapi version".to_string())?;
        if !version.starts_with("3.1") {
            return Err(format!("OpenAPI 3.1 is required, found {version}"));
        }

        let paths = json_value
            .get("paths")
            .and_then(Value::as_object)
            .ok_or_else(|| "OpenAPI document must contain paths".to_string())?;
        let mut operations = HashMap::new();
        for (path, path_item) in paths {
            if path.contains('{') || path.contains('}') {
                return Err(format!("path parameters are not supported: {path}"));
            }
            let Some(path_item) = path_item.as_object() else {
                continue;
            };
            for (method_name, operation_value) in path_item {
                let Some(method) = parse_method(method_name) else {
                    continue;
                };
                let operation = operation_value
                    .as_object()
                    .ok_or_else(|| format!("operation {method_name} {path} must be an object"))?;
                let operation_id = required_string(operation, "operationId", method_name, path)?;
                let cli = operation
                    .get("x-cli")
                    .and_then(Value::as_object)
                    .ok_or_else(|| format!("operation {operation_id} is missing x-cli"))?;
                let executable = required_string(cli, "executable", method_name, path)?;
                if !Path::new(&executable).is_absolute() {
                    return Err(format!(
                        "x-cli executable must be absolute for {operation_id}"
                    ));
                }
                let args = cli
                    .get("args")
                    .and_then(Value::as_array)
                    .ok_or_else(|| format!("operation {operation_id} is missing x-cli.args"))?
                    .iter()
                    .map(|arg| {
                        arg.as_str()
                            .map(str::to_string)
                            .ok_or_else(|| format!("x-cli args must be strings for {operation_id}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let output = cli
                    .get("output")
                    .map(|value| {
                        let output = value.as_str().ok_or_else(|| {
                            format!("x-cli.output must be a string for {operation_id}")
                        })?;
                        validate_relative_path(output)?;
                        Ok::<_, String>(PathBuf::from(output))
                    })
                    .transpose()?;
                let timeout_ms = cli
                    .get("timeout-ms")
                    .and_then(Value::as_u64)
                    .unwrap_or(DEFAULT_TIMEOUT_MS);
                if timeout_ms == 0 {
                    return Err(format!(
                        "x-cli timeout-ms must be positive for {operation_id}"
                    ));
                }
                let response_content_type = operation
                    .get("responses")
                    .and_then(Value::as_object)
                    .and_then(|responses| responses.get("200"))
                    .and_then(Value::as_object)
                    .and_then(|response| response.get("content"))
                    .and_then(Value::as_object)
                    .and_then(|content| content.keys().next().cloned());
                if operations
                    .insert(
                        (method.clone(), path.clone()),
                        Operation {
                            operation_id: operation_id.clone(),
                            executable,
                            args,
                            output,
                            timeout: Duration::from_millis(timeout_ms),
                            response_content_type,
                        },
                    )
                    .is_some()
                {
                    return Err(format!("duplicate operation {method_name} {path}"));
                }
            }
        }
        if operations.is_empty() {
            return Err("OpenAPI document declares no executable operations".to_string());
        }

        Ok(Self {
            yaml: yaml.to_string(),
            json: serde_json::to_string_pretty(&json_value).map_err(|error| error.to_string())?,
            operations,
        })
    }
}

fn required_string(
    map: &serde_json::Map<String, Value>,
    key: &str,
    method: &str,
    path: &str,
) -> Result<String, String> {
    map.get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("{key} is required for {method} {path}"))
}

fn parse_method(method: &str) -> Option<Method> {
    match method.to_ascii_lowercase().as_str() {
        "get" => Some(Method::GET),
        "post" => Some(Method::POST),
        "put" => Some(Method::PUT),
        "patch" => Some(Method::PATCH),
        "delete" => Some(Method::DELETE),
        "head" => Some(Method::HEAD),
        "options" => Some(Method::OPTIONS),
        _ => None,
    }
}

fn validate_relative_path(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(format!(
            "path must stay inside the request workspace: {}",
            path.display()
        ));
    }
    Ok(())
}

async fn health() -> Response {
    json_response(StatusCode::OK, json!({"status": "ok"}))
}

async fn openapi_yaml(axum::extract::State(state): axum::extract::State<AppState>) -> Response {
    response_with_content(
        StatusCode::OK,
        state.spec.yaml.as_bytes().to_vec(),
        "application/yaml",
    )
}

async fn openapi_json(axum::extract::State(state): axum::extract::State<AppState>) -> Response {
    response_with_content(
        StatusCode::OK,
        state.spec.json.as_bytes().to_vec(),
        "application/json",
    )
}

async fn operation(
    axum::extract::State(state): axum::extract::State<AppState>,
    request: Request<Body>,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let Some(operation) = state
        .spec
        .operations
        .get(&(method.clone(), path.clone()))
        .cloned()
    else {
        return json_response(
            StatusCode::NOT_FOUND,
            json!({"error": "route is not declared in the OpenAPI spec"}),
        );
    };

    let workspace = match TempDir::new() {
        Ok(workspace) => workspace,
        Err(error) => {
            return internal_error(format!("failed to create request workspace: {error}"))
        }
    };
    if let Err(error) = write_uploads(request, workspace.path()).await {
        return json_response(StatusCode::BAD_REQUEST, json!({"error": error}));
    }
    debug!(operation = %operation.operation_id, "executing fixed CLI operation");

    let mut command = Command::new(&operation.executable);
    command
        .args(&operation.args)
        .current_dir(workspace.path())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let output = match timeout(operation.timeout, command.output()).await {
        Ok(Ok(output)) => output,
        Ok(Err(error)) => {
            return internal_error(format!(
                "failed to execute {}: {error}",
                operation.operation_id
            ))
        }
        Err(_) => {
            return json_response(
                StatusCode::GATEWAY_TIMEOUT,
                json!({"error": "CLI execution timed out", "operation": operation.operation_id}),
            )
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() {
        return json_response(
            StatusCode::UNPROCESSABLE_ENTITY,
            json!({"error": "CLI execution failed", "operation": operation.operation_id, "exit_status": output.status.code(), "stdout": stdout, "stderr": stderr}),
        );
    }
    if let Some(output_path) = operation.output {
        let path = workspace.path().join(output_path);
        match fs::read(&path).await {
            Ok(bytes) => {
                return response_with_content(
                    StatusCode::OK,
                    bytes,
                    operation
                        .response_content_type
                        .as_deref()
                        .unwrap_or("application/octet-stream"),
                )
            }
            Err(error) => {
                return json_response(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    json!({"error": "CLI did not produce the declared output", "operation": operation.operation_id, "detail": error.to_string(), "stdout": stdout, "stderr": stderr}),
                )
            }
        }
    }
    json_response(
        StatusCode::OK,
        json!({"operation": operation.operation_id, "exit_status": output.status.code(), "stdout": stdout, "stderr": stderr}),
    )
}

async fn write_uploads(request: Request<Body>, workspace: &Path) -> Result<(), String> {
    let content_type = request
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let Some(boundary) = content_type
        .split(';')
        .find_map(|part| part.trim().strip_prefix("boundary="))
        .map(str::to_string)
    else {
        return Ok(());
    };
    let body = to_bytes(request.into_body(), MAX_REQUEST_BYTES)
        .await
        .map_err(|error| format!("failed to read request body: {error}"))?;
    let mut multipart = Multipart::new(
        stream::once(async move { Ok::<_, Infallible>(body) }),
        &boundary,
    );
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|error| format!("invalid multipart body: {error}"))?
    {
        if field.name() != Some("files") {
            return Err("only multipart fields named files are supported".to_string());
        }
        let filename = field
            .file_name()
            .ok_or_else(|| "uploaded files must have a filename".to_string())?;
        if filename.contains('/') || filename.contains('\\') || Path::new(filename).is_absolute() {
            return Err("uploaded filenames must be plain basenames".to_string());
        }
        let filename = Path::new(filename)
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty() && *name != "." && *name != "..")
            .ok_or_else(|| "invalid upload filename".to_string())?;
        let destination = workspace.join(filename);
        if destination.exists() {
            return Err(format!("duplicate uploaded filename: {filename}"));
        }
        fs::write(
            destination,
            field
                .bytes()
                .await
                .map_err(|error| format!("failed to read upload: {error}"))?,
        )
        .await
        .map_err(|error| format!("failed to write upload: {error}"))?;
    }
    Ok(())
}

fn response_with_content(status: StatusCode, body: Vec<u8>, content_type: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(body))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn json_response(status: StatusCode, value: Value) -> Response {
    response_with_content(
        status,
        serde_json::to_vec(&value)
            .unwrap_or_else(|_| b"{\"error\":\"serialization failure\"}".to_vec()),
        "application/json",
    )
}

fn internal_error(message: String) -> Response {
    error!(%message, "cli gateway request failed");
    json_response(StatusCode::INTERNAL_SERVER_ERROR, json!({"error": message}))
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,cli_gateway=debug"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use http::Request;
    use tower::ServiceExt;

    const SPEC: &str = r#"
openapi: 3.1.0
info:
  title: Test CLI
  version: '1.0'
paths:
  /run:
    post:
      operationId: runCommand
      x-cli:
        executable: /bin/cp
        args: [input.txt, output.txt]
        timeout-ms: 1000
        output: output.txt
      responses:
        '200':
          content:
            text/plain: {}
"#;

    #[test]
    fn parses_openapi_31_and_fixed_cli_configuration() {
        let spec = LoadedSpec::from_yaml(SPEC).unwrap();
        let operation = spec
            .operations
            .get(&(Method::POST, "/run".to_string()))
            .unwrap();
        assert_eq!(operation.executable, "/bin/cp");
        assert_eq!(operation.args, ["input.txt", "output.txt"]);
        assert_eq!(operation.output.as_deref(), Some(Path::new("output.txt")));
    }

    #[test]
    fn parses_the_typst_source_spec() {
        let spec = LoadedSpec::from_yaml(include_str!("../specs/typst.openapi.yaml")).unwrap();
        assert!(spec
            .operations
            .contains_key(&(Method::POST, "/compile".to_string())));
    }

    #[test]
    fn rejects_openapi_30() {
        let error = LoadedSpec::from_yaml(&SPEC.replace("3.1.0", "3.0.3")).unwrap_err();
        assert!(error.contains("OpenAPI 3.1 is required"));
    }

    #[test]
    fn rejects_unsafe_output_paths() {
        let error =
            LoadedSpec::from_yaml(&SPEC.replace("output.txt", "../output.txt")).unwrap_err();
        assert!(error.contains("request workspace"));
    }

    #[tokio::test]
    async fn exposes_health_and_openapi() {
        let app = app_router(AppState {
            spec: Arc::new(LoadedSpec::from_yaml(SPEC).unwrap()),
        });
        let response = app
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let app = app_router(AppState {
            spec: Arc::new(LoadedSpec::from_yaml(SPEC).unwrap()),
        });
        let response = app
            .oneshot(Request::get("/openapi.json").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
    }

    #[tokio::test]
    async fn executes_fixed_command_with_multipart_upload() {
        let app = app_router(AppState {
            spec: Arc::new(LoadedSpec::from_yaml(SPEC).unwrap()),
        });
        let boundary = "test-boundary";
        let body = format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"input.txt\"\r\nContent-Type: text/plain\r\n\r\nhello\r\n--{boundary}--\r\n"
        );
        let response = app
            .oneshot(
                Request::post("/run")
                    .header(
                        header::CONTENT_TYPE,
                        format!("multipart/form-data; boundary={boundary}"),
                    )
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = to_bytes(response.into_body(), MAX_REQUEST_BYTES)
            .await
            .unwrap();
        assert_eq!(bytes.as_ref(), b"hello");
    }
}
