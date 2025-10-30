use crate::CustomError;
use axum::{
    extract::Extension,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json, Router,
};
use axum_extra::routing::{RouterExt, TypedPath};
use db::Pool;
use integrations::{BionicOpenAPI, OAuth2TokenProvider, StaticTokenProvider};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::{future::Future, pin::Pin, sync::Arc};
use time::OffsetDateTime;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

#[cfg(test)]
use std::sync::{Mutex, OnceLock};

#[cfg(test)]
type MockResolver = Box<dyn Fn(&str, Uuid) -> IntegrationContext + Send + Sync>;

#[cfg(test)]
static MOCK_RESOLVER: OnceLock<Mutex<Option<MockResolver>>> = OnceLock::new();

#[cfg(test)]
static RESOLVER_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

const JSONRPC_VERSION: &str = "2.0";
const DEFAULT_PROTOCOL_VERSION: &str = "2024-05-30";
const SUPPORTED_PROTOCOL_VERSIONS: &[&str] =
    &[DEFAULT_PROTOCOL_VERSION, "2025-03-26", "2025-06-18"];
const SERVER_NAME: &str = env!("CARGO_PKG_NAME");
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
const SKIP_API_KEY_ENV_VAR: &str = "BIONIC_MCP_SKIP_API_KEY_CHECK";

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/mcp/{slug}/{connection_id}")]
pub struct JsonRpcPath {
    pub slug: String,
    pub connection_id: Uuid,
}

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new().typed_post(handle_json_rpc).layer(cors)
}

#[derive(Deserialize, Debug)]
struct JsonRpcRequest {
    #[serde(default = "default_jsonrpc", rename = "jsonrpc")]
    jsonrpc: String,
    #[serde(default)]
    id: Option<Value>,
    method: String,
    #[serde(default = "default_params")]
    params: Value,
}

#[derive(Serialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: &'static str,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION,
            id,
            result: Some(result),
            error: None,
        }
    }

    fn failure(id: Value, code: i32, message: String, data: Option<Value>) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION,
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data,
            }),
        }
    }
}

#[derive(Serialize, Debug)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[derive(Serialize)]
struct McpTool {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "inputSchema")]
    input_schema: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,
}

#[derive(Deserialize)]
struct ToolCallParams {
    name: String,
    #[serde(default = "default_arguments")]
    arguments: Value,
}

#[derive(Default, Deserialize)]
struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    protocol_version: Option<String>,
    #[serde(default, rename = "capabilities")]
    _capabilities: Value,
    #[serde(rename = "clientInfo")]
    _client_info: Option<ClientInfo>,
}

#[derive(Deserialize)]
struct ClientInfo {
    #[serde(rename = "name")]
    _name: String,
    #[serde(default, rename = "version")]
    _version: Option<String>,
}

#[cfg_attr(test, derive(Clone))]
struct IntegrationContext {
    definition: Value,
    integration_id: i32,
    #[allow(dead_code)]
    user_id: i32,
    team_id: i32,
    user_openid_sub: Option<String>,
    connection: ConnectionAuth,
}

#[cfg_attr(test, derive(Clone))]
enum ConnectionAuth {
    ApiKey {
        connection_id: i32,
        api_key: String,
    },
    OAuth2 {
        connection_id: i32,
        access_token: String,
        refresh_token: Option<String>,
        expires_at: Option<OffsetDateTime>,
    },
}

impl ConnectionAuth {
    fn internal_id(&self) -> i32 {
        match self {
            ConnectionAuth::ApiKey { connection_id, .. }
            | ConnectionAuth::OAuth2 { connection_id, .. } => *connection_id,
        }
    }
}

#[derive(Debug)]
enum ResolveError {
    NotFound { slug: String, connection_id: Uuid },
    MissingDefinition,
    MissingSecret(&'static str),
    UnsupportedConnection(String),
    Database(String),
}

impl From<db::PoolError> for ResolveError {
    fn from(err: db::PoolError) -> Self {
        ResolveError::Database(err.to_string())
    }
}

impl From<db::TokioPostgresError> for ResolveError {
    fn from(err: db::TokioPostgresError) -> Self {
        ResolveError::Database(err.to_string())
    }
}

fn default_jsonrpc() -> String {
    JSONRPC_VERSION.to_string()
}

fn default_params() -> Value {
    Value::Null
}

fn default_arguments() -> Value {
    Value::Object(Map::new())
}

fn should_validate_api_key() -> bool {
    if let Ok(value) = std::env::var(SKIP_API_KEY_ENV_VAR) {
        let normalized = value.trim().to_ascii_lowercase();
        !matches!(normalized.as_str(), "1" | "true" | "yes" | "on")
    } else {
        true
    }
}

#[cfg(test)]
type MockApiKeyStore = std::collections::HashMap<String, i32>;

#[cfg(test)]
static MOCK_API_KEYS: OnceLock<Mutex<MockApiKeyStore>> = OnceLock::new();

#[cfg(test)]
fn mock_api_key_store() -> &'static Mutex<MockApiKeyStore> {
    MOCK_API_KEYS.get_or_init(|| Mutex::new(MockApiKeyStore::new()))
}

