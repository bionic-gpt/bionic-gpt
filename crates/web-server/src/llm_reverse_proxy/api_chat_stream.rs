//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```
use super::sse_chat_enricher::{enriched_chat, GenerationEvent};
use crate::CustomError;
use axum::body::Body;
use axum::extract::Request;
use axum::response::{sse::Event, Sse};
use axum::{Extension, RequestExt};
use db::{queries, Pool};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::ApiChatHandler;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

// Called from the front end to generate a streaming chat with the model
pub async fn chat_generate(
    ApiChatHandler {}: ApiChatHandler,
    Extension(pool): Extension<Pool>,
    req: Request<Body>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>>, CustomError> {
    if let Some(api_key) = req.headers().get("Authorization") {
        let api_key = api_key.to_str().unwrap().replace("Bearer ", "");
        let mut db_client = pool.get().await.unwrap();
        let transaction = db_client.transaction().await.unwrap();

        // Check this first, if we have a false API key then return auth error
        let api_key = queries::api_keys::find_api_key()
            .bind(&transaction, &api_key)
            .one()
            .await
            .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?;

        let prompt = queries::prompts::prompt_by_api_key()
            .bind(&transaction, &api_key.api_key)
            .one()
            .await?;

        let model = queries::models::model()
            .bind(&transaction, &prompt.model_id)
            .one()
            .await?;

        let body: String = req
            .extract()
            .await
            .map_err(|_| CustomError::FaultySetup("Error extracting".to_string()))?;

        let client = reqwest::Client::new();
        let request = if let Some(api_key) = model.api_key {
            client
                .post(format!("{}/chat/completions", model.base_url))
                .header(AUTHORIZATION, format!("Bearer {}", api_key))
                .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .body(body)
        } else {
            client
                .post(format!("{}/chat/completions", model.base_url))
                .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .body(body)
        };

        // Create a channel for sending SSE events
        let (sender, receiver) = mpsc::channel::<Result<GenerationEvent, axum::Error>>(10);

        // Spawn a task that generates SSE events and sends them into the channel
        tokio::spawn(async move {
            // Call your existing function to start generating events
            if let Err(e) = enriched_chat(request, sender, false).await {
                eprintln!("Error generating SSE stream: {:?}", e);
            }
        });

        let receiver_stream = ReceiverStream::new(receiver);

        let event_stream = receiver_stream.map(|item| {
            match item {
                Ok(event) => match event {
                    GenerationEvent::Text(text) => Ok(Event::default().data(text)),
                    GenerationEvent::End(text) => Ok(Event::default().data(text)),
                },
                Err(e) => {
                    // Handle error without altering the accumulator
                    Err(axum::Error::new(e))
                }
            }
        });

        Ok(Sse::new(event_stream))
    } else {
        Err(CustomError::Authentication(
            "You need an API key".to_string(),
        ))
    }
}
