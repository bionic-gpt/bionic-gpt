//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```
use super::stream_assembler::{enriched_chat, EnrichedChatOutcome, GenerationEvent};
use super::stream_errors::error_to_chat;
use crate::chat_converter;
use crate::errors::CustomError;
use crate::jwt::Jwt;
use crate::moderation::{moderate_chat, strip_tool_data, ModerationVerdict};
use crate::user_config::UserConfig;
use async_trait::async_trait;
use axum::response::{sse::Event, Sse};
use axum::Extension;
use db::{queries, Pool};
use db::{ChatRole, ChatStatus};
use openai_api::{BionicChatCompletionRequest, ToolCall};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    RequestBuilder,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tool_runtime::{
    execute_tool_calls, get_chat_tools_user_selected_with_system_openapi, get_tools, ToolScope,
};

use super::{limits, UICompletions};

/// UI SSE contract for `/completions/{chat_id}`.
///
/// Event payloads are JSON encoded strings with this shape:
/// - `{"type":"text_delta","data":{"delta":"..."}}`
/// - `{"type":"done","data":{}}`
/// - `{"type":"error","data":{"message":"..."}}`
///
/// The web client appends `text_delta` values to a local snapshot, finalizes UI on `done`,
/// and shows an error block on `error`.
fn event_data_for_text(delta: String) -> String {
    let content_delta = serde_json::from_str::<serde_json::Value>(&delta)
        .ok()
        .and_then(|value| {
            value
                .get("choices")
                .and_then(|choices| choices.get(0))
                .and_then(|choice| choice.get("delta"))
                .and_then(|choice_delta| choice_delta.get("content"))
                .and_then(|content| content.as_str())
                .map(|content| content.to_string())
        })
        .unwrap_or_default();

    json!({
        "type": "text_delta",
        "data": {
            "delta": content_delta
        }
    })
    .to_string()
}

fn event_data_for_done() -> String {
    json!({
        "type": "done",
        "data": {}
    })
    .to_string()
}

fn event_data_for_error(message: String) -> Event {
    Event::default().data(
        json!({
            "type": "error",
            "data": {
                "message": message
            }
        })
        .to_string(),
    )
}

#[async_trait]
pub(crate) trait ResultSink: Send + Sync {
    async fn save(
        &self,
        snapshot: &str,
        tool_calls: Option<Vec<ToolCall>>,
        chat_id: i32,
        sub: &str,
        status: ChatStatus,
    );
}

struct DbResultSink {
    pool: Pool,
}

#[async_trait]
impl ResultSink for DbResultSink {
    async fn save(
        &self,
        snapshot: &str,
        tool_calls: Option<Vec<ToolCall>>,
        chat_id: i32,
        sub: &str,
        status: ChatStatus,
    ) {
        save_results_db(&self.pool, snapshot, tool_calls, chat_id, sub, status).await;
    }
}

fn extract_tool_calls(
    completion_chunk: &super::stream_assembler::CompletionChunk,
) -> Option<Vec<ToolCall>> {
    completion_chunk
        .merged
        .as_ref()
        .and_then(|merged| merged.choices.first())
        .and_then(|choice| choice.delta.tool_calls.clone())
}

fn extract_tool_calls_from_merged(
    merged: &Option<openai_api::ChatCompletionDelta>,
) -> Option<Vec<ToolCall>> {
    merged
        .as_ref()
        .and_then(|merged| merged.choices.first())
        .and_then(|choice| choice.delta.tool_calls.clone())
}

pub(crate) fn build_event_stream<S>(
    receiver_stream: S,
    result_sink: Arc<dyn ResultSink>,
    chat_id: i32,
    sub: Arc<String>,
) -> impl tokio_stream::Stream<Item = Result<Event, axum::Error>>
where
    S: tokio_stream::Stream<Item = Result<GenerationEvent, axum::Error>> + Send + 'static,
{
    receiver_stream.then(move |item| {
        let result_sink = Arc::clone(&result_sink);
        let sub = Arc::clone(&sub);
        async move {
            match item {
                Ok(event) => match event {
                    GenerationEvent::Text(completion_chunk) => {
                        Ok(Event::default().data(event_data_for_text(completion_chunk.delta)))
                    }
                    GenerationEvent::End(completion_chunk) => {
                        let tool_calls = extract_tool_calls(&completion_chunk);

                        tracing::debug!("End of stream saving data");
                        result_sink
                            .save(
                                &completion_chunk.snapshot,
                                tool_calls,
                                chat_id,
                                &sub,
                                ChatStatus::Success,
                            )
                            .await;

                        Ok(Event::default().data(event_data_for_done()))
                    }
                },
                Err(e) => {
                    let message = e.to_string();
                    result_sink
                        .save(&message, None, chat_id, &sub, ChatStatus::Error)
                        .await;
                    Ok(event_data_for_error(message))
                }
            }
        }
    })
}

