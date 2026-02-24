//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```

use crate::chat_types::ChatCompletionDelta;
use axum::Error;
use reqwest::RequestBuilder;
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

#[derive(Debug)]
pub enum EnrichedChatOutcome {
    Completed,
    ClientDisconnected {
        snapshot: String,
        merged: Option<ChatCompletionDelta>,
    },
}

pub async fn enriched_chat(
    request: RequestBuilder,
    sender: mpsc::Sender<Result<GenerationEvent, Error>>,
    convert_errors_to_chat: bool,
) -> Result<EnrichedChatOutcome, Box<dyn std::error::Error>> {
    tracing::debug!("{:?}", &request);
    let response = request.send().await?;
    if !response.status().is_success() {
        return Err(format!("SSE request failed: {}", response.status()).into());
    }
    let mut stream = response.bytes_stream();
    let mut snapshot = String::new();
    let mut merged: Option<ChatCompletionDelta> = None;
    let mut buffer = String::new();

    while let Some(item) = stream.next().await {
        let chunk = match item {
            Ok(bytes) => bytes,
            Err(err) => {
                let outcome = handle_chat_error(
                    err,
                    convert_errors_to_chat,
                    &mut snapshot,
                    &sender,
                    Some(&buffer),
                    merged.clone(),
                )
                .await?;
                return Ok(outcome);
            }
        };

        buffer.push_str(&String::from_utf8_lossy(&chunk));

        while let Some(event) = extract_next_event(&mut buffer) {
            let data = match event {
                Some(data) => data,
                None => continue,
            };

            if data.trim() == "[DONE]" {
                let chunk = CompletionChunk {
                    delta: data.clone(),
                    merged: merged.clone(),
                    snapshot: snapshot.clone(),
                };
                if sender.send(Ok(GenerationEvent::End(chunk))).await.is_err() {
                    return Ok(EnrichedChatOutcome::ClientDisconnected { snapshot, merged });
                }
                return Ok(EnrichedChatOutcome::Completed);
            }

            tracing::debug!("{}", &data);
            let delta: ChatCompletionDelta = match serde_json::from_str(&data) {
                Ok(delta) => delta,
                Err(e) => {
                    let outcome = handle_chat_error(
                        e,
                        convert_errors_to_chat,
                        &mut snapshot,
                        &sender,
                        Some(&data),
                        merged.clone(),
                    )
                    .await?;
                    return Ok(outcome);
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
                    delta: data.clone(),
                    merged: merged.clone(),
                    snapshot: snapshot.clone(),
                };
                if sender.send(Ok(GenerationEvent::Text(chunk))).await.is_err() {
                    return Ok(EnrichedChatOutcome::ClientDisconnected { snapshot, merged });
                }
            }
        }
    }

    Ok(EnrichedChatOutcome::ClientDisconnected { snapshot, merged })
}

fn convert_error_to_chats(
    err: impl std::fmt::Debug + std::fmt::Display,
    context_message: Option<&str>,
) -> Vec<(CompletionChunk, String)> {
    let mut messages = vec![{
        let msg = "\n\n*Unable to complete your request due to the following error*";
        (super::stream_errors::string_to_chunk(msg), msg.to_string())
    }];

    // Add original context message if provided
    if let Some(context) = context_message {
        let msg = format!(
            "\n\n**Original LLM Provider Response:**\n```\n{}\n```",
            context
        );
        messages.push((super::stream_errors::string_to_chunk(&msg), msg));
    }

    // Add processing error
    messages.extend([
        {
            let msg = format!("\n\n**Processing Error:**\n`{}`", err);
            (super::stream_errors::string_to_chunk(&msg), msg)
        },
        {
            let msg = format!("\n\n```\n{:#?}\n```", err);
            (super::stream_errors::string_to_chunk(&msg), msg)
        },
    ]);

    messages
}

async fn handle_chat_error<E: std::error::Error + Send + Sync + 'static>(
    err: E,
    convert_errors_to_chat: bool,
    snapshot: &mut String,
    sender: &mpsc::Sender<Result<GenerationEvent, Error>>,
    context_message: Option<&str>,
    merged: Option<ChatCompletionDelta>,
) -> Result<EnrichedChatOutcome, Box<dyn std::error::Error>> {
    tracing::error!("Chat error: {:?}", err);

    if convert_errors_to_chat {
        for (chunk, markdown) in convert_error_to_chats(err, context_message) {
            snapshot.push_str(&markdown);
            if sender.send(Ok(GenerationEvent::Text(chunk))).await.is_err() {
                return Ok(EnrichedChatOutcome::ClientDisconnected {
                    snapshot: snapshot.clone(),
                    merged,
                });
            }
        }
        if sender
            .send(Ok(GenerationEvent::End(CompletionChunk {
                delta: "[DONE]".into(),
                merged: None,
                snapshot: snapshot.clone(),
            })))
            .await
            .is_err()
        {
            return Ok(EnrichedChatOutcome::ClientDisconnected {
                snapshot: snapshot.clone(),
                merged,
            });
        }
        return Ok(EnrichedChatOutcome::Completed);
    } else {
        sender.send(Err(Error::new(err))).await?;
    }

    Ok(EnrichedChatOutcome::Completed)
}

fn extract_next_event(buffer: &mut String) -> Option<Option<String>> {
    if let Some(pos) = buffer.find("\n\n") {
        let raw = buffer[..pos].to_string();
        buffer.drain(..pos + 2);
        let data_lines: Vec<&str> = raw
            .lines()
            .filter_map(|line| line.strip_prefix("data:"))
            .map(|line| line.trim_start())
            .collect();
        if data_lines.is_empty() {
            return Some(None);
        }
        return Some(Some(data_lines.join("\n")));
    }
    None
}