#[cfg(test)]
fn maybe_mock_api_key_team(api_key: &str) -> Option<i32> {
    mock_api_key_store()
        .lock()
        .ok()
        .and_then(|store| store.get(api_key).copied())
}

#[cfg(test)]
fn register_mock_api_key(api_key: &str, team_id: i32) {
    if let Ok(mut store) = mock_api_key_store().lock() {
        store.insert(api_key.to_string(), team_id);
    }
}

#[cfg(test)]
fn remove_mock_api_key(api_key: &str) {
    if let Ok(mut store) = mock_api_key_store().lock() {
        store.remove(api_key);
    }
}

async fn validate_api_key(pool: &Pool, api_key_value: &str) -> Result<i32, CustomError> {
    #[cfg(test)]
    if let Some(team_id) = maybe_mock_api_key_team(api_key_value) {
        return Ok(team_id);
    }

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let api_key_record = db::queries::api_keys::find_api_key()
        .bind(&transaction, &api_key_value)
        .opt()
        .await
        .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?
        .ok_or_else(|| CustomError::Authentication("Invalid API Key".to_string()))?;

    if api_key_record.prompt_id.is_some() {
        return Err(CustomError::Authentication(
            "API key is not enabled for MCP".to_string(),
        ));
    }

    let team_id = api_key_record.team_id;

    drop(transaction);
    drop(client);

    Ok(team_id)
}

fn negotiate_protocol_version(requested: Option<&str>) -> Result<&'static str, String> {
    match requested {
        Some(version) => SUPPORTED_PROTOCOL_VERSIONS
            .iter()
            .copied()
            .find(|supported| *supported == version)
            .ok_or_else(|| format!("Unsupported protocol version: {}", version)),
        None => Ok(DEFAULT_PROTOCOL_VERSION),
    }
}

