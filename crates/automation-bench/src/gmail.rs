use crate::store::{sync_gmail_threads, World};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use axum_extra::extract::Query;
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
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
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
    let messages: Vec<Value> = state
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
    let offset = page_offset(query.page_token.as_deref());
    let page_size = query.max_results.unwrap_or(100).min(500);
    let result_size = messages.len();
    let page = messages
        .into_iter()
        .skip(offset)
        .take(page_size)
        .collect::<Vec<_>>();
    let next_page_token =
        (offset + page.len() < result_size).then(|| format!("page-{}", offset + page.len()));
    Ok(Json(
        json!({"messages":page,"nextPageToken":next_page_token,"resultSizeEstimate":result_size}),
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
    let mut state = world.write().await;
    let id = format!("Label_{}", state.gmail.next_id);
    state.gmail.next_id += 1;
    let mut label = body;
    if let Value::Object(fields) = &mut label {
        fields.insert("id".into(), id.clone().into());
        fields.insert("type".into(), "user".into());
    }
    state.gmail.labels.insert(id, label.clone());
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
    Query(query): Query<ListQuery>,
) -> Result<Json<Value>, StatusCode> {
    if !user_ok(&user_id) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut state = world.write().await;
    sync_gmail_threads(&mut state.gmail);
    let mut threads = state
        .gmail
        .threads
        .values()
        .filter(|thread| {
            let messages = thread["messages"].as_array().cloned().unwrap_or_default();
            query.q.as_ref().is_none_or(|q| {
                thread
                    .to_string()
                    .to_lowercase()
                    .contains(&q.to_lowercase())
            }) && query.label_ids.as_ref().is_none_or(|labels| {
                labels.iter().all(|label| {
                    messages.iter().any(|message| {
                        message["labelIds"]
                            .as_array()
                            .is_some_and(|ids| ids.iter().any(|id| id == label))
                    })
                })
            }) && (query.include_spam_trash == Some(true)
                || !messages.iter().any(|message| {
                    message["labelIds"]
                        .as_array()
                        .is_some_and(|ids| ids.iter().any(|id| id == "SPAM" || id == "TRASH"))
                }))
        })
        .map(|thread| json!({"id":thread["id"],"snippet":thread["snippet"]}))
        .collect::<Vec<_>>();
    let result_size = threads.len();
    let offset = page_offset(query.page_token.as_deref());
    let page_size = query.max_results.unwrap_or(100).min(100);
    threads = threads.into_iter().skip(offset).take(page_size).collect();
    let next_page_token =
        (offset + threads.len() < result_size).then(|| format!("page-{}", offset + threads.len()));
    Ok(Json(
        json!({"threads":threads,"nextPageToken":next_page_token,"resultSizeEstimate":result_size}),
    ))
}

fn page_offset(token: Option<&str>) -> usize {
    token
        .and_then(|token| token.strip_prefix("page-"))
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0)
}
async fn get_thread(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    sync_gmail_threads(&mut state.gmail);
    state
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
    let mut state = world.write().await;
    sync_gmail_threads(&mut state.gmail);
    if state.gmail.threads.contains_key(&id) {
        state.gmail.threads.remove(&id);
        state
            .gmail
            .messages
            .retain(|_, message| message["threadId"] != id);
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
    let result = message.clone();
    sync_gmail_threads(&mut state.gmail);
    Ok(Json(result))
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
    let result = message.clone();
    sync_gmail_threads(&mut state.gmail);
    Ok(Json(result))
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
    let result = message.clone();
    sync_gmail_threads(&mut state.gmail);
    Ok(Json(result))
}
async fn modify_thread(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    sync_gmail_threads(&mut state.gmail);
    let message_ids = state
        .gmail
        .threads
        .get(&id)
        .and_then(|thread| thread["messages"].as_array())
        .map(|messages| {
            messages
                .iter()
                .filter_map(|message| message["id"].as_str().map(str::to_string))
                .collect::<Vec<_>>()
        })
        .ok_or(StatusCode::NOT_FOUND)?;
    for message_id in message_ids {
        if let Some(message) = state.gmail.messages.get_mut(&message_id) {
            change_labels(message, &body, None);
        }
    }
    sync_gmail_threads(&mut state.gmail);
    let Some(thread) = state.gmail.threads.get(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
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

async fn list_drafts(
    State(world): State<Arc<World>>,
    Path(_user_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Json<Value> {
    let drafts = world
        .read()
        .await
        .gmail
        .drafts
        .values()
        .filter(|draft| {
            query
                .q
                .as_ref()
                .is_none_or(|q| draft.to_string().to_lowercase().contains(&q.to_lowercase()))
        })
        .map(|draft| json!({"id":draft["id"],"message":draft["message"]}))
        .collect::<Vec<_>>();
    let result_size = drafts.len();
    let offset = page_offset(query.page_token.as_deref());
    let page_size = query.max_results.unwrap_or(100).min(100);
    let page = drafts
        .into_iter()
        .skip(offset)
        .take(page_size)
        .collect::<Vec<_>>();
    let next_page_token =
        (offset + page.len() < result_size).then(|| format!("page-{}", offset + page.len()));
    Json(json!({"drafts":page,"nextPageToken":next_page_token,"resultSizeEstimate":result_size}))
}
async fn create_draft(
    State(world): State<Arc<World>>,
    Path(_user_id): Path<String>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let mut state = world.write().await;
    let id = format!("draft-{}", state.gmail.next_id);
    state.gmail.next_id += 1;
    let mut message = body.get("message").cloned().unwrap_or(body);
    if let Value::Object(fields) = &mut message {
        fields
            .entry("id")
            .or_insert_with(|| Value::String(format!("draft-message-{id}")));
        fields
            .entry("threadId")
            .or_insert_with(|| Value::String(format!("thread-{id}")));
        fields.entry("labelIds").or_insert_with(|| json!(["DRAFT"]));
    }
    let draft = json!({"id":id,"message":message});
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
async fn send_message(
    State(world): State<Arc<World>>,
    Path(_user_id): Path<String>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let mut state = world.write().await;
    let id = format!("msg-sent-{:03}", state.gmail.next_id);
    state.gmail.next_id += 1;
    let thread_id = body
        .get("threadId")
        .and_then(Value::as_str)
        .unwrap_or("thread-sent-001")
        .to_string();
    let message = json!({"id":id,"threadId":thread_id,"labelIds":["SENT"],"snippet":"Sent message","payload":body});
    state.gmail.messages.insert(id, message.clone());
    sync_gmail_threads(&mut state.gmail);
    Json(message)
}
async fn send_draft(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    let Some(draft) = state.gmail.drafts.remove(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    let message_id = format!("msg-sent-{:03}", state.gmail.next_id);
    state.gmail.next_id += 1;
    let thread_id = draft["message"]["threadId"]
        .as_str()
        .unwrap_or("thread-sent-001");
    let message = json!({"id":message_id,"threadId":thread_id,"labelIds":["SENT"],"snippet":"Sent draft","payload":draft["message"]});
    state.gmail.messages.insert(message_id, message.clone());
    sync_gmail_threads(&mut state.gmail);
    Ok(Json(message))
}
