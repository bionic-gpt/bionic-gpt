use crate::store::{sync_gmail_labels, sync_gmail_threads, World};
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmailComposeRequest {
    to: Vec<String>,
    #[serde(default)]
    cc: Vec<String>,
    #[serde(default)]
    bcc: Vec<String>,
    subject: String,
    body: String,
    thread_id: Option<String>,
}

fn composed_message(request: EmailComposeRequest) -> Value {
    let mut headers = vec![
        json!({"name": "To", "value": request.to.join(", ")}),
        json!({"name": "Subject", "value": request.subject}),
    ];
    if !request.cc.is_empty() {
        headers.push(json!({"name": "Cc", "value": request.cc.join(", ")}));
    }
    if !request.bcc.is_empty() {
        headers.push(json!({"name": "Bcc", "value": request.bcc.join(", ")}));
    }
    let mut message = json!({
        "payload": {
            "mimeType": "text/plain",
            "headers": headers,
            "body": {"data": request.body},
        }
    });
    if let Some(thread_id) = request.thread_id {
        message["threadId"] = Value::String(thread_id);
    }
    message
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
            query
                .q
                .as_ref()
                .is_none_or(|q| matches_message_query(message, q))
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

fn matches_message_query(message: &Value, query: &str) -> bool {
    let tokens = tokenize_query(query);
    let mut position = 0;
    evaluate_or(message, &tokens, &mut position).unwrap_or(true)
}

#[derive(Debug, PartialEq, Eq)]
enum QueryToken {
    Term(String),
    Or,
    LeftParen,
    RightParen,
}

fn tokenize_query(query: &str) -> Vec<QueryToken> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    for character in query.chars() {
        if let Some(delimiter) = quote {
            if character == delimiter {
                quote = None;
            } else {
                current.push(character);
            }
            continue;
        }
        match character {
            '\'' | '"' => quote = Some(character),
            '(' => {
                push_query_term(&mut tokens, &mut current);
                tokens.push(QueryToken::LeftParen);
            }
            ')' => {
                push_query_term(&mut tokens, &mut current);
                tokens.push(QueryToken::RightParen);
            }
            character if character.is_whitespace() => {
                push_query_term(&mut tokens, &mut current);
            }
            _ => current.push(character),
        }
    }
    push_query_term(&mut tokens, &mut current);
    tokens
}

fn push_query_term(tokens: &mut Vec<QueryToken>, current: &mut String) {
    if current.is_empty() {
        return;
    }
    if current.eq_ignore_ascii_case("OR") {
        tokens.push(QueryToken::Or);
    } else {
        tokens.push(QueryToken::Term(std::mem::take(current)));
        return;
    }
    current.clear();
}

fn evaluate_or(message: &Value, tokens: &[QueryToken], position: &mut usize) -> Option<bool> {
    let mut result = None;
    while *position < tokens.len() && tokens[*position] != QueryToken::RightParen {
        if tokens[*position] == QueryToken::Or {
            *position += 1;
            continue;
        }
        if let Some(value) = evaluate_and(message, tokens, position) {
            result = Some(result.unwrap_or(false) || value);
        }
        if tokens.get(*position) == Some(&QueryToken::Or) {
            *position += 1;
        }
    }
    result
}

fn evaluate_and(message: &Value, tokens: &[QueryToken], position: &mut usize) -> Option<bool> {
    let mut result = None;
    while *position < tokens.len()
        && tokens[*position] != QueryToken::Or
        && tokens[*position] != QueryToken::RightParen
    {
        let value = match &tokens[*position] {
            QueryToken::LeftParen => {
                *position += 1;
                let value = evaluate_or(message, tokens, position).unwrap_or(true);
                if tokens.get(*position) == Some(&QueryToken::RightParen) {
                    *position += 1;
                }
                value
            }
            QueryToken::Term(term) => {
                *position += 1;
                matches_query_term(message, term)
            }
            QueryToken::Or | QueryToken::RightParen => break,
        };
        result = Some(result.unwrap_or(true) && value);
    }
    result
}

fn matches_query_term(message: &Value, term: &str) -> bool {
    let (negated, term) = term
        .strip_prefix('-')
        .map_or((false, term), |term| (true, term));
    let matched = term.split_once(':').map_or_else(
        || {
            message
                .to_string()
                .to_lowercase()
                .contains(&term.to_lowercase())
        },
        |(operator, value)| match operator.to_ascii_lowercase().as_str() {
            "subject" => header_contains(message, "Subject", value),
            "from" => header_contains(message, "From", value),
            "to" => header_contains(message, "To", value),
            "label" => message["labelIds"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|label| {
                    label
                        .as_str()
                        .is_some_and(|label| label.eq_ignore_ascii_case(value))
                }),
            _ => message
                .to_string()
                .to_lowercase()
                .contains(&term.to_lowercase()),
        },
    );
    matched != negated
}

