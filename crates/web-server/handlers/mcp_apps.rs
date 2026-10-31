use crate::{CustomError, Jwt};
use axum::Router;
use axum::{
    extract::{Extension, Query},
    http::{header, HeaderValue},
    response::{Html, IntoResponse, Redirect, Response},
    Form, Json,
};
use axum_extra::routing::RouterExt;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{Duration, Utc};
use db::{authz, Pool};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use tool_runtime::mcp_client::{validated_endpoint_client, McpClient, McpServerConfig};
use uuid::Uuid;
use web_pages::string_to_visibility;
use web_pages::{
    mcp_apps::McpServerSummary,
    routes::mcp_apps::{Asset, Authorize, Call, Create, Delete, Index, OAuth2Callback, Resource},
};

pub fn routes() -> Router {
    Router::new()
        .typed_get(index)
        .typed_post(create)
        .typed_post(delete)
        .typed_get(authorize)
        .typed_get(oauth2_callback)
        .typed_get(resource)
        .typed_get(asset)
        .typed_post(call)
}

async fn index(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;
    if !rbac.can_view_integrations() {
        return Err(CustomError::Authorization);
    }
    let rows = transaction
        .query(
            "SELECT id, name, server_url, visibility::text AS visibility, auth_type, access_token IS NOT NULL AS connected \
         FROM integrations.mcp_servers WHERE team_id = $1 \
         AND (visibility = 'Team' OR created_by = current_app_user()) ORDER BY name, id",
            &[&team_id_num],
        )
        .await?;
    let servers = rows
        .into_iter()
        .map(|row| McpServerSummary {
            id: row.get("id"),
            name: row.get("name"),
            server_url: row.get("server_url"),
            visibility: row.get("visibility"),
            auth_type: row.get("auth_type"),
            connected: row.get("connected"),
        })
        .collect();
    transaction.commit().await?;
    Ok(Html(web_pages::mcp_apps::page(
        team_id, rbac, servers, None,
    )))
}

#[derive(Deserialize)]
pub struct NewMcpServer {
    name: String,
    server_url: String,
    auth_type: String,
    bearer_token: Option<String>,
    authorization_url: Option<String>,
    token_url: Option<String>,
    oauth_scopes: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    visibility: String,
}

async fn create(
    Create { team_id }: Create,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<NewMcpServer>,
) -> Result<impl IntoResponse, CustomError> {
    let name = form.name.trim();
    if name.is_empty() || name.len() > 120 {
        return Err(CustomError::FaultySetup(
            "Enter a name up to 120 characters long".into(),
        ));
    }
    let auth_type = form.auth_type.as_str();
    if !matches!(form.visibility.as_str(), "Private" | "Team") {
        return Err(CustomError::FaultySetup(
            "Choose personal or team visibility".into(),
        ));
    }
    if form
        .oauth_scopes
        .as_deref()
        .is_some_and(|scopes| scopes.len() > 2048)
    {
        return Err(CustomError::FaultySetup("OAuth scopes are too long".into()));
    }
    let bearer_token = form
        .bearer_token
        .as_deref()
        .map(str::trim)
        .filter(|token| !token.is_empty());
    if !matches!(auth_type, "none" | "bearer" | "oauth2")
        || (auth_type == "bearer" && bearer_token.is_none())
    {
        return Err(CustomError::FaultySetup(
            "Choose no authentication, provide a bearer token, or configure OAuth 2.1".into(),
        ));
    }
    let (authorization_url, token_url, client_id) = if auth_type == "oauth2" {
        let authorization_url = validated_http_url(form.authorization_url.as_deref()).await?;
        let token_url = validated_http_url(form.token_url.as_deref()).await?;
        let client_id = form
            .client_id
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| CustomError::FaultySetup("OAuth client ID is required".into()))?;
        (
            Some(authorization_url),
            Some(token_url),
            Some(client_id.to_owned()),
        )
    } else {
        (None, None, None)
    };
    let mut mcp_client = McpClient::new(&form.server_url, bearer_token.map(str::to_owned))
        .await
        .map_err(CustomError::FaultySetup)?;
    if auth_type != "oauth2" {
        mcp_client
            .list_tools()
            .await
            .map_err(CustomError::ExternalApi)?;
    }

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;
    if !rbac.can_view_integrations()
        || (form.visibility == "Team" && !rbac.can_manage_integrations())
    {
        return Err(CustomError::Authorization);
    }
    let visibility = string_to_visibility(&form.visibility);
    let encrypted_token = if auth_type == "bearer" {
        bearer_token.map(str::to_owned)
    } else {
        None
    };
    let encrypted_client_secret = form
        .client_secret
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    let oauth_scopes = form
        .oauth_scopes
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    transaction.execute(
        "INSERT INTO integrations.mcp_servers (team_id, return_team_id, created_by, visibility, name, server_url, auth_type, access_token, authorization_url, token_url, oauth_scopes, client_id, client_secret) \
         VALUES ($1, $2, current_app_user(), $3, $4, $5, $6, encrypt_text($7::text), $8, $9, $10, $11, encrypt_text($12::text))",
        &[&team_id_num, &team_id, &visibility, &name, &form.server_url, &auth_type, &encrypted_token, &authorization_url, &token_url, &oauth_scopes, &client_id, &encrypted_client_secret],
    ).await?;
    transaction.commit().await?;
    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::mcp_apps::Index { team_id }.to_string(),
        "MCP server connected",
    ))
}

