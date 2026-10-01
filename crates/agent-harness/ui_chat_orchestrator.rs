use crate::chat_request::{create_request, RigChatRequest};
use crate::errors::CustomError;
use crate::jwt::Jwt;
pub(crate) use crate::result_sink::ResultSink;
use crate::result_sink::{DbResultSink, SaveRequest};
use crate::user_config::UserConfig;
use axum::response::{sse::Event, Sse};
use axum::Extension;
use db::{ChatStatus, Pool};
use rig::completion::Usage;
use rig::error::ProviderError;
use rig::message::{
    AssistantContent, Issuer, Reasoning as RigReasoning, ReasoningContent, ToolFunction, ToolName,
};
use rig::providers::{ollama, openai};
use rig::streaming::{CompletionStream, Item, StreamEvent};
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

/// Runs a persisted chat to completion without requiring an HTTP client to drive
/// the tool-call continuation loop. This is used by scheduled task workers.
pub async fn run_scheduled_chat(
    pool: Pool,
    current_user: Jwt,
    chat_id: i32,
    max_turns: usize,
) -> Result<(), String> {
    let result_sink: Arc<dyn ResultSink> = Arc::new(DbResultSink::new(pool.clone()));
    let user_config = UserConfig::default();

    for turn in 0..max_turns {
        let request = create_request(&pool, &current_user, chat_id, &user_config)
            .await
            .map_err(|error| error.to_string())?;

        if limits::is_limit_exceeded_from_pool(&pool, request.model_id, request.user_id)
            .await
            .map_err(|error| error.to_string())?
        {
            let message = "You have exceeded your token limit for this model";
            result_sink
                .save(SaveRequest {
                    snapshot: message,
                    tool_calls: None,
                    reasoning: None,
                    usage: None,
                    chat_id,
                    sub: &current_user.sub,
                    status: ChatStatus::Error,
                })
                .await;
            return Err(message.to_string());
        }

        let (sender, mut receiver) = mpsc::channel::<Result<GenerationEvent, axum::Error>>(32);
        let generation = tokio::spawn(async move { stream_chat_with_rig(request, sender).await });
        let mut completed = None;
        while let Some(event) = receiver.recv().await {
            match event {
                Ok(GenerationEvent::End {
                    snapshot,
                    tool_calls,
                    reasoning,
                    usage,
                }) => {
                    completed = Some((snapshot, tool_calls, reasoning, usage));
                }
                Ok(GenerationEvent::Text { .. }) => {}
                Err(error) => return Err(error.to_string()),
            }
        }

        generation
            .await
            .map_err(|error| error.to_string())?
            .map_err(|error| error.to_string())?;

        let Some((snapshot, tool_calls, reasoning, usage)) = completed else {
            return Err("model returned no completion event".to_string());
        };
        let has_tool_calls = tool_calls.is_some();
        result_sink
            .save(SaveRequest {
                snapshot: &snapshot,
                tool_calls,
                reasoning,
                usage,
                chat_id,
                sub: &current_user.sub,
                status: ChatStatus::Success,
            })
            .await;

        if !has_tool_calls {
            return Ok(());
        }

        tracing::debug!(
            chat_id,
            turn = turn + 1,
            "Continuing scheduled tool call turn"
        );
    }

    Err(format!("scheduled chat exceeded {max_turns} model turns"))
}

