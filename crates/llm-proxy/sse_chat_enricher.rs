//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```

use axum::Error;
use openai_api::ChatCompletionDelta;
use reqwest::RequestBuilder;
use reqwest_eventsource::{Event as ReqwestEvent, EventSource as ReqwestEventSource};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct CompletionChunk {
    pub delta: String,
    pub merged: Option<ChatCompletionDelta>,
    pub snapshot: String,
}

#[derive(Debug)]
pub enum GenerationEvent {
    Text(CompletionChunk),
    End(CompletionChunk),
}

pub async fn enriched_chat(
    request: RequestBuilder,
    sender: mpsc::Sender<Result<GenerationEvent, Error>>,
    convert_errors_to_chat: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!("{:?}", &request);
    let mut stream = ReqwestEventSource::new(request)?;
    let mut snapshot = String::new();
    let mut merged: Option<ChatCompletionDelta> = None;

    while let Some(event) = stream.next().await {
        match event {
            Ok(ReqwestEvent::Open) => tracing::debug!("Connection Open!"),
            Ok(ReqwestEvent::Message(message)) => {
                if message.data.trim() == "[DONE]" {
                    let chunk = CompletionChunk {
                        delta: message.data.clone(),
                        merged: merged.clone(),
                        snapshot: snapshot.clone(),
                    };
                    stream.close();
                    sender.send(Ok(GenerationEvent::End(chunk))).await.ok();
                    break;
                }

                tracing::debug!("{}", &message.data);
                let delta: ChatCompletionDelta = match serde_json::from_str(&message.data) {
                    Ok(delta) => delta,
                    Err(e) => {
                        handle_chat_error(
                            e,
                            convert_errors_to_chat,
                            &mut snapshot,
                            &sender,
                            &mut stream,
                        )
                        .await?;
                        break;
                    }
                };

                match merged.as_mut() {
                    Some(c) => {
                        if let Err(err) = c.merge(delta.clone()) {
                            tracing::warn!("Error merging delta: {:?}, using new delta", err);
                            *c = delta.clone();
                        }
                    }
                    None => merged = Some(delta.clone()),
                }

                if let Some(text) = &delta.choices[0].delta.content {
                    snapshot.push_str(text);
                    let chunk = CompletionChunk {
                        delta: message.data.clone(),
                        merged: merged.clone(),
                        snapshot: snapshot.clone(),
                    };
                    if sender.send(Ok(GenerationEvent::Text(chunk))).await.is_err() {
                        break;
                    }
                }
            }
            Err(err) => {
                handle_chat_error(
                    err,
                    convert_errors_to_chat,
                    &mut snapshot,
                    &sender,
                    &mut stream,
                )
                .await?;
                break;
            }
        }
    }

    Ok(())
}

fn convert_error_to_chats(
    err: impl std::fmt::Debug + std::fmt::Display,
) -> Vec<(CompletionChunk, String)> {
    vec![
        {
            let msg = "\n\n*Unable to complete your request due to the following error*";
            (super::sse_chat_error::string_to_chunk(msg), msg.to_string())
        },
        {
            let msg = format!("\n\n`{}`\n\n", err);
            (super::sse_chat_error::string_to_chunk(&msg), msg)
        },
        {
            let msg = format!("\n\n```\n{:#?}\n```", err);
            (super::sse_chat_error::string_to_chunk(&msg), msg)
        },
    ]
}

async fn handle_chat_error<E: std::error::Error + Send + Sync + 'static>(
    err: E,
    convert_errors_to_chat: bool,
    snapshot: &mut String,
    sender: &mpsc::Sender<Result<GenerationEvent, Error>>,
    stream: &mut ReqwestEventSource,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::error!("Chat error: {:?}", err);
    stream.close();

    if convert_errors_to_chat {
        for (chunk, markdown) in convert_error_to_chats(err) {
            snapshot.push_str(&markdown);
            sender.send(Ok(GenerationEvent::Text(chunk))).await?;
        }
        sender
            .send(Ok(GenerationEvent::End(CompletionChunk {
                delta: "[DONE]".into(),
                merged: None,
                snapshot: snapshot.clone(),
            })))
            .await?;
    } else {
        sender.send(Err(Error::new(err))).await?;
    }

    Err("stream closed after error".into())
}
