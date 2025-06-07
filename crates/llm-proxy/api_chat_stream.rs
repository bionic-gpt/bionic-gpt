use super::limits;
use super::sse_chat_enricher::{enriched_chat, GenerationEvent};
use crate::errors::CustomError;
use axum::body::Body;
use axum::extract::Request;
use axum::response::{sse::Event, Sse};
use axum::response::{IntoResponse, Response};
use axum::{Extension, RequestExt};
use db::{queries, Pool, Transaction};
use http::{HeaderMap, StatusCode};
use openai_api::BionicChatCompletionRequest;
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::ApiChatHandler;

// We reverse proxy an LLM model, we also provide context to the chat if the
// API key is attached to a prompt that is attached to a dataset.
// This handles calls to /v1/chat/completions
pub async fn chat_generate(
    ApiChatHandler {}: ApiChatHandler,
    Extension(pool): Extension<Pool>,
    req: Request<Body>,
) -> Result<Response<Body>, CustomError> {
    if let Some(api_key) = req.headers().get("Authorization") {
        let api_key = api_key
            .to_str()
            .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?
            .replace("Bearer ", "");
        let mut db_client = pool.get().await?;
        let transaction = db_client.transaction().await?;

        let body: String = req
            .extract()
            .await
            .map_err(|_| CustomError::FaultySetup("Error extracting".to_string()))?;
        let completion: BionicChatCompletionRequest = serde_json::from_str(&body)?;
        let streaming = completion.stream.unwrap_or(false);

        let (api_key, request) = create_request(&transaction, api_key, completion).await?;

        if limits::is_limit_exceeded(&transaction, api_key.model_id, api_key.user_id).await? {
            return Err(CustomError::Limits(
                "You have exceededs the token limits for this user and model".to_string(),
            ));
        }

        if streaming {
            // Create a channel for sending SSE events
            let (sender, receiver) = mpsc::channel::<Result<GenerationEvent, axum::Error>>(10);

            // Commit the transaction so we are sure the request is in the database.
            transaction.commit().await?;

            // Spawn a task that generates SSE events and sends them into the channel
            tokio::spawn(async move {
                tracing::debug!("Spawning enriched chat process");
                // Call your existing function to start generating events
                if let Err(e) = enriched_chat(request, sender, false).await {
                    tracing::error!("Error generating SSE stream: {:?}", e);
                }
            });

            let receiver_stream = ReceiverStream::new(receiver);
            let pool_arc = Arc::new(pool);
            let api_key_arc = Arc::new(api_key.api_key);

            // For every Server Side Event we get from the model process it
            // and return it to the caller.
            // The only extra processing we do is to log the full reponse when
            // the stream ends.
            let event_stream = receiver_stream.then(move |item| {
                let pool = Arc::clone(&pool_arc);
                let api_key = Arc::clone(&api_key_arc);
                async move {
                    match item {
                        Ok(event) => match event {
                            GenerationEvent::Text(completion_chunk) => {
                                Ok(Event::default().data(completion_chunk.delta))
                            }
                            GenerationEvent::End(completion_chunk) => {
                                log_end_of_chat(pool, &completion_chunk.snapshot, &api_key).await?;
                                Ok(Event::default().data(completion_chunk.delta))
                            }
                        },
                        Err(e) => Err(axum::Error::new(e)),
                    }
                }
            });
            Ok(Sse::new(event_stream).into_response())
        } else {
            // Non-streaming logic: generate the full response and return it
            let response = request.send().await.map_err(|e| {
                tracing::error!("Error calling model: {:?}", e);
                CustomError::FaultySetup("Error calling model".to_string())
            })?;

            // Commit the transaction, as the request was successful.
            transaction.commit().await?;

            // Extract status code
            let status = StatusCode::from_u16(response.status().as_u16()).map_err(|e| {
                tracing::error!("Error generating status code: {:?}", e);
                CustomError::FaultySetup("Error generating status code".to_string())
            })?;

            // Extract headers from reqwest response
            let mut headers = HeaderMap::new();
            for (key, value) in response.headers() {
                headers.insert(key, value.clone());
            }

            // Extract body
            let body_bytes = response.bytes().await?;
            let body = body_bytes.to_vec(); // Convert body to Vec<u8> (Axum uses hyper)

            // Build axum response
            let response = (status, headers, body).into_response();

            Ok(response)
        }
    } else {
        Err(CustomError::Authentication(
            "You need an API key".to_string(),
        ))
    }
}

