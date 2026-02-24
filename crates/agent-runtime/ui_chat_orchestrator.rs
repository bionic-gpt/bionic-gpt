use crate::errors::CustomError;
use crate::jwt::Jwt;
use crate::moderation::{moderate_chat, strip_tool_data, ModerationVerdict};
use crate::result_sink::DbResultSink;
pub(crate) use crate::result_sink::ResultSink;
use crate::user_config::UserConfig;
use axum::response::{sse::Event, Sse};
use axum::Extension;
use db::{queries, ChatRole, ChatStatus, Pool};
use rig::client::CompletionClient;
use rig::completion::{CompletionModel as _, CompletionRequest, Message as RigMessage};
use rig::providers::openai;
use rig::streaming::StreamedAssistantContent;
use rig::OneOrMany;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tool_runtime::{
    get_chat_tools_user_selected_with_system_openapi, get_tools, ToolCall, ToolScope,
};

use super::{limits, UICompletions};

fn event_data_for_text(delta: String) -> String {
    json!({
        "type": "text_delta",
        "data": {
            "delta": delta
        }
    })
    .to_string()
}

fn event_data_for_done() -> String {
    json!({
        "type": "done",
        "data": {}
    })
    .to_string()
}

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
    },
}

#[derive(Debug)]
enum StreamOutcome {
    Completed,
    ClientDisconnected {
        snapshot: String,
        tool_calls: Option<Vec<ToolCall>>,
    },
}

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
                    } => {
                        result_sink
                            .save(&snapshot, tool_calls, chat_id, &sub, ChatStatus::Success)
                            .await;
                        Ok(Event::default().data(event_data_for_done()))
                    }
                },
                Err(e) => {
                    let message = e.to_string();
                    result_sink
                        .save(&message, None, chat_id, &sub, ChatStatus::Error)
                        .await;
                    Ok(event_data_for_error(message))
                }
            }
        }
    })
}

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
                    let _ = sender
                        .send(Err(axum::Error::new(std::io::Error::other(limit_message))))
                        .await;
                    result_sink_clone
                        .save(
                            limit_message,
                            None,
                            chat_id,
                            &sub_for_save,
                            ChatStatus::Error,
                        )
                        .await;
                    return;
                }

                match stream_chat_with_rig(request, sender).await {
                    Ok(StreamOutcome::Completed) => {}
                    Ok(StreamOutcome::ClientDisconnected {
                        snapshot,
                        tool_calls,
                    }) => {
                        result_sink_clone
                            .save(
                                &snapshot,
                                tool_calls,
                                chat_id,
                                &sub_for_save,
                                ChatStatus::Error,
                            )
                            .await;
                    }
                    Err(err) => {
                        let err_msg = err.to_string();
                        tracing::error!("Error generating SSE stream: {}", err_msg);
                        result_sink_clone
                            .save(&err_msg, None, chat_id, &sub_for_save, ChatStatus::Error)
                            .await;
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
                .save(
                    &err.to_string(),
                    None,
                    chat_id,
                    &current_user.sub,
                    ChatStatus::Error,
                )
                .await;
            Err(CustomError::FaultySetup(err.to_string()))
        }
    }
}

struct RigChatRequest {
    model_name: String,
    base_url: String,
    api_key: Option<String>,
    completion: CompletionRequest,
    model_id: i32,
    user_id: i32,
}

async fn stream_chat_with_rig(
    request: RigChatRequest,
    sender: mpsc::Sender<Result<GenerationEvent, axum::Error>>,
) -> Result<StreamOutcome, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = request.api_key.as_deref().unwrap_or("");
    let client = openai::Client::builder(api_key)
        .base_url(&request.base_url)
        .build();
    let model = client.completion_model(&request.model_name);
    let mut stream = model.stream(request.completion).await?;

    let mut snapshot = String::new();
    let mut tool_calls: Vec<ToolCall> = Vec::new();

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
                    });
                }
            }
            Ok(StreamedAssistantContent::ToolCall(tool_call)) => {
                let arguments = serde_json::to_string(&tool_call.function.arguments)
                    .unwrap_or_else(|_| "{}".to_string());
                tool_calls.push(ToolCall {
                    id: tool_call.id,
                    index: None,
                    r#type: "function".to_string(),
                    function: tool_runtime::ToolCallFunction {
                        name: tool_call.function.name,
                        arguments,
                    },
                });
            }
            Ok(StreamedAssistantContent::ToolCallDelta { .. }) => {}
            Ok(StreamedAssistantContent::Reasoning(_)) => {}
            Ok(StreamedAssistantContent::Final(_)) => {}
            Err(err) => return Err(Box::new(err)),
        }
    }

    let tool_calls_for_end = if tool_calls.is_empty() {
        None
    } else {
        Some(tool_calls.clone())
    };

    if sender
        .send(Ok(GenerationEvent::End {
            snapshot: snapshot.clone(),
            tool_calls: tool_calls_for_end,
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
        });
    }

    Ok(StreamOutcome::Completed)
}

