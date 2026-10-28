//! Small Streamable HTTP MCP client used by remote MCP integrations.

use db::{authz, Pool};
use futures_util::StreamExt;
use reqwest::{redirect::Policy, Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
const MAX_MCP_RESPONSE_BYTES: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, rename = "inputSchema")]
    pub input_schema: Value,
    #[serde(default, rename = "_meta")]
    pub meta: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceContent {
    #[serde(default)]
    pub uri: String,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub blob: Option<String>,
}

#[derive(Debug, Clone)]
pub struct McpClient {
    client: Client,
    endpoint: Url,
    bearer_token: Option<String>,
    session_id: Option<String>,
    initialized: bool,
    protocol_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct McpServerConfig {
    pub id: i32,
    pub name: String,
    pub server_url: String,
    pub bearer_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub token_url: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

pub async fn servers_for_conversation(
    pool: &Pool,
    sub: &str,
    conversation_id: i64,
) -> Result<Vec<McpServerConfig>, String> {
    let mut client = pool.get().await.map_err(|error| error.to_string())?;
    let transaction = client
        .transaction()
        .await
        .map_err(|error| error.to_string())?;
    authz::set_row_level_security_user_id(&transaction, sub.to_owned())
        .await
        .map_err(|error| error.to_string())?;
    let rows = transaction
        .query(
            "SELECT s.id, s.name, s.server_url, s.auth_type, decrypt_text(s.access_token) AS access_token, \
                    decrypt_text(s.refresh_token) AS refresh_token, s.expires_at, s.token_url, s.client_id, \
                    decrypt_text(s.client_secret) AS client_secret \
             FROM integrations.mcp_servers s \
             JOIN llm.conversations c ON c.id = $1 AND c.team_id = s.team_id \
             WHERE (s.visibility = 'Team' OR s.created_by = current_app_user()) \
               AND (s.auth_type <> 'oauth2' OR s.access_token IS NOT NULL) \
             ORDER BY s.name, s.id",
            &[&conversation_id],
        )
        .await
        .map_err(|error| error.to_string())?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    let mut servers = rows
        .into_iter()
        .map(|row| McpServerConfig {
            id: row.get("id"),
            name: row.get("name"),
            server_url: row.get("server_url"),
            bearer_token: row.get("access_token"),
            refresh_token: row.get("refresh_token"),
            expires_at: row.get("expires_at"),
            token_url: row.get("token_url"),
            client_id: row.get("client_id"),
            client_secret: row.get("client_secret"),
        })
        .collect::<Vec<_>>();
    for server in &mut servers {
        refresh_expiring_token(pool, sub, server).await?;
    }
    Ok(servers)
}

#[derive(Deserialize)]
struct RefreshResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
}

pub async fn refresh_expiring_token(
    pool: &Pool,
    sub: &str,
    server: &mut McpServerConfig,
) -> Result<(), String> {
    let expires_soon = server
        .expires_at
        .map(|expires_at| expires_at <= chrono::Utc::now() + chrono::Duration::seconds(30))
        .unwrap_or(false);
    if !expires_soon {
        return Ok(());
    }
    let Some(refresh_token) = server.refresh_token.clone() else {
        return Err(format!(
            "OAuth token for {} has expired; reconnect the MCP server",
            server.name
        ));
    };
    let token_url = server
        .token_url
        .as_deref()
        .ok_or("OAuth token endpoint is missing")?;
    let client_id = server
        .client_id
        .as_deref()
        .ok_or("OAuth client ID is missing")?;
    let (http, endpoint) = secure_endpoint_client(token_url).await?;
    let mut params = vec![
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token.as_str()),
        ("client_id", client_id),
    ];
    if let Some(secret) = server.client_secret.as_deref() {
        params.push(("client_secret", secret));
    }
    let response = http
        .post(endpoint)
        .form(&params)
        .send()
        .await
        .map_err(|error| error.to_string())?;
    if !response.status().is_success() {
        return Err(format!(
            "OAuth refresh for {} returned HTTP {}",
            server.name,
            response.status()
        ));
    }
    let tokens: RefreshResponse = response.json().await.map_err(|error| error.to_string())?;
    let expires_at = tokens
        .expires_in
        .map(|seconds| chrono::Utc::now() + chrono::Duration::seconds(seconds));
    let mut client = pool.get().await.map_err(|error| error.to_string())?;
    let transaction = client
        .transaction()
        .await
        .map_err(|error| error.to_string())?;
    authz::set_row_level_security_user_id(&transaction, sub.to_owned())
        .await
        .map_err(|error| error.to_string())?;
    transaction
        .execute(
            "UPDATE integrations.mcp_servers SET access_token = encrypt_text($1), \
             refresh_token = encrypt_text(COALESCE($2, decrypt_text(refresh_token))), expires_at = $3 \
             WHERE id = $4",
            &[&tokens.access_token, &tokens.refresh_token, &expires_at, &server.id],
        )
        .await
        .map_err(|error| error.to_string())?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    server.bearer_token = Some(tokens.access_token);
    if let Some(refresh_token) = tokens.refresh_token {
        server.refresh_token = Some(refresh_token);
    }
    server.expires_at = expires_at;
    Ok(())
}