async fn delete(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;
    if !rbac.can_view_integrations() {
        return Err(CustomError::Authorization);
    }
    let deleted = transaction
        .execute(
            "DELETE FROM integrations.mcp_servers WHERE id = $1 AND team_id = $2 \
         AND ((visibility = 'Private' AND created_by = current_app_user()) OR (visibility = 'Team' AND $3))",
            &[&id, &team_id_num, &rbac.can_manage_integrations()],
        )
        .await?;
    if deleted == 0 {
        return Err(CustomError::Authorization);
    }
    transaction.commit().await?;
    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::mcp_apps::Index { team_id }.to_string(),
        "MCP server disconnected",
    ))
}

async fn validated_http_url(value: Option<&str>) -> Result<String, CustomError> {
    let value = value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| CustomError::FaultySetup("Enter an HTTP or HTTPS URL".into()))?;
    validated_endpoint_client(value)
        .await
        .map_err(CustomError::FaultySetup)?;
    Ok(value.to_owned())
}

async fn authorize(
    Authorize { team_id, id }: Authorize,
    current_user: Jwt,
    Extension(config): Extension<crate::config::Config>,
    Extension(pool): Extension<Pool>,
) -> Result<Redirect, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;
    if !rbac.can_view_integrations() {
        return Err(CustomError::Authorization);
    }
    let row = transaction
        .query_opt(
            "SELECT authorization_url, client_id, oauth_scopes FROM integrations.mcp_servers \
             WHERE id = $1 AND team_id = $2 AND auth_type = 'oauth2' \
             AND (visibility = 'Team' OR created_by = current_app_user())",
            &[&id, &team_id_num],
        )
        .await?
        .ok_or(CustomError::Authorization)?;
    let state = Uuid::new_v4().to_string();
    let verifier = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    transaction
        .execute(
            "DELETE FROM integrations.mcp_oauth_states WHERE created_at < NOW() - INTERVAL '1 day'",
            &[],
        )
        .await?;
    transaction
        .execute(
            "INSERT INTO integrations.mcp_oauth_states (state, server_id, user_id, team_id, return_team_id, pkce_verifier) \
             VALUES ($1, $2, current_app_user(), $3, $4, $5)",
            &[&state, &id, &team_id_num, &team_id, &verifier],
        )
        .await?;
    transaction.commit().await?;

    let mut authorization_url = url::Url::parse(&row.get::<_, String>("authorization_url"))
        .map_err(|error| CustomError::FaultySetup(error.to_string()))?;
    let callback_url = format!(
        "{}{}",
        config.base_url.trim_end_matches('/'),
        OAuth2Callback {}
    );
    authorization_url
        .query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", &row.get::<_, String>("client_id"))
        .append_pair("redirect_uri", &callback_url)
        .append_pair("state", &state)
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256");
    if let Some(scopes) = row.get::<_, Option<String>>("oauth_scopes") {
        authorization_url
            .query_pairs_mut()
            .append_pair("scope", &scopes);
    }
    Ok(Redirect::temporary(authorization_url.as_str()))
}

#[derive(Deserialize)]
struct OAuthCallbackQuery {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
}

