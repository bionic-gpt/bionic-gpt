use std::sync::Arc;

use super::sse_chat_enricher::{enriched_chat, GenerationEvent};
use super::Completion;
use crate::CustomError;
use axum::body::Body;
use axum::extract::Request;
use axum::response::{sse::Event, Sse};
use axum::{Extension, RequestExt};
use db::ChatStatus;
use db::{queries, Pool, Transaction};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::ApiChatHandler;

// We reverse proxy an LLM model, we also provide context to the chat if the
// API key is attached to a prompt that is attached to a dataset.
pub async fn chat_generate(
    ApiChatHandler {}: ApiChatHandler,
    Extension(pool): Extension<Pool>,
    req: Request<Body>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>>, CustomError> {
    if let Some(api_key) = req.headers().get("Authorization") {
        let api_key = api_key.to_str().unwrap().replace("Bearer ", "");
        let mut db_client = pool.get().await.unwrap();
        let transaction = db_client.transaction().await.unwrap();

        let body: String = req
            .extract()
            .await
            .map_err(|_| CustomError::FaultySetup("Error extracting".to_string()))?;
        let completion: Completion = serde_json::from_str(&body)?;

        let (api_key, request, api_chat_id) =
            create_request(&transaction, api_key, completion).await?;

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
                            log_end_of_chat(
                                pool,
                                &completion_chunk.snapshot,
                                &api_key,
                                api_chat_id,
                            )
                            .await
                            .unwrap();
                            Ok(Event::default().data(completion_chunk.delta))
                        }
                    },
                    Err(e) => Err(axum::Error::new(e)),
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

async fn create_request(
    transaction: &Transaction<'_>,
    api_key: String,
    completion: Completion,
) -> Result<(db::ApiKey, reqwest::RequestBuilder, i32), CustomError> {
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

    let prompt = queries::prompts::prompt_by_api_key()
        .bind(transaction, &api_key.api_key)
        .one()
        .await?;
    let model = queries::models::model()
        .bind(transaction, &prompt.model_id)
        .one()
        .await?;

    let messages = super::prompt::execute_prompt(
        transaction,
        prompt.id,
        prompt.team_id,
        None,
        completion.messages,
    )
    .await?;

    let completion = Completion {
        messages,
        ..completion
    };

    let completion_json = serde_json::to_string(&completion)?;

    tracing::debug!("{:?}", &completion_json);

    let api_chat_id =
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

    Ok((api_key, request, api_chat_id))
}

async fn log_initial_chat(
    transaction: &Transaction<'_>,
    api_key_id: i32,
    completion_json: &str,
    completion: &Completion,
) -> Result<i32, CustomError> {
    let size = super::token_count::token_count(completion).await;

    // Store the prompt, ready for the front end webcomponent to pickup
    let api_chat_id = queries::api_keys::new_api_chat()
        .bind(transaction, &api_key_id, &completion_json, &size)
        .one()
        .await?;

    Ok(api_chat_id)
}

async fn log_end_of_chat(
    pool: Arc<Pool>,
    snapshot: &str,
    api_key: &str,
    api_chat_id: i32,
) -> Result<(), CustomError> {
    let completion = Completion {
        model: "".to_string(),
        max_tokens: None,
        stream: None,
        messages: vec![super::Message {
            role: "".to_string(),
            content: snapshot.to_string(),
        }],
        temperature: None,
    };
    let size = super::token_count::token_count(&completion).await;
    let db_client = pool.get().await.unwrap();
    queries::api_keys::update_api_chat()
        .bind(
            &db_client,
            &snapshot,
            &ChatStatus::Success,
            &size,
            &api_key,
            &api_chat_id,
        )
        .await?;

    Ok(())
}