impl McpClient {
    pub async fn new(endpoint: &str, bearer_token: Option<String>) -> Result<Self, String> {
        let (client, endpoint) = secure_endpoint_client(endpoint).await?;
        Ok(Self {
            client,
            endpoint,
            bearer_token,
            session_id: None,
            initialized: false,
            protocol_version: None,
        })
    }

    pub async fn initialize(&mut self) -> Result<(), String> {
        let result = self
            .request(
                "initialize",
                json!({
                    "protocolVersion": "2025-11-25",
                    "capabilities": { "extensions": { "io.modelcontextprotocol/ui": {} } },
                    "clientInfo": { "name": "bionic-gpt", "version": env!("CARGO_PKG_VERSION") }
                }),
            )
            .await?;
        if result
            .get("protocolVersion")
            .and_then(Value::as_str)
            .is_none()
        {
            return Err("MCP server returned an invalid initialize result".into());
        }
        self.protocol_version = result
            .get("protocolVersion")
            .and_then(Value::as_str)
            .map(str::to_owned);
        self.notification("notifications/initialized", json!({}))
            .await?;
        self.initialized = true;
        Ok(())
    }

    pub async fn list_tools(&mut self) -> Result<Vec<McpTool>, String> {
        self.ensure_initialized().await?;
        let mut tools = Vec::new();
        let mut cursor: Option<String> = None;
        let mut cursors = std::collections::HashSet::new();
        for _ in 0..100 {
            let mut params = json!({});
            if let Some(cursor) = &cursor {
                params["cursor"] = json!(cursor);
            }
            let result = self.request("tools/list", params).await?;
            let page: Vec<McpTool> =
                serde_json::from_value(result.get("tools").cloned().unwrap_or_else(|| json!([])))
                    .map_err(|error| format!("Invalid MCP tools/list response: {error}"))?;
            tools.extend(page);
            cursor = result
                .get("nextCursor")
                .and_then(Value::as_str)
                .map(str::to_owned);
            if cursor.is_none() {
                return Ok(tools);
            }
            if !cursors.insert(cursor.clone().unwrap_or_default()) {
                return Err("MCP tools/list returned a repeated cursor".into());
            }
        }
        Err("MCP tools/list exceeded the page limit".into())
    }

    pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<Value, String> {
        self.ensure_initialized().await?;
        self.request(
            "tools/call",
            json!({ "name": name, "arguments": arguments }),
        )
        .await
    }

    pub async fn read_resource(&mut self, uri: &str) -> Result<Value, String> {
        self.ensure_initialized().await?;
        self.request("resources/read", json!({ "uri": uri })).await
    }

    async fn ensure_initialized(&mut self) -> Result<(), String> {
        if !self.initialized {
            self.initialize().await?;
        }
        Ok(())
    }

    async fn notification(&mut self, method: &str, params: Value) -> Result<(), String> {
        let mut request = self
            .client
            .post(self.endpoint.clone())
            .header(
                reqwest::header::ACCEPT,
                "application/json, text/event-stream",
            )
            .json(&json!({
                "jsonrpc": "2.0", "method": method, "params": params
            }));
        request = self.with_headers(request);
        let response = request.send().await.map_err(|error| error.to_string())?;
        self.capture_session(&response);
        if !response.status().is_success() && response.status().as_u16() != 202 {
            return Err(format!(
                "MCP notification failed with HTTP {}",
                response.status()
            ));
        }
        Ok(())
    }