fn header_contains(message: &Value, name: &str, needle: &str) -> bool {
    message["payload"]["headers"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|header| {
            header["name"]
                .as_str()
                .is_some_and(|header_name| header_name.eq_ignore_ascii_case(name))
                && header["value"]
                    .as_str()
                    .is_some_and(|value| value.to_lowercase().contains(&needle.to_lowercase()))
        })
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
    let mut state = world.write().await;
    if state.gmail.messages.remove(&id).is_some() {
        sync_gmail_threads(&mut state.gmail);
        sync_gmail_labels(&mut state.gmail);
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
    let mut state = world.write().await;
    sync_gmail_labels(&mut state.gmail);
    Ok(Json(json!({
        "labels": state.gmail.labels.values().cloned().collect::<Vec<_>>()
    })))
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
        fields.insert("messagesTotal".into(), 0.into());
        fields.insert("messagesUnread".into(), 0.into());
        fields.insert("threadsTotal".into(), 0.into());
        fields.insert("threadsUnread".into(), 0.into());
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
    let mut state = world.write().await;
    if state.gmail.labels.remove(&id).is_some() {
        for message in state.gmail.messages.values_mut() {
            if let Some(labels) = message.get_mut("labelIds").and_then(Value::as_array_mut) {
                labels.retain(|label| label != &id);
            }
        }
        for message in state
            .gmail
            .drafts
            .values_mut()
            .filter_map(|draft| draft.get_mut("message"))
        {
            if let Some(labels) = message.get_mut("labelIds").and_then(Value::as_array_mut) {
                labels.retain(|label| label != &id);
            }
        }
        sync_gmail_labels(&mut state.gmail);
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
                messages
                    .iter()
                    .any(|message| matches_message_query(message, q))
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
        sync_gmail_threads(&mut state.gmail);
        sync_gmail_labels(&mut state.gmail);
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
    sync_gmail_labels(&mut state.gmail);
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
    sync_gmail_labels(&mut state.gmail);
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
    sync_gmail_labels(&mut state.gmail);
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
    sync_gmail_labels(&mut state.gmail);
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
            query.q.as_ref().is_none_or(|q| {
                draft
                    .get("message")
                    .is_some_and(|message| matches_message_query(message, q))
            })
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
    Json(request): Json<EmailComposeRequest>,
) -> Json<Value> {
    let mut state = world.write().await;
    let id = format!("draft-{}", state.gmail.next_id);
    state.gmail.next_id += 1;
    let mut message = composed_message(request);
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
    sync_gmail_labels(&mut state.gmail);
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
    Json(request): Json<EmailComposeRequest>,
) -> Result<Json<Value>, StatusCode> {
    let mut state = world.write().await;
    let Some(draft) = state.gmail.drafts.get_mut(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    let mut message = composed_message(request);
    message["id"] = draft["message"]["id"].clone();
    message["threadId"] = draft["message"]["threadId"].clone();
    message["labelIds"] = json!(["DRAFT"]);
    draft["message"] = message;
    let result = draft.clone();
    sync_gmail_labels(&mut state.gmail);
    Ok(Json(result))
}
async fn delete_draft(
    State(world): State<Arc<World>>,
    Path((_user_id, id)): Path<(String, String)>,
) -> StatusCode {
    let mut state = world.write().await;
    if state.gmail.drafts.remove(&id).is_some() {
        sync_gmail_labels(&mut state.gmail);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
async fn send_message(
    State(world): State<Arc<World>>,
    Path(_user_id): Path<String>,
    Json(request): Json<EmailComposeRequest>,
) -> Json<Value> {
    let mut state = world.write().await;
    let id = format!("msg-sent-{:03}", state.gmail.next_id);
    state.gmail.next_id += 1;
    let mut payload = composed_message(request);
    let thread_id = payload
        .get("threadId")
        .and_then(Value::as_str)
        .unwrap_or("thread-sent-001")
        .to_string();
    if let Some(value) = payload.as_object_mut() {
        value.remove("threadId");
    }
    let snippet = payload
        .pointer("/payload/body/data")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let message = json!({"id":id,"threadId":thread_id,"labelIds":["SENT"],"snippet":snippet,"payload":payload["payload"]});
    state.gmail.messages.insert(id, message.clone());
    sync_gmail_threads(&mut state.gmail);
    sync_gmail_labels(&mut state.gmail);
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
    let mut message = draft["message"].clone();
    message["id"] = Value::String(message_id.clone());
    message["labelIds"] = json!(["SENT"]);
    message["snippet"] = Value::String(
        message
            .pointer("/payload/body/data")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
    );
    state.gmail.messages.insert(message_id, message.clone());
    sync_gmail_threads(&mut state.gmail);
    sync_gmail_labels(&mut state.gmail);
    Ok(Json(message))
}
