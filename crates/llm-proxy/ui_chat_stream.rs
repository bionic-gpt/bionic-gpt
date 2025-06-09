//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```
use super::sse_chat_enricher::{enriched_chat, GenerationEvent};
use super::sse_chat_error::error_to_chat;
use crate::chat_converter;
use crate::errors::CustomError;
use crate::jwt::Jwt;
use crate::user_config::UserConfig;
use axum::response::{sse::Event, Sse};
use axum::Extension;
use db::{queries, Pool};
use db::{ChatRole, ChatStatus};
use integrations::{execute_tool_calls, get_chat_tools_user_selected, get_tools, ToolScope};
use openai_api::{BionicChatCompletionRequest, ToolCall};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    RequestBuilder,
};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::{limits, UICompletions};

// Called from the front end to generate a streaming chat with the model
pub async fn chat_generate(
    UICompletions { chat_id }: UICompletions,
    current_user: Jwt,
    user_config: UserConfig,
    Extension(pool): Extension<Pool>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>>, CustomError> {
    match create_request(&pool, &current_user, chat_id, &user_config).await {
        Ok((request, model_id, user_id)) => {
            let is_limit_breached =
                limits::is_limit_exceeded_from_pool(&pool, model_id, user_id).await?;

            // Create a channel for sending SSE events
            let (sender, receiver) = mpsc::channel::<Result<GenerationEvent, axum::Error>>(10);

            // Spawn a task that generates SSE events and sends them into the channel
            tokio::spawn(async move {
                if is_limit_breached {
                    // Call your existing function to start generating events
                    if let Err(e) =
                        error_to_chat("You have exceeded your token limit for this model", sender)
                            .await
                    {
                        tracing::warn!("Limits exceeded: {:?}", e);
                    }
                } else {
                    // Call your existing function to start generating events
                    if let Err(e) = enriched_chat(request, sender, true).await {
                        tracing::error!("Error generating SSE stream: {:?}", e);
                    }
                }
            });

            let sub_arc = Arc::new(current_user.sub.clone());
            let pool_arc = Arc::new(pool.clone());
            let receiver_stream = ReceiverStream::new(receiver);

            let event_stream = receiver_stream.then(move |item| {
                let pool = Arc::clone(&pool_arc);
                let sub = Arc::clone(&sub_arc);
                async move {
                    match item {
                        Ok(event) => match event {
                            GenerationEvent::Text(completion_chunk) => {
                                Ok(Event::default().data(completion_chunk.delta))
                            }
                            GenerationEvent::End(completion_chunk) => {
                                let mut tool_calls: Option<Vec<ToolCall>> = None;
                                if let Some(merged) = completion_chunk.merged {
                                    if let Some(tcs) = &merged.choices[0].delta.tool_calls {
                                        tracing::info!("Detected tool calls: {:?}", tcs);
                                        tool_calls = Some(tcs.clone());
                                    }
                                }

                                tracing::debug!("End of stream saving data");
                                save_results(
                                    &pool,
                                    &completion_chunk.snapshot,
                                    tool_calls,
                                    chat_id,
                                    &sub,
                                    ChatStatus::Success,
                                )
                                .await;

                                Ok(Event::default().data(completion_chunk.delta))
                            }
                        },
                        Err(e) => {
                            save_results(
                                &pool,
                                &e.to_string(),
                                None,
                                chat_id,
                                &sub,
                                ChatStatus::Error,
                            )
                            .await;
                            Err(axum::Error::new(e))
                        }
                    }
                }
            });

            Ok(Sse::new(event_stream))
        }
        Err(err) => {
            save_results(
                &pool,
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
async fn save_results(
    pool: &Pool,
    snapshot: &str,
    tool_calls: Option<Vec<ToolCall>>,
    chat_id: i32,
    sub: &str,
    status: ChatStatus, // New parameter
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
    let completion_tokens = super::token_count::token_count_from_string(snapshot);

    tracing::debug!(
        "save_results: Executing chat query with chat_id: {}",
        chat_id
    );
    if let Ok(chat) = queries::chats::chat()
        .bind(&transaction, &chat_id)
        .one()
        .await
    {
        if let Err(e) = queries::chats::new_chat()
            .bind(
                &transaction,
                &chat.conversation_id,
                &chat.prompt_id,
                &None::<String>,
                &tool_calls_json,
                &snapshot,
                &ChatRole::Assistant,
                &ChatStatus::Success,
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

        if let Some(tool_calls) = tool_calls {
            let tool_call_results = execute_tool_calls(
                tool_calls.clone(),
                Some(pool),
                Some(sub.to_string()),
                Some(chat.conversation_id),
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

                if let Err(e) = queries::chats::new_chat()
                    .bind(
                        &transaction,
                        &chat.conversation_id,
                        &chat.prompt_id,
                        &Some(tool_call.id),
                        &None::<String>,
                        &result_json,
                        &ChatRole::Tool,
                        &ChatStatus::Pending,
                    )
                    .one()
                    .await
                {
                    tracing::error!("Error creating tool call results chat: {:?}", e);
                    return;
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

    let messages = super::prompt::execute_prompt(
        &transaction,
        prompt.clone(),
        Some(conversation.id),
        chat_history,
    )
    .await?;

    let size = super::token_count::token_count(messages.clone());

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
        let mut all_tools = get_chat_tools_user_selected(user_config.enabled_tools.as_ref());

        // Check if the chat has attachments
        if attachment_count > 0 {
            all_tools.extend(get_tools(ToolScope::DocumentIntelligence));
        }

        // Add integration tools from the prompt
        match super::prompt::get_prompt_integration_tools(
            &transaction,
            prompt.id,
            current_user.sub.clone(),
        )
        .await
        {
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

    transaction.commit().await?;

    let completion = BionicChatCompletionRequest {
        model: model.name,
        stream: Some(true),
        max_tokens: Some(prompt.max_tokens),
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