/// Executes a streaming rig completion and publishes intermediate events.
pub(crate) async fn stream_chat_with_rig(
    request: RigChatRequest,
    sender: mpsc::Sender<Result<GenerationEvent, axum::Error>>,
) -> Result<StreamOutcome, Box<dyn std::error::Error + Send + Sync>> {
    tracing::debug!(
        provider = ?request.provider_type,
        model = %request.model_name,
        base_url = %request.base_url,
        tool_count = request.completion.tools.len(),
        stream = true,
        "Starting model stream"
    );
    let api_key = request.api_key.as_deref().unwrap_or("");
    let stream = match request.provider_type {
        db::ModelProvider::OpenAI
        | db::ModelProvider::OpenAICompatible
        | db::ModelProvider::Groq
        | db::ModelProvider::OpenRouter => {
            let dialect = match request.provider_type {
                db::ModelProvider::Groq => &openai::wire::GROQ,
                db::ModelProvider::OpenRouter => &openai::wire::OPENROUTER,
                _ => &openai::wire::OPENAI,
            };
            openai::OpenAIConfig::with_key(dialect, api_key)
                .with_base_url(&request.base_url)
                .client()
                .chat(&request.model_name)
                .erase()
                .stream(request.completion)?
        }
        db::ModelProvider::Ollama => ollama::OllamaConfig::new()
            .with_api_key(api_key)
            .with_base_url(ollama_base_url(&request.base_url))
            .client()
            .completion(&request.model_name)
            .erase()
            .stream(request.completion)?,
    };
    consume_rig_stream(stream, sender).await
}

fn ollama_base_url(base_url: &str) -> &str {
    base_url
        .trim_end_matches('/')
        .strip_suffix("/v1")
        .unwrap_or_else(|| base_url.trim_end_matches('/'))
}

async fn consume_rig_stream(
    mut stream: CompletionStream,
    sender: mpsc::Sender<Result<GenerationEvent, axum::Error>>,
) -> Result<StreamOutcome, Box<dyn std::error::Error + Send + Sync>> {
    let mut snapshot = String::new();
    let mut tool_calls: Vec<ToolCall> = Vec::new();
    let mut reasoning: Vec<RigReasoning> = Vec::new();
    let reasoning_issuer = stream.reasoning_issuer();
    let mut usage: Option<Usage> = None;
    let mut text_event_count = 0;
    let mut tool_call_delta_count = 0;
    let mut final_event_received = false;
    let mut malformed_tool_input_received = false;
    let mut unknown_event_count = 0;
    let mut event_count = 0;
    let mut text_parts_with_deltas = std::collections::HashSet::new();

    while let Some(item) = stream.next().await {
        event_count += 1;
        match item {
            Ok(Item::Event(StreamEvent::Text { part, text })) => {
                text_event_count += 1;
                tracing::debug!(
                    event = event_count,
                    delta_length = text.len(),
                    "Rig stream text event"
                );
                if !text.is_empty() {
                    text_parts_with_deltas.insert(part.index());
                }
                snapshot.push_str(&text);
                if sender
                    .send(Ok(GenerationEvent::Text { delta: text }))
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
                        reasoning: seal_reasoning(&reasoning, &reasoning_issuer),
                        usage,
                    });
                }
            }
            Ok(Item::Event(StreamEvent::Arguments { .. })) => {
                tool_call_delta_count += 1;
                tracing::debug!(event = event_count, "Rig stream tool-call argument delta");
            }
            Ok(Item::Event(StreamEvent::Reasoning { part, text })) => {
                tracing::debug!(
                    event = event_count,
                    delta_length = text.len(),
                    "Rig stream reasoning event"
                );
                push_reasoning_delta(&mut reasoning, Some(part.index().to_string()), text);
            }
            Ok(Item::Event(StreamEvent::End { part, content })) => match content {
                AssistantContent::ToolCall(tool_call) => {
                    tracing::debug!(
                        event = event_count,
                        tool_name = %tool_call.function.name,
                        tool_id = %tool_call.id,
                        "Rig stream complete tool-call event"
                    );
                    tool_calls.push(tool_call);
                }
                AssistantContent::Text(text) => {
                    if !text_parts_with_deltas.contains(&part.index()) {
                        snapshot.push_str(&text.text);
                        if sender
                            .send(Ok(GenerationEvent::Text { delta: text.text }))
                            .await
                            .is_err()
                        {
                            return Ok(StreamOutcome::ClientDisconnected {
                                snapshot,
                                tool_calls: (!tool_calls.is_empty()).then_some(tool_calls),
                                reasoning: seal_reasoning(&reasoning, &reasoning_issuer),
                                usage,
                            });
                        }
                    }
                }
                AssistantContent::Reasoning(sealed) => {
                    if let Some(mut item) = sealed.open(&reasoning_issuer).cloned() {
                        if item.id.is_none() {
                            item.id = Some(part.index().to_string());
                        }
                        push_reasoning(&mut reasoning, item);
                    }
                }
                _ => {}
            },
            Ok(Item::Event(StreamEvent::Start { .. })) => {}
            Ok(Item::Unknown(_)) => {
                unknown_event_count += 1;
                tracing::debug!(event = event_count, "Rig stream unknown event");
            }
            Err(ProviderError::MalformedToolInput(input)) => {
                tracing::warn!(
                    event = event_count,
                    tool_name = %input.name,
                    tool_id = %input.id,
                    error = %input.error,
                    "Preserving malformed streamed tool call as a retryable tool error"
                );
                let additional_params = json!({
                    "bionic_malformed_tool_call": input.error,
                    "bionic_malformed_tool_call_raw": input.raw,
                });
                tool_calls.push(
                    ToolCall::new(
                        input.id,
                        ToolFunction::new(ToolName::new(input.name)?, json!({})),
                    )
                    .with_additional_params(Some(additional_params)),
                );
                malformed_tool_input_received = true;
                break;
            }
            Err(err) => {
                tracing::error!(event = event_count, error = %err, "Rig stream item failed");
                return Err(Box::new(err));
            }
        }
    }

    if !malformed_tool_input_received {
        let final_response = stream.finish().await?;
        final_event_received = true;
        usage = Some(final_response.usage);
    }

    tracing::debug!(
        event_count,
        text_event_count,
        text_length = snapshot.len(),
        tool_call_count = tool_calls.len(),
        tool_call_delta_count,
        reasoning_count = reasoning.len(),
        final_event_received,
        unknown_event_count,
        "Rig model stream ended"
    );

    let tool_calls_for_end = if tool_calls.is_empty() {
        None
    } else {
        Some(tool_calls.clone())
    };
    let reasoning_for_end = seal_reasoning(&reasoning, &reasoning_issuer);

    if snapshot.trim().is_empty() && tool_calls_for_end.is_none() {
        tracing::warn!(
            text_event_count,
            text_length = snapshot.len(),
            tool_call_count = tool_calls.len(),
            tool_call_delta_count,
            reasoning_count = reasoning.len(),
            final_event_received,
            unknown_event_count,
            "Model stream produced no user-visible text or complete tool call"
        );
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
            reasoning: seal_reasoning(&reasoning, &reasoning_issuer),
            usage,
        });
    }

    Ok(StreamOutcome::Completed)
}

