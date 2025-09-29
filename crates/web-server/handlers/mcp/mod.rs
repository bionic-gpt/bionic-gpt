use crate::CustomError;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use db::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

const API_KEY_CONNECTION_LOOKUP: &str = r#"
    SELECT 1
    FROM integrations i
    JOIN api_key_connections c ON c.integration_id = i.id
    WHERE LOWER(COALESCE(i.definition->'info'->>'x-bionic-slug', i.definition->'info'->>'bionic-slug')) = LOWER($1)
      AND c.external_id = $2
    LIMIT 1
"#;

const OAUTH2_CONNECTION_LOOKUP: &str = r#"
    SELECT 1
    FROM integrations i
    JOIN oauth2_connections c ON c.integration_id = i.id
    WHERE LOWER(COALESCE(i.definition->'info'->>'x-bionic-slug', i.definition->'info'->>'bionic-slug')) = LOWER($1)
      AND c.external_id = $2
    LIMIT 1
"#;

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/mcp/{slug}/{connection_id}")]
pub struct JsonRpcPath {
    pub slug: String,
    pub connection_id: Uuid,
}

pub fn routes() -> Router {
    Router::new().typed_post(handle_json_rpc)
}

pub async fn handle_json_rpc(
    JsonRpcPath {
        slug,
        connection_id,
    }: JsonRpcPath,
    Extension(pool): Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let slug_param = slug.to_ascii_lowercase();

    let mut connection_exists = client
        .query_opt(API_KEY_CONNECTION_LOOKUP, &[&slug_param, &connection_id])
        .await?
        .is_some();

    if !connection_exists {
        connection_exists = client
            .query_opt(OAUTH2_CONNECTION_LOOKUP, &[&slug_param, &connection_id])
            .await?
            .is_some();
    }

    if !connection_exists {
        let response = json!({
            "jsonrpc": payload
                .get("jsonrpc")
                .and_then(|value| value.as_str())
                .unwrap_or("2.0"),
            "id": payload.get("id").cloned().unwrap_or(Value::Null),
            "error": {
                "code": -32004,
                "message": format!(
                    "Unknown MCP connection {} for integration slug {}",
                    connection_id, slug
                ),
            }
        });

        return Ok((StatusCode::NOT_FOUND, Json(response)));
    }

    let response = json!({
        "jsonrpc": payload
            .get("jsonrpc")
            .and_then(|value| value.as_str())
            .unwrap_or("2.0"),
        "id": payload.get("id").cloned().unwrap_or(Value::Null),
        "error": {
            "code": -32601,
            "message": "MCP JSON-RPC handling is not implemented yet",
        }
    });

    Ok((StatusCode::OK, Json(response)))
}