// Called by the web console UI to stream a chat completion.
pub async fn chat_generate(
    UICompletions { chat_id }: UICompletions,
    current_user: Jwt,
    user_config: UserConfig,
    Extension(pool): Extension<Pool>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>>, CustomError> {
    let result_sink: Arc<dyn ResultSink> = Arc::new(DbResultSink { pool: pool.clone() });

    match create_request(&pool, &current_user, chat_id, &user_config).await {
        Ok((request, model_id, user_id)) => {
            let is_limit_breached =
                limits::is_limit_exceeded_from_pool(&pool, model_id, user_id).await?;

            // Create a channel for sending SSE events
            let (sender, receiver) = mpsc::channel::<Result<GenerationEvent, axum::Error>>(10);
            let result_sink_clone = Arc::clone(&result_sink);
            let sub_for_save = current_user.sub.clone();

            // Generate provider events in the background and forward to SSE.
            tokio::spawn(async move {
                if is_limit_breached {
                    let limit_message = "You have exceeded your token limit for this model";
                    let send_error = error_to_chat(limit_message, sender)
                        .await
                        .err()
                        .map(|e| e.to_string());
                    if let Some(err_msg) = send_error {
                        tracing::warn!("Limits exceeded: {}", err_msg);
                        result_sink_clone
                            .save(
                                limit_message,
                                None,
                                chat_id,
                                &sub_for_save,
                                ChatStatus::Error,
                            )
                            .await;
                    }
                } else {
                    let stream_outcome = enriched_chat(request, sender, true)
                        .await
                        .map_err(|e| e.to_string());
                    match stream_outcome {
                        Ok(EnrichedChatOutcome::Completed) => {}
                        Ok(EnrichedChatOutcome::ClientDisconnected { snapshot, merged }) => {
                            let tool_calls = extract_tool_calls_from_merged(&merged);
                            result_sink_clone
                                .save(
                                    &snapshot,
                                    tool_calls,
                                    chat_id,
                                    &sub_for_save,
                                    ChatStatus::Error,
                                )
                                .await;
                        }
                        Err(err_msg) => {
                            tracing::error!("Error generating SSE stream: {}", err_msg);
                            result_sink_clone
                                .save(&err_msg, None, chat_id, &sub_for_save, ChatStatus::Error)
                                .await;
                        }
                    }
                }
            });

            let sub_arc = Arc::new(current_user.sub.clone());
            let receiver_stream = ReceiverStream::new(receiver);

            let event_stream =
                build_event_stream(receiver_stream, Arc::clone(&result_sink), chat_id, sub_arc);

            Ok(Sse::new(event_stream))
        }
        Err(err) => {
            result_sink
                .save(
                    &err.to_string(),
                    None,
                    chat_id,
                    &current_user.sub,
                    ChatStatus::Error,
                )
                .await;
            Err(CustomError::FaultySetup(err.to_string()))
        }
    }
}

// When the chat has completed, store the results in the database.
async fn save_results_db(
    pool: &Pool,
    snapshot: &str,
    tool_calls: Option<Vec<ToolCall>>,
    chat_id: i32,
    sub: &str,
    status: ChatStatus,
) {
    let mut db_client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            tracing::error!("Error getting database client: {:?}", e);
            return;
        }
    };

    tracing::debug!("Got a client from the pool");

    let transaction = match db_client.transaction().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Error starting transaction: {:?}", e);
            return;
        }
    };

    tracing::debug!("Starting the transaction");

    if let Err(e) = db::authz::set_row_level_security_user_id(&transaction, sub.to_string()).await {
        tracing::error!("Error setting row level security: {:?}", e);
        return;
    }

    tracing::debug!("Setting RLS ID");

    // Set the chat status to InProgress
    if let Err(e) = queries::chats::set_chat_status()
        .bind(&transaction, &status, &chat_id)
        .await
    {
        tracing::error!("Error updating chat status: {:?}", e);
        return;
    }

    tracing::debug!("About to create the assistants response");

    let tool_calls_json = serde_json::to_string(&tool_calls).ok();

    // Calculate completion tokens from the response
    let completion_tokens = openai_api::token_count_from_string(snapshot);

    tracing::debug!(
        "save_results: Executing chat query with chat_id: {}",
        chat_id
    );
    if let Ok(chat) = queries::chats::chat()
        .bind(&transaction, &chat_id)
        .one()
        .await
    {
        if status == ChatStatus::Success {
            // Mark only pending tool rows from prior iterations as completed.
            if let Err(e) = transaction
                .execute(
                    "UPDATE llm.chats
                     SET status = 'Success'
                     WHERE status = 'Pending'
                     AND conversation_id = $1
                     AND role = 'Tool'",
                    &[&chat.conversation_id],
                )
                .await
            {
                tracing::error!("Error updating pending tool chats: {:?}", e);
                return;
            }
        }

        if let Err(e) = queries::chats::new_chat()
            .bind(
                &transaction,
                &chat.conversation_id,
                &chat.prompt_id,
                &None::<String>,
                &tool_calls_json,
                &snapshot,
                &ChatRole::Assistant,
                &status,
            )
            .one()
            .await
        {
            tracing::error!("Error creating chat: {:?}", e);
            return;
        }

        // Track completion token usage in token_usage_metrics
        if let Err(e) = queries::token_usage_metrics::create_token_usage_metric()
            .bind(
                &transaction,
                &Some(chat_id),
                &None::<i32>, // api_key_id
                &db::TokenUsageType::Completion,
                &completion_tokens,
                &None::<i32>, // duration_ms - could add timing here later
            )
            .one()
            .await
        {
            tracing::error!("Error tracking completion tokens: {:?}", e);
            // Don't return here, continue with the rest of the function
        }

        if status == ChatStatus::Success {
            if let Some(tool_calls) = tool_calls {
                let tool_call_results = execute_tool_calls(
                    tool_calls.clone(),
                    pool,
                    sub.to_string(),
                    chat.conversation_id,
                    chat.prompt_id,
                )
                .await;
                for tool_call in tool_call_results {
                    let result_json = match serde_json::to_string(&tool_call.result) {
                        Ok(json) => json,
                        Err(e) => {
                            tracing::error!("Failed to serialize tool result: {:?}", e);
                            return;
                        }
                    };
                    let tool_chat_status = if tool_call.result.get("error").is_some() {
                        ChatStatus::Error
                    } else {
                        ChatStatus::Pending
                    };

                    if let Err(e) = queries::chats::new_chat()
                        .bind(
                            &transaction,
                            &chat.conversation_id,
                            &chat.prompt_id,
                            &Some(tool_call.id),
                            &None::<String>,
                            &result_json,
                            &ChatRole::Tool,
                            &tool_chat_status,
                        )
                        .one()
                        .await
                    {
                        tracing::error!("Error creating tool call results chat: {:?}", e);
                        return;
                    }
                }
            }
        }
    } else {
        tracing::error!("Error retrieving chat");
    }

    if let Err(e) = transaction.commit().await {
        tracing::error!("Error committing transaction: {:?}", e);
    }

    tracing::debug!("Successfully completed save_results");
}

