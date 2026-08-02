use crate::chat_request::{create_request, RigChatRequest};
use crate::errors::CustomError;
use crate::jwt::Jwt;
pub(crate) use crate::result_sink::ResultSink;
use crate::result_sink::{DbResultSink, SaveRequest};
use crate::user_config::UserConfig;
use axum::response::{sse::Event, Sse};
use axum::Extension;
use db::{ChatStatus, Pool};
use rig::client::CompletionClient;
use rig::completion::{CompletionModel as _, GetTokenUsage, Usage};
use rig::message::ReasoningContent;
use rig::providers::openai;
use rig::streaming::StreamedAssistantContent;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tool_runtime::{Reasoning, ToolCall};

use super::{limits, UICompletions};

/// Formats an SSE message for a streaming text chunk.
fn event_data_for_text(delta: String) -> String {
    json!({
        "type": "text_delta",
        "data": {
            "delta": delta
        }
    })
    .to_string()
}

/// Formats an SSE message for stream completion.
fn event_data_for_done() -> String {
    json!({
        "type": "done",
        "data": {}
    })
    .to_string()
}

/// Formats an SSE error event payload.
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

#[derive(Debug)]
pub enum GenerationEvent {
    Text {
        delta: String,
    },
    End {
        snapshot: String,
        tool_calls: Option<Vec<ToolCall>>,
        reasoning: Option<Vec<Reasoning>>,
        usage: Option<Usage>,
    },
}

#[derive(Debug)]
pub(crate) enum StreamOutcome {
    Completed,
    ClientDisconnected {
        snapshot: String,
        tool_calls: Option<Vec<ToolCall>>,
        reasoning: Option<Vec<Reasoning>>,
        usage: Option<Usage>,
    },
}

/// Converts generation events into SSE events and persists the final result.
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
                    GenerationEvent::Text { delta } => {
                        Ok(Event::default().data(event_data_for_text(delta)))
                    }
                    GenerationEvent::End {
                        snapshot,
                        tool_calls,
                        reasoning,
                        usage,
                    } => {
                        result_sink
                            .save(SaveRequest {
                                snapshot: &snapshot,
                                tool_calls,
                                reasoning,
                                usage,
                                chat_id,
                                sub: &sub,
                                status: ChatStatus::Success,
                            })
                            .await;
                        Ok(Event::default().data(event_data_for_done()))
                    }
                },
                Err(e) => {
                    let message = e.to_string();
                    result_sink
                        .save(SaveRequest {
                            snapshot: &message,
                            tool_calls: None,
                            reasoning: None,
                            usage: None,
                            chat_id,
                            sub: &sub,
                            status: ChatStatus::Error,
                        })
                        .await;
                    Ok(event_data_for_error(message))
                }
            }
        }
    })
}