fn to_rig_tools(tools: Vec<tool_runtime::ToolDefinition>) -> Vec<rig::completion::ToolDefinition> {
    tools
        .into_iter()
        .map(|tool| rig::completion::ToolDefinition {
            name: tool.function.name,
            description: tool.function.description,
            parameters: tool.function.parameters,
        })
        .collect()
}

async fn create_request(
    pool: &Pool,
    current_user: &Jwt,
    chat_id: i32,
    user_config: &UserConfig,
) -> Result<RigChatRequest, CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;
    db::authz::set_row_level_security_user_id(&transaction, current_user.sub.to_string()).await?;

    let model = queries::models::model_host_by_chat_id()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    let capabilities = queries::capabilities::get_model_capabilities()
        .bind(&transaction, &model.id)
        .all()
        .await?;

    let chat = queries::chats::chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    let conversation = queries::conversations::get_conversation_from_chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    let attachment_count = queries::conversations::count_attachments()
        .bind(&transaction, &conversation.id)
        .one()
        .await?;

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &chat.prompt_id, &conversation.team_id)
        .one()
        .await?;

    let chat_history = queries::chats::chat_history()
        .bind(
            &transaction,
            &conversation.id,
            &(prompt.max_history_items as i64),
        )
        .all()
        .await?;

    let chat_history = super::context_builder::convert_chat_to_messages(chat_history);

    let messages = super::context_builder::execute_prompt(
        &transaction,
        prompt.clone(),
        Some(conversation.id),
        chat_history,
    )
    .await?;

    let size = crate::token_count::token_count(messages.clone());

    queries::token_usage_metrics::create_token_usage_metric()
        .bind(
            &transaction,
            &Some(chat_id),
            &None::<i32>,
            &db::TokenUsageType::Prompt,
            &size,
            &None::<i32>,
        )
        .one()
        .await?;

    queries::chats::set_chat_status()
        .bind(&transaction, &ChatStatus::InProgress, &chat_id)
        .await?;

    let tools = if capabilities
        .iter()
        .any(|c| c.capability == db::ModelCapability::tool_use)
    {
        let mut all_tools = get_chat_tools_user_selected_with_system_openapi(
            pool,
            user_config.enabled_tools.as_ref(),
        )
        .await;

        if attachment_count > 0 {
            all_tools.extend(get_tools(ToolScope::DocumentIntelligence));
        }

        if let Ok(integration_tools) =
            super::context_builder::get_prompt_integration_tools(&transaction, prompt.id).await
        {
            all_tools.extend(integration_tools);
        }

        if all_tools.is_empty() {
            None
        } else {
            Some(all_tools)
        }
    } else {
        None
    };

    if capabilities
        .iter()
        .any(|c| c.capability == db::ModelCapability::Guarded)
    {
        let guard_model = queries::models::models()
            .bind(&transaction, &db::ModelType::Guard)
            .one()
            .await?;

        let sanitized = strip_tool_data(&messages);
        match moderate_chat(
            &guard_model.base_url,
            guard_model.api_key.as_deref(),
            &guard_model.name,
            sanitized,
        )
        .await
        {
            Ok(ModerationVerdict::Safe) => {}
            Ok(ModerationVerdict::Unsafe(code)) => {
                queries::chats::new_chat()
                    .bind(
                        &transaction,
                        &conversation.id,
                        &chat.prompt_id,
                        &None::<String>,
                        &None::<String>,
                        &"Your question violated our guidelines",
                        &ChatRole::Assistant,
                        &ChatStatus::Error,
                    )
                    .one()
                    .await?;
                queries::prompt_flags::insert_prompt_flag()
                    .bind(&transaction, &chat_id, &code)
                    .await?;
                transaction.commit().await?;
                return Err(CustomError::FaultySetup("Moderation failed".into()));
            }
            Err(status) => {
                transaction.commit().await?;
                return Err(CustomError::FaultySetup(format!(
                    "Moderation failed: {status}"
                )));
            }
        }
    }

    transaction.commit().await?;

    let completion = CompletionRequest {
        preamble: None,
        chat_history: OneOrMany::many(messages)
            .unwrap_or_else(|_| OneOrMany::one(RigMessage::user(""))),
        documents: vec![],
        tools: tools.map(to_rig_tools).unwrap_or_default(),
        temperature: prompt.temperature.map(|t| t as f64),
        max_tokens: prompt.max_completion_tokens.map(|t| t as u64),
        tool_choice: None,
        additional_params: None,
    };

    Ok(RigChatRequest {
        model_name: model.name,
        base_url: model.base_url,
        api_key: model.api_key,
        completion,
        model_id: model.id,
        user_id: conversation.user_id,
    })
}