pub async fn handle_json_rpc(
    JsonRpcPath {
        slug,
        connection_id,
    }: JsonRpcPath,
    Extension(pool): Extension<Pool>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Response, CustomError> {
    tracing::debug!("{:?}", headers);

    let enforce_api_key = should_validate_api_key();

    let api_key_team_id = if enforce_api_key {
        let authorization_header = headers
            .get(AUTHORIZATION)
            .ok_or_else(|| CustomError::Authentication("You need an API key".to_string()))?;
        let authorization_value = authorization_header
            .to_str()
            .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?;
        let api_key_value = authorization_value
            .strip_prefix("Bearer ")
            .unwrap_or(authorization_value)
            .trim();

        if api_key_value.is_empty() {
            return Err(CustomError::Authentication("Invalid API Key".to_string()));
        }

        Some(validate_api_key(&pool, api_key_value).await?)
    } else {
        None
    };

    tracing::debug!("{:?}", payload);

    let id_from_payload = payload.get("id").cloned().unwrap_or(Value::Null);
    let request: JsonRpcRequest = match serde_json::from_value(payload.clone()) {
        Ok(req) => req,
        Err(err) => {
            let response = JsonRpcResponse::failure(
                id_from_payload,
                -32600,
                "Invalid JSON-RPC request".to_string(),
                Some(json!({ "details": err.to_string() })),
            );
            return Ok(json_response(response));
        }
    };

    let request_id = request.id.clone().unwrap_or(Value::Null);

    if request.jsonrpc != JSONRPC_VERSION {
        let response = JsonRpcResponse::failure(
            request_id.clone(),
            -32600,
            format!("Unsupported JSON-RPC version: {}", request.jsonrpc),
            None,
        );
        return Ok(json_response(response));
    }

    let context = match resolve_integration_context(&pool, &slug, connection_id).await {
        Ok(ctx) => ctx,
        Err(err) => {
            let (code, message, data, log_level) = match &err {
                ResolveError::NotFound {
                    slug: missing_slug,
                    connection_id: missing_id,
                } => (
                    -32004,
                    format!(
                        "Unknown MCP connection {} for integration slug {}",
                        missing_id, missing_slug
                    ),
                    Some(json!({
                        "slug": missing_slug,
                        "connectionId": missing_id,
                    })),
                    tracing::Level::DEBUG,
                ),
                ResolveError::MissingDefinition => (
                    -32603,
                    "Integration definition is missing".to_string(),
                    None,
                    tracing::Level::ERROR,
                ),
                ResolveError::MissingSecret(secret) => (
                    -32603,
                    format!("Missing {} for connection", secret),
                    None,
                    tracing::Level::ERROR,
                ),
                ResolveError::UnsupportedConnection(kind) => (
                    -32601,
                    format!("Unsupported connection type: {}", kind),
                    None,
                    tracing::Level::ERROR,
                ),
                ResolveError::Database(message) => (
                    -32603,
                    "Database error while loading connection".to_string(),
                    Some(json!({ "details": message })),
                    tracing::Level::ERROR,
                ),
            };

            match log_level {
                tracing::Level::ERROR => tracing::error!(?err, "Failed to resolve MCP context"),
                tracing::Level::WARN => tracing::warn!(?err, "Failed to resolve MCP context"),
                tracing::Level::INFO => tracing::info!(?err, "Failed to resolve MCP context"),
                tracing::Level::DEBUG => tracing::debug!(?err, "Failed to resolve MCP context"),
                tracing::Level::TRACE => tracing::trace!(?err, "Failed to resolve MCP context"),
            }

            let response = JsonRpcResponse::failure(request_id.clone(), code, message, data);
            return Ok(json_response(response));
        }
    };

    if let Some(team_id) = api_key_team_id {
        if context.team_id != team_id {
            return Err(CustomError::Authentication(
                "API key is not authorized for this connection".to_string(),
            ));
        }
    }

    let integration_openapi = match BionicOpenAPI::new(&context.definition) {
        Ok(api) => api,
        Err(err) => {
            tracing::error!("Failed to parse integration definition: {}", err);
            let response = JsonRpcResponse::failure(
                request_id.clone(),
                -32603,
                "Failed to parse integration definition".to_string(),
                Some(json!({ "details": err.to_string() })),
            );
            return Ok(json_response(response));
        }
    };

    let Some(spec_slug) = integration_openapi.get_mcp_slug() else {
        let response = JsonRpcResponse::failure(
            request_id.clone(),
            -32601,
            "Integration is not configured for MCP".to_string(),
            None,
        );
        return Ok(json_response(response));
    };

    if !spec_slug.eq_ignore_ascii_case(&slug) {
        let response = JsonRpcResponse::failure(
            request_id.clone(),
            -32602,
            "Integration slug mismatch".to_string(),
            Some(json!({ "expected": spec_slug, "received": slug })),
        );
        return Ok(json_response(response));
    }

    let openapi = integration_openapi;

    let tool_definitions = openapi.create_tool_definitions();

    match request.method.as_str() {
        "initialize" => {
            let params: InitializeParams = match serde_json::from_value(request.params.clone()) {
                Ok(p) => p,
                Err(err) => {
                    let response = JsonRpcResponse::failure(
                        request_id.clone(),
                        -32602,
                        "Invalid parameters for initialize".to_string(),
                        Some(json!({ "details": err.to_string() })),
                    );
                    return Ok(json_response(response));
                }
            };

            let negotiated_protocol =
                match negotiate_protocol_version(params.protocol_version.as_deref()) {
                    Ok(version) => version,
                    Err(message) => {
                        let response =
                            JsonRpcResponse::failure(request_id.clone(), -32602, message, None);
                        return Ok(json_response(response));
                    }
                };

            let response = JsonRpcResponse::success(
                request_id.clone(),
                json!({
                    "protocolVersion": negotiated_protocol,
                    "capabilities": {
                        "tools": {
                            "listChanged": false
                        }
                    },
                    "serverInfo": {
                        "name": SERVER_NAME,
                        "version": SERVER_VERSION,
                    },
                    "metadata": {
                        "integrationId": context.integration_id,
                        "connectionId": context.connection.internal_id(),
                        "slug": slug,
                    }
                }),
            );
            Ok(json_response(response))
        }
        "session.initialize" => {
            let response = JsonRpcResponse::success(
                request_id.clone(),
                json!({
                    "protocolVersion": DEFAULT_PROTOCOL_VERSION,
                    "capabilities": {
                        "tools": {
                            "listChanged": false
                        }
                    },
                    "metadata": {
                        "integrationId": context.integration_id,
                        "connectionId": context.connection.internal_id(),
                        "slug": slug,
                    }
                }),
            );
            Ok(json_response(response))
        }
        "notifications/initialized" => {
            if request.id.is_some() {
                let response = JsonRpcResponse::failure(
                    request_id.clone(),
                    -32600,
                    "notifications/initialized must not include an id".to_string(),
                    None,
                );
                Ok(json_response(response))
            } else {
                Ok(StatusCode::ACCEPTED.into_response())
            }
        }
        "tools/list" => {
            let tools: Vec<McpTool> = tool_definitions
                .tool_definitions
                .iter()
                .map(|tool| McpTool {
                    name: tool.function.name.clone(),
                    description: if tool.function.description.trim().is_empty() {
                        None
                    } else {
                        Some(tool.function.description.clone())
                    },
                    input_schema: tool.function.parameters.clone(),
                    metadata: Some(json!({
                        "integrationId": context.integration_id,
                        "connectionId": context.connection.internal_id(),
                    })),
                })
                .collect();

            let response = JsonRpcResponse::success(request_id.clone(), json!({ "tools": tools }));
            Ok(json_response(response))
        }
        "tools/call" => {
            let params: ToolCallParams = match serde_json::from_value(request.params.clone()) {
                Ok(p) => p,
                Err(err) => {
                    let response = JsonRpcResponse::failure(
                        request_id.clone(),
                        -32602,
                        "Invalid parameters for tools/call".to_string(),
                        Some(json!({ "details": err.to_string() })),
                    );
                    return Ok(json_response(response));
                }
            };

            let token_provider = match &context.connection {
                ConnectionAuth::ApiKey { api_key, .. } => {
                    Some(Arc::new(StaticTokenProvider::new(api_key.clone())) as Arc<_>)
                }
                ConnectionAuth::OAuth2 {
                    access_token,
                    refresh_token,
                    expires_at,
                    connection_id,
                } => {
                    if let Some(config) = openapi.get_oauth2_config() {
                        if let Some(sub) = context.user_openid_sub.clone() {
                            Some(Arc::new(OAuth2TokenProvider::new(
                                pool.clone(),
                                sub,
                                *connection_id,
                                Some(access_token.clone()),
                                refresh_token.clone(),
                                *expires_at,
                                config,
                            )) as Arc<_>)
                        } else {
                            Some(Arc::new(StaticTokenProvider::new(access_token.clone())) as Arc<_>)
                        }
                    } else {
                        Some(Arc::new(StaticTokenProvider::new(access_token.clone())) as Arc<_>)
                    }
                }
            };

            let tools = match openapi.create_tools(token_provider) {
                Ok(tools) => tools,
                Err(err) => {
                    tracing::error!("Failed to create tools: {}", err);
                    let response = JsonRpcResponse::failure(
                        request_id.clone(),
                        -32603,
                        "Failed to create tool instances".to_string(),
                        Some(json!({ "details": err })),
                    );
                    return Ok(json_response(response));
                }
            };

            let Some(tool) = tools.into_iter().find(|tool| tool.name() == params.name) else {
                let response = JsonRpcResponse::failure(
                    request_id.clone(),
                    -32602,
                    format!("Unknown tool: {}", params.name),
                    None,
                );
                return Ok(json_response(response));
            };

            let argument_payload = match arguments_to_string(params.arguments) {
                Ok(payload) => payload,
                Err(err) => {
                    let response = JsonRpcResponse::failure(
                        request_id.clone(),
                        -32602,
                        "Arguments must be valid JSON".to_string(),
                        Some(json!({ "details": err.to_string() })),
                    );
                    return Ok(json_response(response));
                }
            };

            match tool.execute(&argument_payload).await {
                Ok(result) => {
                    let text = match result {
                        Value::String(text) => text,
                        other => other.to_string(),
                    };

                    let response_payload = json!({
                        "content": [
                            {
                                "type": "text",
                                "text": text,
                            }
                        ]
                    });

                    let response = JsonRpcResponse::success(request_id.clone(), response_payload);
                    Ok(json_response(response))
                }
                Err(error) => {
                    tracing::error!(?error, "Tool execution failed");
                    let response = JsonRpcResponse::failure(
                        request_id.clone(),
                        -32002,
                        "Tool execution failed".to_string(),
                        Some(error),
                    );
                    Ok(json_response(response))
                }
            }
        }
        _ => {
            let response = JsonRpcResponse::failure(
                request_id,
                -32601,
                format!("Unknown method: {}", request.method),
                None,
            );
            Ok(json_response(response))
        }
    }
}

type ResolveFuture<'a> =
    Pin<Box<dyn Future<Output = Result<IntegrationContext, ResolveError>> + Send + 'a>>;

fn resolve_integration_context<'a>(
    pool: &'a Pool,
    slug: &'a str,
    connection_id: Uuid,
) -> ResolveFuture<'a> {
    Box::pin(async move {
        #[cfg(test)]
        if let Some(context) = maybe_mock_resolver(slug, connection_id) {
            return Ok(context);
        }

        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;
        let slug_param = slug.to_ascii_lowercase();

        let base_context = db::queries::connections::mcp_connection_context()
            .bind(&transaction, &slug_param, &connection_id)
            .opt()
            .await?;

        let Some(base_context) = base_context else {
            return Err(ResolveError::NotFound {
                slug: slug.to_string(),
                connection_id,
            });
        };

        let definition = base_context
            .definition
            .clone()
            .ok_or(ResolveError::MissingDefinition)?;

        transaction
            .query(
                &format!(
                    "SET LOCAL row_level_security.user_id = {}",
                    base_context.user_id
                ),
                &[],
            )
            .await?;

        if let Some(key) = db::customer_keys::get_customer_key() {
            let escaped = key.replace('\'', "''");
            transaction
                .query(
                    &format!("SET LOCAL encryption.root_key = '{}'", escaped),
                    &[],
                )
                .await?;
        }

        let (team_id, connection) = match base_context.connection_type.as_str() {
            "api_key" => {
                let api_key_secret = db::queries::connections::mcp_api_key_connection_secret()
                    .bind(&transaction, &slug_param, &connection_id)
                    .one()
                    .await?;

                let api_key = api_key_secret
                    .api_key
                    .ok_or(ResolveError::MissingSecret("api_key"))?;

                let team_id_row = transaction
                    .query_one(
                        "SELECT team_id FROM api_key_connections WHERE id = $1",
                        &[&api_key_secret.connection_id],
                    )
                    .await?;
                let team_id: i32 = team_id_row.get(0);

                (
                    team_id,
                    ConnectionAuth::ApiKey {
                        connection_id: api_key_secret.connection_id,
                        api_key,
                    },
                )
            }
            "oauth2" => {
                let oauth_secret = db::queries::connections::mcp_oauth2_connection_secret()
                    .bind(&transaction, &slug_param, &connection_id)
                    .one()
                    .await?;

                let access_token = oauth_secret
                    .access_token
                    .ok_or(ResolveError::MissingSecret("access_token"))?;

                let team_id_row = transaction
                    .query_one(
                        "SELECT team_id FROM oauth2_connections WHERE id = $1",
                        &[&oauth_secret.connection_id],
                    )
                    .await?;
                let team_id: i32 = team_id_row.get(0);

                (
                    team_id,
                    ConnectionAuth::OAuth2 {
                        connection_id: oauth_secret.connection_id,
                        access_token,
                        refresh_token: oauth_secret.refresh_token,
                        expires_at: oauth_secret.expires_at,
                    },
                )
            }
            other => return Err(ResolveError::UnsupportedConnection(other.to_string())),
        };

        transaction.commit().await?;

        Ok(IntegrationContext {
            definition,
            integration_id: base_context.integration_id,
            user_id: base_context.user_id,
            team_id,
            user_openid_sub: base_context.user_openid_sub.clone(),
            connection,
        })
    })
}

