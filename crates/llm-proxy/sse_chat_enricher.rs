//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```
use axum::Error;
use reqwest::RequestBuilder;
use reqwest_eventsource::{Event as ReqwestEvent, EventSource as ReqwestEventSource};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct CompletionChunk {
    pub delta: String,
    pub snapshot: String,
}

#[derive(Debug)]
pub enum GenerationEvent {
    Text(CompletionChunk),
    End(CompletionChunk),
}

// Create an SSE connection to the model and intercept the incoming stream.
// Enrich the stream i.e. add results so far.
pub async fn enriched_chat(
    request: RequestBuilder,
    sender: mpsc::Sender<Result<GenerationEvent, Error>>,
    convert_errors_to_chat: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!("{:?}", &request);
    // Start streaming
    let mut stream = ReqwestEventSource::new(request)?;
    let mut snapshot = String::new();

    // Handle streaming events
    while let Some(event) = stream.next().await {
        match event {
            Ok(ReqwestEvent::Open) => tracing::debug!("Connection Open!"),
            Ok(ReqwestEvent::Message(message)) => {
                if message.data.trim() == "[DONE]" {
                    let chunk = CompletionChunk {
                        delta: message.data.clone(),
                        snapshot: snapshot.clone(),
                    };
                    stream.close();
                    if sender.send(Ok(GenerationEvent::End(chunk))).await.is_err() {
                        break; // Receiver has dropped, stop sending.
                    }
                    break;
                } else {
                    let m: Value = serde_json::from_str(&message.data)?;
                    if let Some(text) = m["choices"][0]["delta"]["content"].as_str() {
                        snapshot.push_str(text);
                        let chunk = CompletionChunk {
                            delta: message.data.clone(),
                            snapshot: snapshot.clone(),
                        };
                        if sender.send(Ok(GenerationEvent::Text(chunk))).await.is_err() {
                            break; // Receiver has dropped, stop sending.
                        }
                    }
                }
            }
            Err(err) => {
                tracing::error!("{:?}", err);
                if convert_errors_to_chat {
                    let completion_chunks = convert_error_to_chats(err);
                    for (chunk, markdown) in completion_chunks {
                        snapshot.push_str(&markdown);
                        sender.send(Ok(GenerationEvent::Text(chunk))).await?;
                    }
                    sender
                        .send(Ok(GenerationEvent::End(CompletionChunk {
                            delta: "[DONE]".to_string(),
                            snapshot: snapshot.clone(),
                        })))
                        .await?;
                } else if sender.send(Err(axum::Error::new(err))).await.is_err() {
                    break; // Receiver has dropped, stop sending.
                }
                stream.close();
            }
        }
    }

    Ok(())
}
// During a chat setion its good to let the assitant show the error,
// otherwise they get buried in the logs.
fn convert_error_to_chats(err: reqwest_eventsource::Error) -> Vec<(CompletionChunk, String)> {
    vec![
        string_to_chunk("\n\n*Unable to complete your request due to the following error*"),
        string_to_chunk(&format!("\n\n`{}`\n\n", err)),
        string_to_chunk(&format!("\n\n```\n{:#?}\n```", err)),
    ]
}

// During a chat setion its good to let the assitant show the error,
// otherwise they get buried in the logs.
fn string_to_chunk(content: &str) -> (CompletionChunk, String) {
    (
        super::sse_chat_error::string_to_chunk(content),
        content.to_string(),
    )
}