fn push_reasoning(reasoning: &mut Vec<RigReasoning>, reasoning_item: RigReasoning) {
    if let Some(id) = reasoning_item.id.as_deref() {
        reasoning.retain(|existing| existing.id.as_deref() != Some(id));
    }
    reasoning.push(reasoning_item);
}

fn seal_reasoning(reasoning: &[RigReasoning], issuer: &Issuer) -> Option<Vec<Reasoning>> {
    (!reasoning.is_empty()).then(|| {
        reasoning
            .iter()
            .cloned()
            .map(|item| item.sealed(issuer.clone()))
            .collect()
    })
}

fn push_reasoning_delta(reasoning: &mut Vec<RigReasoning>, id: Option<String>, delta: String) {
    if let Some(existing) = reasoning
        .iter_mut()
        .find(|item| item.id == id && reasoning_ends_with_text(item))
    {
        if let Some(ReasoningContent::Text { text, .. }) = existing.content.last_mut() {
            text.push_str(&delta);
            return;
        }
    }

    let mut item = RigReasoning::new(&delta);
    item.id = id;
    reasoning.push(item);
}

fn reasoning_ends_with_text(reasoning: &RigReasoning) -> bool {
    matches!(
        reasoning.content.last(),
        Some(ReasoningContent::Text { .. })
    )
}