// Create the request that we'll send to reqwest to create an SSE stream of incoming
// chat completions.
async fn create_request(
    pool: &Pool,
    current_user: &Jwt,
    chat_id: i32,
    user_config: &UserConfig,
) -> Result<(RequestBuilder, i32, i32), CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;
    db::authz::set_row_level_security_user_id(&transaction, current_user.sub.to_string()).await?;

    tracing::debug!(
        "Executing model_host_by_chat_id query with chat_id: {}",
        chat_id
    );
    let model = queries::models::model_host_by_chat_id()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    tracing::debug!(
        "Executing get_model_capabilities query with model_id: {}",
        model.id
    );
    let capabilities = queries::capabilities::get_model_capabilities()
        .bind(&transaction, &model.id)
        .all()
        .await?;

    tracing::debug!("Executing chat query with chat_id: {}", chat_id);
    let chat = queries::chats::chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    tracing::debug!(
        "Executing get_conversation_from_chat query with chat_id: {}",
        chat_id
    );
    let conversation = queries::conversations::get_conversation_from_chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    tracing::debug!(
        "Executing count_attachments query with conversation_id: {}",
        conversation.id
    );
    let attachment_count = queries::conversations::count_attachments()
        .bind(&transaction, &conversation.id)
        .one()
        .await?;

    tracing::debug!(
        "Executing prompt query with prompt_id: {}, team_id: {}",
        chat.prompt_id,
        conversation.team_id
    );
    let prompt = queries::prompts::prompt()
        .bind(&transaction, &chat.prompt_id, &conversation.team_id)
        .one()
        .await?;
    // Get the maximum required amount of chat history
    let chat_history = queries::chats::chat_history()
        .bind(
            &transaction,
            &conversation.id,
            &(prompt.max_history_items as i64),
        )
        .all()
        .await?;

    tracing::debug!("{:?}", &chat_history);

    let chat_history = chat_converter::convert_chat_to_messages(chat_history);

    let messages = super::context_builder::execute_prompt(
        &transaction,
        prompt.clone(),
        Some(conversation.id),
        chat_history,
    )
    .await?;

    let size = openai_api::token_count(messages.clone());

    // Track prompt tokens in the new token_usage_metrics table
    queries::token_usage_metrics::create_token_usage_metric()
        .bind(
            &transaction,
            &Some(chat_id),
            &None::<i32>, // api_key_id
            &db::TokenUsageType::Prompt,
            &size,
            &None::<i32>, // duration_ms
        )
        .one()
        .await?;

    // Set the chat status to InProgress
    queries::chats::set_chat_status()
        .bind(&transaction, &ChatStatus::InProgress, &chat_id)
        .await?;

    tracing::debug!("{:?}", &user_config);

    // Are we tool aware? If so let's add tools to this conversation
    let tools = if capabilities
        .iter()
        .any(|c| c.capability == db::ModelCapability::tool_use)
    {
        // Get the base tools selected by the user
        let mut all_tools = get_chat_tools_user_selected_with_system_openapi(
            pool,
            user_config.enabled_tools.as_ref(),
        )
        .await;

        // Check if the chat has attachments
        if attachment_count > 0 {
            all_tools.extend(get_tools(ToolScope::DocumentIntelligence));
        }

        // Add integration tools from the prompt
        match super::context_builder::get_prompt_integration_tools(&transaction, prompt.id).await {
            Ok(integration_tools) => {
                tracing::info!("Adding {} integration tools", integration_tools.len());
                all_tools.extend(integration_tools);
            }
            Err(e) => {
                tracing::warn!("Failed to get integration tools: {}", e);
            }
        }

        if all_tools.is_empty() {
            None
        } else {
            Some(all_tools)
        }
    } else {
        None
    };

    if capabilities
        .iter()
        .any(|c| c.capability == db::ModelCapability::Guarded)
    {
        tracing::info!("This model is guarded. Sending chat to moderation model");
        let guard_model = queries::models::models()
            .bind(&transaction, &db::ModelType::Guard)
            .one()
            .await?;
        tracing::info!("Using {}", guard_model.name);

        let sanitized = strip_tool_data(&messages);
        match moderate_chat(
            &guard_model.base_url,
            guard_model.api_key.as_deref(),
            &guard_model.name,
            sanitized,
        )
        .await
        {
            Ok(ModerationVerdict::Safe) => {}
            Ok(ModerationVerdict::Unsafe(code)) => {
                queries::chats::new_chat()
                    .bind(
                        &transaction,
                        &conversation.id,
                        &chat.prompt_id,
                        &None::<String>,
                        &None::<String>,
                        &"Your question violated our guidelines",
                        &ChatRole::Assistant,
                        &ChatStatus::Error,
                    )
                    .one()
                    .await?;
                queries::prompt_flags::insert_prompt_flag()
                    .bind(&transaction, &chat_id, &code)
                    .await?;
                transaction.commit().await?;
                return Err(CustomError::FaultySetup("Moderation failed".into()));
            }
            Err(status) => {
                transaction.commit().await?;
                return Err(CustomError::FaultySetup(format!(
                    "Moderation failed: {status}"
                )));
            }
        }
    }

    transaction.commit().await?;

    let completion = BionicChatCompletionRequest {
        model: model.name,
        stream: Some(true),
        max_tokens: prompt.max_completion_tokens,
        temperature: prompt.temperature,
        messages,
        tools,
        tool_choice: None,
    };
    let completion_json = serde_json::to_string(&completion)?;

    tracing::debug!(completion_json);

    let client = reqwest::Client::new();
    let request = if let Some(api_key) = model.api_key {
        client
            .post(format!("{}/chat/completions", model.base_url))
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(completion_json.to_string())
    } else {
        client
            .post(format!("{}/chat/completions", model.base_url))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(completion_json.to_string())
    };
    Ok((request, model.id, conversation.user_id))
}