fn json_response(response: JsonRpcResponse) -> Response {
    tracing::info!("{:?}", response);
    (StatusCode::OK, Json(response)).into_response()
}

fn arguments_to_string(value: Value) -> Result<String, serde_json::Error> {
    match value {
        Value::String(s) => Ok(s),
        other => serde_json::to_string(&other),
    }
}

#[cfg(test)]
fn maybe_mock_resolver(slug: &str, connection_id: Uuid) -> Option<IntegrationContext> {
    let lock = MOCK_RESOLVER.get_or_init(|| Mutex::new(None));
    let guard = lock.lock().unwrap();
    guard
        .as_ref()
        .map(|resolver| (resolver)(slug, connection_id))
}

#[cfg(test)]
fn set_mock_resolver<F>(resolver: F)
where
    F: Fn(&str, Uuid) -> IntegrationContext + Send + Sync + 'static,
{
    let lock = MOCK_RESOLVER.get_or_init(|| Mutex::new(None));
    let mut guard = lock.lock().unwrap();
    *guard = Some(Box::new(resolver));
}

#[cfg(test)]
fn clear_mock_resolver() {
    if let Some(lock) = MOCK_RESOLVER.get() {
        let mut guard = lock.lock().unwrap();
        *guard = None;
    }
}

#[cfg(test)]
mod tests;