async fn oauth2_callback(
    OAuth2Callback {}: OAuth2Callback,
    Query(query): Query<OAuthCallbackQuery>,
    current_user: Jwt,
    Extension(config): Extension<crate::config::Config>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let user_sub = current_user.sub.clone();
    authz::set_row_level_security_user_id(&transaction, user_sub.clone()).await?;
    let row = transaction
        .query_opt(
            "SELECT s.server_id, s.team_id, s.return_team_id, s.pkce_verifier, m.token_url, m.client_id, \
                    decrypt_text(m.client_secret) AS client_secret \
             FROM integrations.mcp_oauth_states s JOIN integrations.mcp_servers m ON m.id = s.server_id \
             WHERE s.state = $1 AND s.user_id = current_app_user() AND s.created_at > NOW() - INTERVAL '10 minutes'",
            &[&query.state],
        )
        .await?
        .ok_or(CustomError::Authorization)?;
    let server_id: i32 = row.get("server_id");
    let team_id: i32 = row.get("team_id");
    let return_team_id: String = row.get("return_team_id");
    let verifier: String = row.get("pkce_verifier");
    let token_url: String = row.get("token_url");
    let client_id: String = row.get("client_id");
    let client_secret: Option<String> = row.get("client_secret");
    transaction
        .execute(
            "DELETE FROM integrations.mcp_oauth_states WHERE state = $1",
            &[&query.state],
        )
        .await?;
    transaction.commit().await?;

    let redirect_uri = format!(
        "{}{}",
        config.base_url.trim_end_matches('/'),
        OAuth2Callback {}
    );
    let (http, token_endpoint) = validated_endpoint_client(&token_url)
        .await
        .map_err(CustomError::ExternalApi)?;
    let mut params = vec![
        ("grant_type", "authorization_code".to_owned()),
        ("code", query.code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("code_verifier", verifier),
    ];
    if let Some(secret) = client_secret {
        params.push(("client_secret", secret));
    }
    let response = http
        .post(token_endpoint)
        .form(&params)
        .send()
        .await
        .map_err(|error| CustomError::ExternalApi(error.to_string()))?;
    if !response.status().is_success() {
        return Err(CustomError::ExternalApi(format!(
            "OAuth token exchange returned HTTP {}",
            response.status()
        )));
    }
    let tokens: OAuthTokenResponse = response
        .json()
        .await
        .map_err(|error| CustomError::ExternalApi(error.to_string()))?;
    let expires_at = tokens
        .expires_in
        .map(|seconds| Utc::now() + Duration::seconds(seconds));

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    authz::set_row_level_security_user_id(&transaction, user_sub).await?;
    let updated = transaction
        .execute(
            "UPDATE integrations.mcp_servers SET access_token = encrypt_text($1), \
             refresh_token = encrypt_text(COALESCE($2, decrypt_text(refresh_token))), expires_at = $3 \
             WHERE id = $4 AND team_id = $5 AND auth_type = 'oauth2'",
            &[&tokens.access_token, &tokens.refresh_token, &expires_at, &server_id, &team_id],
        )
        .await?;
    if updated == 0 {
        return Err(CustomError::Authorization);
    }
    transaction.commit().await?;
    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::mcp_apps::Index {
            team_id: return_team_id,
        }
        .to_string(),
        "MCP server authorized",
    ))
}

#[derive(Deserialize)]
struct ResourceQuery {
    uri: String,
}

async fn resource(
    Resource { team_id, id }: Resource,
    Query(query): Query<ResourceQuery>,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Response, CustomError> {
    let (server, _rbac) = authorized_server(&pool, current_user, &team_id, id).await?;
    let mut client = McpClient::new(&server.server_url, server.bearer_token)
        .await
        .map_err(CustomError::ExternalApi)?;
    let result = client
        .read_resource(&query.uri)
        .await
        .map_err(CustomError::ExternalApi)?;
    let content = result
        .get("contents")
        .and_then(Value::as_array)
        .and_then(|contents| contents.first())
        .ok_or_else(|| CustomError::ExternalApi("MCP resource returned no content".into()))?;
    let mime = content
        .get("mimeType")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !mime.starts_with("text/html") || !mime.contains("mcp-app") {
        return Err(CustomError::ExternalApi(
            "MCP App resource must use text/html;profile=mcp-app".into(),
        ));
    }
    let html = content.get("text").and_then(Value::as_str).ok_or_else(|| {
        CustomError::ExternalApi("MCP App resource must contain HTML text".into())
    })?;
    let mut response = Html(html.to_owned()).into_response();
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(
        "sandbox allow-scripts allow-forms; default-src https: data: blob:; script-src https: 'unsafe-inline'; style-src https: 'unsafe-inline'; connect-src 'none'; frame-src 'none'; form-action 'none'"
    ));
    response.headers_mut().insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    if content
        .pointer("/_meta/bionic~1assets/files")
        .and_then(Value::as_array)
        .is_some()
    {
        response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static("sandbox allow-scripts; default-src 'none'; script-src 'unsafe-inline' blob: 'wasm-unsafe-eval'; style-src 'unsafe-inline'; img-src data: blob:; connect-src 'none'; frame-src 'none'; form-action 'none'"));
    }
    Ok(response)
}

