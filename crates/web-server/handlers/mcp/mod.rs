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
use std::sync::{Mutex, MutexGuard, OnceLock};

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

    let api_key_team_id = validate_api_key(&pool, api_key_value).await?;

    tracing::debug!("{:?}", payload);

    if payload.get("id").is_none() {
        tracing::debug!("Payload doesn't have an id");
        return Ok(StatusCode::NO_CONTENT.into_response());
    }

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

    if context.team_id != api_key_team_id {
        return Err(CustomError::Authentication(
            "API key is not authorized for this connection".to_string(),
        ));
    }

    let context_definition = context.definition.clone();
    let integration_openapi = match BionicOpenAPI::new(&context_definition) {
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

    let canonical_spec = match mcp::find_spec(&slug) {
        Some(spec) => match serde_json::from_str::<Value>(spec.json) {
            Ok(value) => value,
            Err(err) => {
                tracing::error!("Failed to parse canonical MCP spec: {}", err);
                let response = JsonRpcResponse::failure(
                    request_id.clone(),
                    -32603,
                    "Failed to parse canonical MCP specification".to_string(),
                    Some(json!({ "details": err.to_string() })),
                );
                return Ok(json_response(response));
            }
        },
        None => context_definition,
    };

    let openapi = match BionicOpenAPI::new(&canonical_spec) {
        Ok(api) => api,
        Err(err) => {
            tracing::error!("Failed to parse MCP specification: {}", err);
            let response = JsonRpcResponse::failure(
                request_id.clone(),
                -32603,
                "Failed to parse MCP specification".to_string(),
                Some(json!({ "details": err.to_string() })),
            );
            return Ok(json_response(response));
        }
    };

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
                    let response =
                        JsonRpcResponse::success(request_id.clone(), json!({ "output": result }));
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
    tracing::debug!("{:?}", response);
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
mod tests {
    use super::*;
    use http_body_util::BodyExt;
    use serde_json::json;
    use tokio::task::JoinHandle;
    use tower::ServiceExt;

    struct ResolverGuard {
        _lock: MutexGuard<'static, ()>,
    }

    impl ResolverGuard {
        fn new<F>(resolver: F) -> Self
        where
            F: Fn(&str, Uuid) -> IntegrationContext + Send + Sync + 'static,
        {
            let lock = RESOLVER_LOCK.get_or_init(|| Mutex::new(()));
            let guard = lock.lock().unwrap();
            set_mock_resolver(resolver);
            Self { _lock: guard }
        }
    }

    impl Drop for ResolverGuard {
        fn drop(&mut self) {
            clear_mock_resolver();
        }
    }

    struct ApiKeyGuard {
        key: String,
    }

    impl ApiKeyGuard {
        fn new(key: &str, team_id: i32) -> Self {
            register_mock_api_key(key, team_id);
            Self {
                key: key.to_string(),
            }
        }
    }

    impl Drop for ApiKeyGuard {
        fn drop(&mut self) {
            remove_mock_api_key(&self.key);
        }
    }

    fn create_test_pool() -> Pool {
        db::create_pool("postgres://postgres:postgres@127.0.0.1:1/postgres")
    }

    fn test_router(pool: Pool) -> Router {
        Router::new().merge(routes()).layer(Extension(pool))
    }

    fn sample_spec(base_url: &str, slug: &str) -> Value {
        json!({
            "openapi": "3.0.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0",
                "bionic-slug": slug,
                "x-bionic-slug": slug,
            },
            "servers": [
                { "url": base_url }
            ],
            "paths": {
                "/ping": {
                    "get": {
                        "operationId": "ping",
                        "summary": "Ping",
                        "description": "Ping endpoint",
                        "responses": {
                            "200": {
                                "description": "success"
                            }
                        }
                    }
                }
            }
        })
    }

    fn minimal_spec(slug: &str) -> Value {
        json!({
            "openapi": "3.0.0",
            "info": {
                "title": "Minimal API",
                "version": "1.0.0",
                "bionic-slug": slug,
                "x-bionic-slug": slug,
            },
            "servers": [
                { "url": "http://example.com" }
            ],
            "paths": {}
        })
    }

    async fn spawn_ping_service(response: Value) -> (String, JoinHandle<()>) {
        let app = Router::new().route(
            "/ping",
            axum::routing::get(move || {
                let payload = response.clone();
                async move { Json(payload) }
            }),
        );

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("listener");
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app.into_make_service())
                .await
                .expect("server");
        });
        (format!("http://{}", addr), handle)
    }

    #[tokio::test]
    async fn session_initialize_returns_capabilities() {
        let pool = create_test_pool();
        let slug = "test".to_string();
        let connection_id = Uuid::new_v4();
        let spec = sample_spec("http://example.com", &slug);

        let api_key_value = "test-capabilities-key";

        let context = IntegrationContext {
            definition: spec,
            integration_id: 7,
            user_id: 11,
            team_id: 21,
            user_openid_sub: Some("user-1".to_string()),
            connection: ConnectionAuth::ApiKey {
                connection_id: 42,
                api_key: "abc".to_string(),
            },
        };

        let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);

        let slug_for_guard = slug.clone();
        let _guard = ResolverGuard::new(move |requested_slug, requested_id| {
            assert_eq!(requested_slug, slug_for_guard);
            assert_eq!(requested_id, connection_id);
            context.clone()
        });

        let app = test_router(pool.clone());

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "session.initialize",
            "params": {}
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", api_key_value))
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["result"]["protocolVersion"], DEFAULT_PROTOCOL_VERSION);
        assert_eq!(
            json["result"]["capabilities"]["tools"]["listChanged"],
            false
        );
        assert_eq!(json["result"]["metadata"]["integrationId"], 7);
    }

    #[tokio::test]
    async fn initialize_returns_server_info() {
        let pool = create_test_pool();
        let slug = "test".to_string();
        let connection_id = Uuid::new_v4();
        let spec = sample_spec("http://example.com", &slug);

        let api_key_value = "test-initialize-key";

        let context = IntegrationContext {
            definition: spec,
            integration_id: 12,
            user_id: 44,
            team_id: 33,
            user_openid_sub: Some("user-7".to_string()),
            connection: ConnectionAuth::ApiKey {
                connection_id: 88,
                api_key: "stu".to_string(),
            },
        };

        let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);

        let slug_for_guard = slug.clone();
        let _guard = ResolverGuard::new(move |requested_slug, requested_id| {
            assert_eq!(requested_slug, slug_for_guard);
            assert_eq!(requested_id, connection_id);
            context.clone()
        });

        let app = test_router(pool.clone());

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 99,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {
                    "elicitation": {}
                },
                "clientInfo": {
                    "name": "example-client",
                    "version": "1.0.0"
                }
            }
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", api_key_value))
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["result"]["protocolVersion"], "2025-06-18");
        assert_eq!(
            json["result"]["capabilities"]["tools"]["listChanged"],
            false
        );
        assert_eq!(json["result"]["serverInfo"]["name"], SERVER_NAME);
        assert_eq!(json["result"]["serverInfo"]["version"], SERVER_VERSION);
        assert_eq!(json["result"]["metadata"]["integrationId"], 12);
    }

    #[tokio::test]
    async fn initialize_supports_march_2025_protocol() {
        let pool = create_test_pool();
        let slug = "test".to_string();
        let connection_id = Uuid::new_v4();
        let spec = sample_spec("http://example.com", &slug);

        let api_key_value = "test-initialize-march-key";

        let context = IntegrationContext {
            definition: spec,
            integration_id: 13,
            user_id: 45,
            team_id: 34,
            user_openid_sub: Some("user-8".to_string()),
            connection: ConnectionAuth::ApiKey {
                connection_id: 89,
                api_key: "vwx".to_string(),
            },
        };

        let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);

        let slug_for_guard = slug.clone();
        let _guard = ResolverGuard::new(move |requested_slug, requested_id| {
            assert_eq!(requested_slug, slug_for_guard);
            assert_eq!(requested_id, connection_id);
            context.clone()
        });

        let app = test_router(pool.clone());

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 101,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-03-26",
                "capabilities": {
                    "elicitation": {}
                },
                "clientInfo": {
                    "name": "example-client",
                    "version": "1.0.0"
                }
            }
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", api_key_value))
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["result"]["protocolVersion"], "2025-03-26");
        assert_eq!(
            json["result"]["capabilities"]["tools"]["listChanged"],
            false
        );
        assert_eq!(json["result"]["serverInfo"]["name"], SERVER_NAME);
        assert_eq!(json["result"]["serverInfo"]["version"], SERVER_VERSION);
        assert_eq!(json["result"]["metadata"]["integrationId"], 13);
    }

    #[tokio::test]
    async fn tools_list_returns_available_tools() {
        let pool = create_test_pool();
        let slug = "test".to_string();
        let connection_id = Uuid::new_v4();
        let spec = sample_spec("http://example.com", &slug);

        let api_key_value = "test-tools-list-key";

        let context = IntegrationContext {
            definition: spec,
            integration_id: 9,
            user_id: 22,
            team_id: 44,
            user_openid_sub: Some("user-2".to_string()),
            connection: ConnectionAuth::ApiKey {
                connection_id: 55,
                api_key: "def".to_string(),
            },
        };

        let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
        let _guard = ResolverGuard::new(move |_, _| context.clone());
        let app = test_router(pool.clone());

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", api_key_value))
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["result"]["tools"].as_array().unwrap().len(), 1);
        assert_eq!(json["result"]["tools"][0]["name"], "ping");
    }

    #[tokio::test]
    async fn tools_list_uses_canonical_spec_when_available() {
        let pool = create_test_pool();
        let slug = "dropbox".to_string();
        let connection_id = Uuid::new_v4();
        let spec = minimal_spec(&slug);

        let api_key_value = "test-canonical-key";

        let context = IntegrationContext {
            definition: spec,
            integration_id: 10,
            user_id: 33,
            team_id: 52,
            user_openid_sub: Some("user-4".to_string()),
            connection: ConnectionAuth::ApiKey {
                connection_id: 70,
                api_key: "jkl".to_string(),
            },
        };

        let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
        let _guard = ResolverGuard::new(move |_, _| context.clone());
        let app = test_router(pool.clone());

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 4,
            "method": "tools/list",
            "params": {}
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", api_key_value))
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        let tools = json["result"]["tools"].as_array().unwrap();
        assert!(!tools.is_empty(), "expected canonical spec tools");
    }

    #[tokio::test]
    async fn tools_call_executes_tool() {
        let pool = create_test_pool();
        let slug = "test".to_string();
        let connection_id = Uuid::new_v4();
        let (base_url, handle) = spawn_ping_service(json!({ "ok": true })).await;
        let spec = sample_spec(&base_url, &slug);

        let api_key_value = "test-tools-call-key";

        let context = IntegrationContext {
            definition: spec,
            integration_id: 3,
            user_id: 12,
            team_id: 65,
            user_openid_sub: Some("user-3".to_string()),
            connection: ConnectionAuth::ApiKey {
                connection_id: 60,
                api_key: "ghi".to_string(),
            },
        };

        let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
        let _guard = ResolverGuard::new(move |_, _| context.clone());
        let app = test_router(pool.clone());

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "ping",
                "arguments": {}
            }
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", api_key_value))
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        handle.abort();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["result"]["output"]["ok"], true);
    }

    #[tokio::test]
    async fn notifications_without_id_are_ignored() {
        let pool = create_test_pool();
        let slug = "test".to_string();
        let connection_id = Uuid::new_v4();
        let app = test_router(pool.clone());

        let api_key_value = "test-notification-key";
        let _api_guard = ApiKeyGuard::new(api_key_value, 77);

        let payload = json!({
            "jsonrpc": "2.0",
            "method": "session.initialize",
            "params": {}
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", api_key_value))
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn unknown_method_returns_error() {
        let pool = create_test_pool();
        let slug = "test".to_string();
        let connection_id = Uuid::new_v4();
        let spec = sample_spec("http://example.com", &slug);

        let api_key_value = "test-unknown-method-key";

        let context = IntegrationContext {
            definition: spec,
            integration_id: 8,
            user_id: 22,
            team_id: 80,
            user_openid_sub: Some("user-5".to_string()),
            connection: ConnectionAuth::ApiKey {
                connection_id: 75,
                api_key: "mno".to_string(),
            },
        };

        let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
        let _guard = ResolverGuard::new(move |_, _| context.clone());
        let app = test_router(pool.clone());

        let payload = json!({
            "jsonrpc": "2.0",
            "id": "abc",
            "method": "unknown.method",
            "params": {}
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", api_key_value))
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"]["code"], -32601);
        assert_eq!(json["error"]["message"], "Unknown method: unknown.method");
    }

    #[tokio::test]
    async fn tools_call_with_invalid_params_returns_error() {
        let pool = create_test_pool();
        let slug = "test".to_string();
        let connection_id = Uuid::new_v4();
        let spec = sample_spec("http://example.com", &slug);

        let api_key_value = "test-invalid-params-key";

        let context = IntegrationContext {
            definition: spec,
            integration_id: 5,
            user_id: 18,
            team_id: 84,
            user_openid_sub: Some("user-6".to_string()),
            connection: ConnectionAuth::ApiKey {
                connection_id: 81,
                api_key: "pqr".to_string(),
            },
        };

        let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
        let _guard = ResolverGuard::new(move |_, _| context.clone());
        let app = test_router(pool.clone());

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 9,
            "method": "tools/call",
            "params": {}
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", api_key_value))
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"]["code"], -32602);
        assert_eq!(
            json["error"]["message"],
            "Invalid parameters for tools/call"
        );
    }
}