    async fn request(&mut self, method: &str, params: Value) -> Result<Value, String> {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let mut request = self
            .client
            .post(self.endpoint.clone())
            .header(
                reqwest::header::ACCEPT,
                "application/json, text/event-stream",
            )
            .json(&json!({
                "jsonrpc": "2.0", "id": id, "method": method, "params": params
            }));
        request = self.with_headers(request);
        let response = request.send().await.map_err(|error| error.to_string())?;
        self.capture_session(&response);
        let status = response.status();
        if !status.is_success() {
            return Err(format!("MCP request failed with HTTP {status}"));
        }
        if status.as_u16() == 202 || status.as_u16() == 204 {
            return Err("MCP server did not return a JSON-RPC response".into());
        }
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let value: Value = if content_type.contains("text/event-stream") {
            let stream = limited_response_text(response, MAX_MCP_RESPONSE_BYTES).await?;
            stream
                .lines()
                .filter_map(|line| line.strip_prefix("data:"))
                .filter_map(|data| serde_json::from_str::<Value>(data.trim()).ok())
                .find(|message| message.get("id") == Some(&json!(id)))
                .ok_or_else(|| {
                    "MCP SSE response contained no matching JSON-RPC response".to_string()
                })?
        } else {
            let body = limited_response_text(response, MAX_MCP_RESPONSE_BYTES).await?;
            serde_json::from_str::<Value>(&body).map_err(|error| error.to_string())?
        };
        if value.get("id") != Some(&json!(id)) {
            return Err(format!(
                "MCP {method} response id did not match the request"
            ));
        }
        if let Some(error) = value.get("error") {
            return Err(format!("MCP {method} failed: {error}"));
        }
        value
            .get("result")
            .cloned()
            .ok_or_else(|| format!("MCP {method} response has no result"))
    }

    fn with_headers(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let request = if let Some(version) = &self.protocol_version {
            request.header("MCP-Protocol-Version", version)
        } else {
            request
        };
        let request = if let Some(session_id) = &self.session_id {
            request.header("MCP-Session-Id", session_id)
        } else {
            request
        };
        if let Some(token) = &self.bearer_token {
            request.bearer_auth(token)
        } else {
            request
        }
    }

    fn capture_session(&mut self, response: &reqwest::Response) {
        if let Some(session_id) = response.headers().get("MCP-Session-Id") {
            if let Ok(session_id) = session_id.to_str() {
                self.session_id = Some(session_id.to_owned());
            }
        }
    }
}

pub async fn secure_endpoint_client(endpoint: &str) -> Result<(Client, Url), String> {
    let endpoint = Url::parse(endpoint).map_err(|error| error.to_string())?;
    if endpoint.scheme() != "https" || endpoint.host_str().is_none() {
        return Err("MCP server URL must use HTTPS".into());
    }
    if !endpoint.username().is_empty() || endpoint.password().is_some() {
        return Err("Credentials must be configured separately from the MCP server URL".into());
    }
    let addresses = crate::connector_network::validate_public_destination(&endpoint)
        .await
        .map_err(|error| error.to_string())?;
    let host = endpoint
        .host_str()
        .ok_or_else(|| "MCP server URL has no host".to_string())?;
    let client = Client::builder()
        .redirect(Policy::none())
        .no_proxy()
        .resolve_to_addrs(host, &addresses)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|error| error.to_string())?;
    Ok((client, endpoint))
}

async fn limited_response_text(
    response: reqwest::Response,
    limit: usize,
) -> Result<String, String> {
    if response
        .content_length()
        .is_some_and(|length| length > limit as u64)
    {
        return Err("MCP response exceeded the size limit".into());
    }
    let mut stream = response.bytes_stream();
    let mut body = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|error| error.to_string())?;
        if body.len().saturating_add(chunk.len()) > limit {
            return Err("MCP response exceeded the size limit".into());
        }
        body.extend_from_slice(&chunk);
    }
    String::from_utf8(body).map_err(|error| error.to_string())
}