async fn create_request(
    transaction: &Transaction<'_>,
    api_key: String,
    completion: BionicChatCompletionRequest,
) -> Result<(db::ApiKey, reqwest::RequestBuilder), CustomError> {
    let api_key = queries::api_keys::find_api_key()
        .bind(transaction, &api_key)
        .one()
        .await
        .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?;

    // Now we have an API Key we can kick off RLS
    transaction
        .query(
            &format!("SET LOCAL row_level_security.user_id = {}", api_key.user_id),
            &[],
        )
        .await?;

    // First get the prompt ID from the API key
    let prompt_info = queries::prompts::prompt_by_api_key()
        .bind(transaction, &api_key.api_key)
        .one()
        .await?;

    // Then get the full prompt details using the SinglePrompt query
    let prompt = queries::prompts::prompt()
        .bind(transaction, &prompt_info.id, &prompt_info.team_id)
        .one()
        .await?;
    let model = queries::models::model()
        .bind(transaction, &prompt.model_id)
        .one()
        .await?;

    let messages =
        super::prompt::execute_prompt(transaction, prompt.clone(), None, completion.messages)
            .await?;
    let completion = BionicChatCompletionRequest {
        messages,
        ..completion
    };

    let completion_json = serde_json::to_string(&completion)?;

    tracing::debug!("{:?}", &completion_json);

    log_initial_chat(transaction, api_key.id, &completion_json, &completion).await?;

    let client = reqwest::Client::new();
    let request = if let Some(api_key) = model.api_key {
        client
            .post(format!("{}/chat/completions", model.base_url))
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(completion_json)
    } else {
        client
            .post(format!("{}/chat/completions", model.base_url))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(completion_json)
    };

    Ok((api_key, request))
}

async fn log_initial_chat(
    transaction: &Transaction<'_>,
    api_key_id: i32,
    completion_json: &str,
    completion: &BionicChatCompletionRequest,
) -> Result<(), CustomError> {
    let size = super::token_count::token_count(completion.messages.clone());

    // Create the API chat entry with new structure
    queries::api_keys::new_api_chat()
        .bind(
            transaction,
            &api_key_id,
            &completion_json,
            &db::ChatRole::User,
            &db::ChatStatus::Pending,
        )
        .one()
        .await?;

    // Track prompt token usage in token_usage_metrics
    queries::token_usage_metrics::create_token_usage_metric()
        .bind(
            transaction,
            &None::<i32>, // chat_id
            &Some(api_key_id),
            &db::TokenUsageType::Prompt,
            &size,
            &None::<i32>, // duration_ms
        )
        .one()
        .await?;

    Ok(())
}

async fn log_end_of_chat(
    pool: Arc<Pool>,
    snapshot: &str,
    api_key: &str,
) -> Result<(), CustomError> {
    let completion_tokens = super::token_count::token_count_from_string(snapshot);
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;

    // Get the API key record to get the api_key_id
    let api_key_record = queries::api_keys::find_api_key()
        .bind(&transaction, &api_key)
        .one()
        .await?;

    // Create a new API chat entry for the assistant's response
    queries::api_keys::new_api_chat()
        .bind(
            &transaction,
            &api_key_record.id,
            &snapshot,
            &db::ChatRole::Assistant,
            &db::ChatStatus::Success,
        )
        .one()
        .await?;

    // Track completion token usage in token_usage_metrics
    queries::token_usage_metrics::create_token_usage_metric()
        .bind(
            &transaction,
            &None::<i32>, // chat_id
            &Some(api_key_record.id),
            &db::TokenUsageType::Completion,
            &completion_tokens,
            &None::<i32>, // duration_ms - we could add timing here later
        )
        .one()
        .await?;

    transaction.commit().await?;
    Ok(())
}
