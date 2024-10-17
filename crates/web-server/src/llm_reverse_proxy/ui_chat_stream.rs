//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```
use db::ChatStatus;
use std::sync::Arc;

use super::sse_chat_enricher::{enriched_chat, GenerationEvent};
use super::sse_chat_error::error_to_chat;
use crate::auth::Authentication;
use crate::CustomError;
use axum::response::{sse::Event, Sse};
use axum::Extension;
use db::{queries, Pool};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    RequestBuilder,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::{limits, UICompletions};
use super::{Completion, Message};

// Called from the front end to generate a streaming chat with the model
pub async fn chat_generate(
    UICompletions { chat_id }: UICompletions,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>>, CustomError> {
    match create_request(&pool, &current_user, chat_id).await {
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
                                save_results(&pool, &completion_chunk.snapshot, chat_id, &sub)
                                    .await
                                    .unwrap();
                                Ok(Event::default().data(completion_chunk.delta))
                            }
                        },
                        Err(e) => {
                            let save = save_results(&pool, &e.to_string(), chat_id, &sub).await;
                            if let Err(save) = save {
                                tracing::error!(
                                    "Error trying to save results from receiver stream: {:?}",
                                    save
                                );
                            }
                            Err(axum::Error::new(e))
                        }
                    }
                }
            });

            Ok(Sse::new(event_stream))
        }
        Err(err) => {
            let save = save_results(&pool, &err.to_string(), chat_id, &current_user.sub).await;
            if let Err(save) = save {
                tracing::error!(
                    "Error trying to save results from create_request: {:?}",
                    save
                );
            }
            Err(CustomError::FaultySetup(err.to_string()))
        }
    }
}

// When the chta has completed, store the results in the database.
async fn save_results(
    pool: &Pool,
    delta: &str,
    chat_id: i32,
    sub: &str,
) -> Result<(), CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;
    db::authz::set_row_level_security_user_id(&transaction, sub.to_string()).await?;
    queries::chats::update_chat()
        .bind(
            &transaction,
            &delta,
            &super::token_count::token_count_from_string(delta).await,
            &ChatStatus::Success,
            &chat_id,
        )
        .await?;
    transaction.commit().await?;
    Ok(())
}

// Create the request that we'll send to reqwest to create an SSE stream of incoming
// chat completions.
async fn create_request(
    pool: &Pool,
    current_user: &Authentication,
    chat_id: i32,
) -> Result<(RequestBuilder, i32, i32), CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;
    db::authz::set_row_level_security_user_id(&transaction, current_user.sub.to_string()).await?;
    let model = queries::models::model_host_by_chat_id()
        .bind(&transaction, &chat_id)
        .one()
        .await?;
    let chat = queries::chats::chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;
    let conversation = queries::conversations::get_conversation_from_chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;
    let prompt = queries::prompts::prompt()
        .bind(&transaction, &chat.prompt_id, &conversation.team_id)
        .one()
        .await?;
    let chat_request = Message {
        role: "user".to_string(),
        content: chat.user_request,
    };
    let messages = super::prompt::execute_prompt(
        &transaction,
        prompt.id,
        conversation.team_id,
        Some(conversation.id),
        vec![chat_request],
    )
    .await?;
    let json_messages = serde_json::to_string(&messages)?;
    let size = super::token_count::token_count_from_messages(messages.clone()).await;
    queries::chats::update_prompt()
        .bind(&transaction, &json_messages, &size, &chat_id)
        .await?;
    transaction.commit().await?;
    let completion = Completion {
        model: model.name,
        stream: Some(true),
        max_tokens: Some(prompt.max_tokens),
        temperature: prompt.temperature,
        messages,
    };
    let completion_json = serde_json::to_string(&completion)?;
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