/// Handles `/completions/{chat_id}` and streams model output to the client.
pub async fn chat_generate(
    UICompletions { chat_id }: UICompletions,
    current_user: Jwt,
    user_config: UserConfig,
    Extension(pool): Extension<Pool>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>>, CustomError> {
    let result_sink: Arc<dyn ResultSink> = Arc::new(DbResultSink::new(pool.clone()));

    match create_request(&pool, &current_user, chat_id, &user_config).await {
        Ok(request) => {
            let is_limit_breached =
                limits::is_limit_exceeded_from_pool(&pool, request.model_id, request.user_id)
                    .await?;

            let (sender, receiver) = mpsc::channel::<Result<GenerationEvent, axum::Error>>(10);
            let result_sink_clone = Arc::clone(&result_sink);
            let sub_for_save = current_user.sub.clone();

            tokio::spawn(async move {
                if is_limit_breached {
                    let limit_message = "You have exceeded your token limit for this model";
                    if sender
                        .send(Err(axum::Error::new(std::io::Error::other(limit_message))))
                        .await
                        .is_err()
                    {
                        result_sink_clone
                            .save(SaveRequest {
                                snapshot: limit_message,
                                tool_calls: None,
                                reasoning: None,
                                usage: None,
                                chat_id,
                                sub: &sub_for_save,
                                status: ChatStatus::Error,
                            })
                            .await;
                    }
                    return;
                }

                match stream_chat_with_rig(request, sender.clone()).await {
                    Ok(StreamOutcome::Completed) => {}
                    Ok(StreamOutcome::ClientDisconnected {
                        snapshot,
                        tool_calls,
                        reasoning,
                        usage,
                    }) => {
                        result_sink_clone
                            .save(SaveRequest {
                                snapshot: &snapshot,
                                tool_calls,
                                reasoning,
                                usage,
                                chat_id,
                                sub: &sub_for_save,
                                status: ChatStatus::Error,
                            })
                            .await;
                    }
                    Err(err) => {
                        let err_msg = err.to_string();
                        tracing::error!("Error generating SSE stream: {}", err_msg);
                        if sender
                            .send(Err(axum::Error::new(std::io::Error::other(
                                err_msg.clone(),
                            ))))
                            .await
                            .is_err()
                        {
                            result_sink_clone
                                .save(SaveRequest {
                                    snapshot: &err_msg,
                                    tool_calls: None,
                                    reasoning: None,
                                    usage: None,
                                    chat_id,
                                    sub: &sub_for_save,
                                    status: ChatStatus::Error,
                                })
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
                .save(SaveRequest {
                    snapshot: &err.to_string(),
                    tool_calls: None,
                    reasoning: None,
                    usage: None,
                    chat_id,
                    sub: &current_user.sub,
                    status: ChatStatus::Error,
                })
                .await;
            Err(CustomError::FaultySetup(err.to_string()))
        }
    }
}

/// Executes a streaming rig completion and publishes intermediate events.
pub(crate) async fn stream_chat_with_rig(
    request: RigChatRequest,
    sender: mpsc::Sender<Result<GenerationEvent, axum::Error>>,
) -> Result<StreamOutcome, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = request.api_key.as_deref().unwrap_or("");
    let client = openai::Client::builder()
        .api_key(api_key)
        .base_url(&request.base_url)
        .build()?;
    let model = client
        .completion_model(&request.model_name)
        .completions_api();
    let mut stream = model.stream(request.completion).await?;

    let mut snapshot = String::new();
    let mut tool_calls: Vec<ToolCall> = Vec::new();
    let mut reasoning: Vec<Reasoning> = Vec::new();
    let mut usage: Option<Usage> = None;

    while let Some(item) = stream.next().await {
        match item {
            Ok(StreamedAssistantContent::Text(text)) => {
                snapshot.push_str(&text.text);
                if sender
                    .send(Ok(GenerationEvent::Text {
                        delta: text.text.clone(),
                    }))
                    .await
                    .is_err()
                {
                    return Ok(StreamOutcome::ClientDisconnected {
                        snapshot,
                        tool_calls: if tool_calls.is_empty() {
                            None
                        } else {
                            Some(tool_calls)
                        },
                        reasoning: if reasoning.is_empty() {
                            None
                        } else {
                            Some(reasoning)
                        },
                        usage,
                    });
                }
            }
            Ok(StreamedAssistantContent::ToolCall { tool_call, .. }) => {
                tool_calls.push(tool_call);
            }
            Ok(StreamedAssistantContent::ToolCallDelta { .. }) => {}
            Ok(StreamedAssistantContent::Reasoning(reasoning_item)) => {
                push_reasoning(&mut reasoning, reasoning_item);
            }
            Ok(StreamedAssistantContent::ReasoningDelta {
                id,
                reasoning: delta,
            }) => {
                push_reasoning_delta(&mut reasoning, id, delta);
            }
            Ok(StreamedAssistantContent::Unknown(_)) => {}
            Ok(StreamedAssistantContent::Final(final_response)) => {
                usage = Some(final_response.token_usage());
            }
            Err(err) => return Err(Box::new(err)),
        }
    }

    let tool_calls_for_end = if tool_calls.is_empty() {
        None
    } else {
        Some(tool_calls.clone())
    };
    let reasoning_for_end = if reasoning.is_empty() {
        None
    } else {
        Some(reasoning.clone())
    };

    if snapshot.trim().is_empty() && tool_calls_for_end.is_none() {
        return Err(Box::new(std::io::Error::other(
            "Model returned an empty response",
        )));
    }

    if sender
        .send(Ok(GenerationEvent::End {
            snapshot: snapshot.clone(),
            tool_calls: tool_calls_for_end,
            reasoning: reasoning_for_end,
            usage,
        }))
        .await
        .is_err()
    {
        return Ok(StreamOutcome::ClientDisconnected {
            snapshot,
            tool_calls: if tool_calls.is_empty() {
                None
            } else {
                Some(tool_calls)
            },
            reasoning: if reasoning.is_empty() {
                None
            } else {
                Some(reasoning)
            },
            usage,
        });
    }

    Ok(StreamOutcome::Completed)
}

fn push_reasoning(reasoning: &mut Vec<Reasoning>, reasoning_item: Reasoning) {
    if let Some(id) = reasoning_item.id.as_deref() {
        reasoning.retain(|existing| existing.id.as_deref() != Some(id));
    }
    reasoning.push(reasoning_item);
}

fn push_reasoning_delta(reasoning: &mut Vec<Reasoning>, id: Option<String>, delta: String) {
    if let Some(existing) = reasoning
        .iter_mut()
        .find(|item| item.id == id && reasoning_ends_with_text(item))
    {
        if let Some(ReasoningContent::Text { text, .. }) = existing.content.last_mut() {
            text.push_str(&delta);
            return;
        }
    }

    reasoning.push(Reasoning::new(&delta).optional_id(id));
}

fn reasoning_ends_with_text(reasoning: &Reasoning) -> bool {
    matches!(
        reasoning.content.last(),
        Some(ReasoningContent::Text { .. })
    )
}
