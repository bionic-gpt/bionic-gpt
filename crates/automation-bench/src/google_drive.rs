use crate::store::World;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn routes() -> Router<Arc<World>> {
    Router::new()
        .route("/drive/v3/files", get(list_files))
        .route("/drive/v3/files/{file_id}", get(get_file))
}

#[derive(Deserialize, Default)]
struct ListQuery {
    q: Option<String>,
    #[serde(rename = "pageSize")]
    page_size: Option<usize>,
    #[serde(rename = "pageToken")]
    page_token: Option<String>,
}

async fn list_files(
    State(world): State<Arc<World>>,
    Query(query): Query<ListQuery>,
) -> Json<Value> {
    let state = world.read().await;
    let mut files = state
        .google_drive
        .files
        .values()
        .filter(|file| query.q.as_ref().is_none_or(|q| matches_query(file, q)))
        .cloned()
        .collect::<Vec<_>>();
    let offset = query
        .page_token
        .as_deref()
        .and_then(|token| token.strip_prefix("page-"))
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);
    let page_size = query.page_size.unwrap_or(10).min(1_000);
    let total = files.len();
    files = files.into_iter().skip(offset).take(page_size).collect();
    let next_page_token =
        (offset + files.len() < total).then(|| format!("page-{}", offset + files.len()));

    Json(json!({
        "kind": "drive#fileList",
        "files": files,
        "nextPageToken": next_page_token,
        "incompleteSearch": false,
    }))
}

fn matches_query(file: &Value, query: &str) -> bool {
    let lower = query.to_lowercase();
    if let Some(start) = lower.find("name contains '") {
        let value = &query[start + "name contains '".len()..];
        if let Some(end) = value.find('\'') {
            return file["name"]
                .as_str()
                .is_some_and(|name| name.to_lowercase().contains(&value[..end].to_lowercase()));
        }
    }
    if let Some(start) = lower.find("mimetype = '") {
        let value = &query[start + "mimetype = '".len()..];
        if let Some(end) = value.find('\'') {
            return file["mimeType"].as_str() == Some(&value[..end]);
        }
    }
    file.to_string().to_lowercase().contains(&lower)
}

async fn get_file(
    State(world): State<Arc<World>>,
    Path(file_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    world
        .read()
        .await
        .google_drive
        .files
        .get(&file_id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
