use crate::store::World;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn routes() -> Router<Arc<World>> {
    Router::new()
        .route("/gmail/v1/users/{user_id}/messages", get(list_messages))
        .route(
            "/gmail/v1/users/{user_id}/messages/{id}",
            get(get_message).delete(delete_message),
        )
        .route(
            "/gmail/v1/users/{user_id}/labels",
            get(list_labels).post(create_label),
        )
        .route(
            "/gmail/v1/users/{user_id}/labels/{id}",
            get(get_label).put(update_label).delete(delete_label),
        )
        .route("/gmail/v1/users/{user_id}/threads", get(list_threads))
        .route(
            "/gmail/v1/users/{user_id}/threads/{id}",
            get(get_thread).delete(delete_thread),
        )
        .route(
            "/gmail/v1/users/{user_id}/messages/{id}/modify",
            post(modify_message),
        )
        .route(
            "/gmail/v1/users/{user_id}/messages/{id}/trash",
            post(trash_message),
        )
        .route(
            "/gmail/v1/users/{user_id}/messages/{id}/untrash",
            post(untrash_message),
        )
        .route(
            "/gmail/v1/users/{user_id}/threads/{id}/modify",
            post(modify_thread),
        )
        .route(
            "/gmail/v1/users/{user_id}/threads/{id}/trash",
            post(trash_thread),
        )
        .route(
            "/gmail/v1/users/{user_id}/threads/{id}/untrash",
            post(untrash_thread),
        )
        .route(
            "/gmail/v1/users/{user_id}/drafts",
            get(list_drafts).post(create_draft),
        )
        .route(
            "/gmail/v1/users/{user_id}/drafts/{id}",
            get(get_draft).put(update_draft).delete(delete_draft),
        )
        .route(
            "/gmail/v1/users/{user_id}/messages/send",
            post(send_message),
        )
        .route("/gmail/v1/users/{user_id}/drafts/send", post(send_draft))
}

#[derive(Deserialize, Default)]
pub struct ListQuery {
    pub q: Option<String>,
    #[serde(rename = "labelIds")]
    pub label_ids: Option<Vec<String>>,
    #[serde(rename = "maxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "includeSpamTrash")]
    pub include_spam_trash: Option<bool>,
}

fn user_ok(user_id: &str) -> bool {
    user_id == "me" || !user_id.is_empty()
}