async fn asset(
    Asset { team_id, id, path }: Asset,
    Query(query): Query<ResourceQuery>,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Response, CustomError> {
    if !valid_asset_path(&path) {
        return Err(CustomError::Authorization);
    }
    let (server, _) = authorized_server(&pool, current_user, &team_id, id).await?;
    let mut mcp = McpClient::new(&server.server_url, server.bearer_token.clone())
        .await
        .map_err(CustomError::ExternalApi)?;
    let resource = mcp
        .read_resource(&query.uri)
        .await
        .map_err(CustomError::ExternalApi)?;
    let files = resource
        .pointer("/contents/0/_meta/bionic~1assets/files")
        .and_then(Value::as_array)
        .ok_or(CustomError::Authorization)?;
    if !files.iter().any(|file| file.as_str() == Some(&path)) {
        return Err(CustomError::Authorization);
    }
    let endpoint = url::Url::parse(&server.server_url)
        .map_err(|e| CustomError::ExternalApi(e.to_string()))?
        .join(&format!("/assets/{path}"))
        .map_err(|e| CustomError::ExternalApi(e.to_string()))?;
    let (http, endpoint) = validated_endpoint_client(endpoint.as_str())
        .await
        .map_err(CustomError::ExternalApi)?;
    let mut request = http
        .get(endpoint)
        .timeout(std::time::Duration::from_secs(120));
    if let Some(token) = server.bearer_token {
        request = request.bearer_auth(token);
    }
    let mut upstream = request
        .send()
        .await
        .map_err(|e| CustomError::ExternalApi(e.to_string()))?;
    if !upstream.status().is_success() {
        return Err(CustomError::ExternalApi(format!(
            "Asset HTTP {}",
            upstream.status()
        )));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = upstream
        .chunk()
        .await
        .map_err(|e| CustomError::ExternalApi(e.to_string()))?
    {
        if bytes.len().saturating_add(chunk.len()) > 128 * 1024 * 1024 {
            return Err(CustomError::ExternalApi("Asset too large".into()));
        }
        bytes.extend_from_slice(&chunk);
    }
    let mime = if path.ends_with(".wasm") {
        "application/wasm"
    } else if path.ends_with(".js") {
        "text/javascript"
    } else {
        return Err(CustomError::Authorization);
    };
    Ok((
        [
            (header::CONTENT_TYPE, mime),
            (header::CACHE_CONTROL, "private, no-store"),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        ],
        bytes,
    )
        .into_response())
}

#[derive(Deserialize)]
struct CallRequest {
    name: String,
    #[serde(default)]
    arguments: Value,
}

async fn call(
    Call { team_id, id }: Call,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Json(request): Json<CallRequest>,
) -> Result<Json<Value>, CustomError> {
    let (server, _) = authorized_server(&pool, current_user, &team_id, id).await?;
    let mut client = McpClient::new(&server.server_url, server.bearer_token)
        .await
        .map_err(CustomError::ExternalApi)?;
    let tool = client
        .list_tools()
        .await
        .map_err(CustomError::ExternalApi)?
        .into_iter()
        .find(|tool| tool.name == request.name)
        .ok_or(CustomError::Authorization)?;
    let result = client
        .call_tool(&tool.name, request.arguments)
        .await
        .map_err(CustomError::ExternalApi)?;
    Ok(Json(result))
}

async fn authorized_server(
    pool: &Pool,
    current_user: Jwt,
    team_id: &str,
    id: i32,
) -> Result<(McpServerConfig, db::authz::Rbac), CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let sub = current_user.sub.clone();
    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), team_id).await?;
    if !rbac.can_view_integrations() {
        return Err(CustomError::Authorization);
    }
    let row = transaction
        .query_opt(
            "SELECT id, name, server_url, decrypt_text(access_token) AS access_token, \
                    decrypt_text(refresh_token) AS refresh_token, expires_at, token_url, client_id, \
                    decrypt_text(client_secret) AS client_secret \
         FROM integrations.mcp_servers WHERE id = $1 AND team_id = $2 \
         AND (visibility = 'Team' OR created_by = current_app_user()) \
         AND (auth_type <> 'oauth2' OR access_token IS NOT NULL)",
            &[&id, &team_id_num],
        )
        .await?
        .ok_or(CustomError::Authorization)?;
    let mut server = McpServerConfig {
        id: row.get("id"),
        name: row.get("name"),
        server_url: row.get("server_url"),
        bearer_token: row.get("access_token"),
        refresh_token: row.get("refresh_token"),
        expires_at: row.get("expires_at"),
        token_url: row.get("token_url"),
        client_id: row.get("client_id"),
        client_secret: row.get("client_secret"),
    };
    transaction.commit().await?;
    tool_runtime::mcp_client::refresh_expiring_token(pool, &sub, &mut server)
        .await
        .map_err(CustomError::ExternalApi)?;
    Ok((server, rbac))
}

fn valid_asset_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains("..")
        && path
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
}
#[cfg(test)]
mod asset_tests {
    use super::valid_asset_path;
    #[test]
    fn rejects_paths_that_can_escape_the_asset_directory() {
        for path in [
            "",
            "../secret",
            "%2e%2e",
            "/secret",
            "https://example.org/a",
            "a/b",
            "a?b",
            "a\\b",
        ] {
            assert!(!valid_asset_path(path), "{path}");
        }
        assert!(valid_asset_path("cadcraft-web-a123_bg.wasm"));
    }
}
