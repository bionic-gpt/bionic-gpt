//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```
use crate::auth::Authentication;
use crate::CustomError;
use axum::response::{sse::Event, Sse};
use axum::Error;
use axum::Extension;
use db::{queries, Pool};
use futures::stream::{self};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    RequestBuilder,
};
use reqwest_eventsource::{Event as ReqwestEvent, EventSource as ReqwestEventSource};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::api_reverse_proxy::{Completion, Message};
use super::UICompletions;

#[derive(Debug)]
pub enum GenerationEvent {
    Text(String),
    End(String),
}

pub async fn chat_generate(
    UICompletions { chat_id }: UICompletions,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>>, CustomError> {
    // Read api key from .env

    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;

    db::authz::set_row_level_security_user_id(&transaction, current_user.sub).await?;

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

    queries::chats::update_prompt()
        .bind(&transaction, &json_messages, &chat_id)
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

    // Create a client
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

    // Create a channel for sending SSE events
    let (sender, receiver) = mpsc::channel::<Result<GenerationEvent, axum::Error>>(10);

    // Spawn a task that generates SSE events and sends them into the channel
    tokio::spawn(async move {
        // Call your existing function to start generating events
        if let Err(e) = generate_sse_stream_real(request, sender).await {
            eprintln!("Error generating SSE stream: {:?}", e);
        }
    });

    let receiver_stream = ReceiverStream::new(receiver);
    let initial_state = (receiver_stream, String::new()); // Initial state with an empty accumulator
    let event_stream = stream::unfold(initial_state, move |(mut rc, mut accumulated)| {
        async move {
            match rc.next().await {
                Some(Ok(event)) => {
                    // Process the event
                    match event {
                        GenerationEvent::Text(text) => {
                            accumulated.push_str(&text);

                            Some((Ok(Event::default().data(text)), (rc, accumulated)))
                        }
                        GenerationEvent::End(text) => {
                            println!("accumulated: {:?}", accumulated);

                            let s = format!(
                                r##"<div hx-swap-oob="outerHTML:#message-container">{}</div>"##,
                                accumulated
                            );
                            // append s to text
                            let ss = format!("{}\n{}", text, s);
                            println!("ss: {}", ss);

                            // accumulated.push_str(&ss);
                            // Handle the end of a sequence, possibly resetting the accumulator if needed
                            Some((Ok(Event::default().data(text)), (rc, String::new())))
                        } // ... handle other event types if necessary ...
                    }
                }
                Some(Err(e)) => {
                    // Handle error without altering the accumulator
                    Some((Err(axum::Error::new(e)), (rc, accumulated)))
                }
                None => None, // When the receiver stream ends, finish the stream
            }
        }
    });

    Ok(Sse::new(event_stream))
}

pub async fn generate_sse_stream_real(
    request: RequestBuilder,
    sender: mpsc::Sender<Result<GenerationEvent, Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Start streaming
    let mut stream = ReqwestEventSource::new(request)?;

    // Handle streaming events
    while let Some(event) = stream.next().await {
        match event {
            Ok(ReqwestEvent::Open) => println!("Connection Open!"),
            Ok(ReqwestEvent::Message(message)) => {
                if message.data.trim() == "[DONE]" {
                    println!("Stream completed.");
                    stream.close();
                    if sender
                        // .send(Ok(Event::default()
                        //     .data(r#"<div id="sse-listener" hx-swap-oob="true"></div>"#)))
                        .send(Ok(GenerationEvent::End("[DONE]".to_string())))
                        .await
                        .is_err()
                    {
                        break; // Receiver has dropped, stop sending.
                    }
                    break;
                } else {
                    let m: Value = serde_json::from_str(&message.data).unwrap();
                    if sender
                        .send(Ok(GenerationEvent::Text(m.to_string())))
                        .await
                        .is_err()
                    {
                        break; // Receiver has dropped, stop sending.
                    }
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                stream.close();
                if sender.send(Err(axum::Error::new(err))).await.is_err() {
                    break; // Receiver has dropped, stop sending.
                }
            }
        }
    }

    Ok(())
}