async fn list_messages(
    State(world): State<Arc<World>>,
    Path(user_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Value>, StatusCode> {
    if !user_ok(&user_id) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let state = world.read().await;
    let mut messages: Vec<Value> = state
        .gmail
        .messages
        .values()
        .filter(|message| {
            let labels = message.get("labelIds").and_then(Value::as_array);
            if query.include_spam_trash != Some(true)
                && labels.is_some_and(|labels| {
                    labels
                        .iter()
                        .any(|label| label == "TRASH" || label == "SPAM")
                })
            {
                return false;
            }
            if let Some(required) = &query.label_ids {
                if !required.iter().all(|label| {
                    labels.is_some_and(|labels| labels.iter().any(|item| item == label))
                }) {
                    return false;
                }
            }
            query.q.as_ref().is_none_or(|q| {
                message
                    .to_string()
                    .to_lowercase()
                    .contains(&q.to_lowercase())
            })
        })
        .map(|message| json!({"id":message["id"],"threadId":message["threadId"]}))
        .collect();
    messages.truncate(query.max_results.unwrap_or(100).min(500));
    Ok(Json(
        json!({"messages":messages,"resultSizeEstimate":messages.len()}),
    ))
}

async fn get_message(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    world
        .read()
        .await
        .gmail
        .messages
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
async fn delete_message(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> StatusCode {
    if world.write().await.gmail.messages.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn list_labels(
    State(world): State<Arc<World>>,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    if !user_ok(&user_id) {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(
        json!({"labels":world.read().await.gmail.labels.values().cloned().collect::<Vec<_>>()}),
    ))
}
async fn create_label(
    State(world): State<Arc<World>>,
    Path(_user_id): Path<String>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let id = format!("Label_{}", world.write().await.gmail.next_id);
    let mut label = body;
    if let Value::Object(fields) = &mut label {
        fields.insert("id".into(), id.clone().into());
        fields.insert("type".into(), "user".into());
    }
    world.write().await.gmail.labels.insert(id, label.clone());
    Json(label)
}
async fn get_label(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    world
        .read()
        .await
        .gmail
        .labels
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
async fn update_label(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    let Some(existing) = state.gmail.labels.get_mut(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    *existing = body;
    Ok(Json(existing.clone()))
}
async fn delete_label(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> StatusCode {
    if matches!(id.as_str(), "INBOX" | "SENT" | "DRAFT" | "TRASH") {
        return StatusCode::BAD_REQUEST;
    }
    if world.write().await.gmail.labels.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn list_threads(
    State(world): State<Arc<World>>,
    Path(user_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Result<Json<Value>, StatusCode> {
    if !user_ok(&user_id) {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(
        json!({"threads":world.read().await.gmail.threads.values().map(|thread| json!({"id":thread["id"],"snippet":thread["snippet"]})).collect::<Vec<_>>()}),
    ))
}
async fn get_thread(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    world
        .read()
        .await
        .gmail
        .threads
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
async fn delete_thread(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> StatusCode {
    if world.write().await.gmail.threads.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

fn change_labels(message: &mut Value, body: &Value, trash: Option<bool>) {
    if let Some(labels) = message.get_mut("labelIds").and_then(Value::as_array_mut) {
        if let Some(add) = body.get("addLabelIds").and_then(Value::as_array) {
            for label in add {
                if !labels.contains(label) {
                    labels.push(label.clone());
                }
            }
        }
        if let Some(remove) = body.get("removeLabelIds").and_then(Value::as_array) {
            labels.retain(|label| !remove.contains(label));
        }
        if let Some(trash) = trash {
            labels.retain(|label| label != "TRASH");
            if trash {
                labels.push("TRASH".into());
            }
        }
    }
}
async fn modify_message(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    let Some(message) = state.gmail.messages.get_mut(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    change_labels(message, &body, None);
    Ok(Json(message.clone()))
}
async fn trash_message(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    let Some(message) = state.gmail.messages.get_mut(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    change_labels(message, &json!({}), Some(true));
    Ok(Json(message.clone()))
}
async fn untrash_message(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    let Some(message) = state.gmail.messages.get_mut(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    change_labels(message, &json!({}), Some(false));
    Ok(Json(message.clone()))
}
async fn modify_thread(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    let Some(thread) = state.gmail.threads.get_mut(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    if let Some(messages) = thread.get_mut("messages").and_then(Value::as_array_mut) {
        for message in messages {
            change_labels(message, &body, None);
        }
    }
    Ok(Json(thread.clone()))
}
async fn trash_thread(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    modify_thread(
        State(world),
        Path((String::new(), id)),
        Json(json!({"addLabelIds":["TRASH"]})),
    )
    .await
}
async fn untrash_thread(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    modify_thread(
        State(world),
        Path((String::new(), id)),
        Json(json!({"removeLabelIds":["TRASH"]})),
    )
    .await
}

async fn list_drafts(State(world): State<Arc<World>>, Path(_user_id): Path<String>) -> Json<Value> {
    Json(
        json!({"drafts":world.read().await.gmail.drafts.values().map(|draft| json!({"id":draft["id"],"message":draft["message"]})).collect::<Vec<_>>()}),
    )
}
async fn create_draft(
    State(world): State<Arc<World>>,
    Path(_user_id): Path<String>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let mut state = world.write().await;
    let id = format!("draft-{}", state.gmail.next_id);
    state.gmail.next_id += 1;
    let draft = json!({"id":id,"message":body.get("message").cloned().unwrap_or(body)});
    state.gmail.drafts.insert(id, draft.clone());
    Json(draft)
}
async fn get_draft(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    world
        .read()
        .await
        .gmail
        .drafts
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
async fn update_draft(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    let Some(draft) = state.gmail.drafts.get_mut(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    draft["message"] = body.get("message").cloned().unwrap_or(body);
    Ok(Json(draft.clone()))
}
async fn delete_draft(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> StatusCode {
    if world.write().await.gmail.drafts.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
async fn send_message(Json(body): Json<Value>) -> Json<Value> {
    Json(
        json!({"id":"sent-message-001","threadId":body.get("threadId").cloned().unwrap_or(Value::String("thread-sent-001".into())),"labelIds":["SENT"],"payload":body}),
    )
}
async fn send_draft(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    let Some(draft) = state.gmail.drafts.remove(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(
        json!({"id":format!("sent-{id}"),"labelIds":["SENT"],"payload":draft["message"]}),
    ))
}
