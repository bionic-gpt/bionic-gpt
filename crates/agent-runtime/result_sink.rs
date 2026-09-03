use async_trait::async_trait;
use db::{queries, ChatRole, ChatStatus, Pool};
use rig::completion::Usage;
use tool_runtime::{
    execute_tool_calls, serialize_assistant_tool_state, Reasoning, ToolCall, ToolResultContent,
};

pub(crate) struct SaveRequest<'a> {
    pub(crate) snapshot: &'a str,
    pub(crate) tool_calls: Option<Vec<ToolCall>>,
    pub(crate) reasoning: Option<Vec<Reasoning>>,
    pub(crate) usage: Option<Usage>,
    pub(crate) chat_id: i32,
    pub(crate) sub: &'a str,
    pub(crate) status: ChatStatus,
}

#[async_trait]
pub(crate) trait ResultSink: Send + Sync {
    async fn save(&self, request: SaveRequest<'_>);
}

pub(crate) struct DbResultSink {
    pool: Pool,
}

impl DbResultSink {
    pub(crate) fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ResultSink for DbResultSink {
    async fn save(&self, request: SaveRequest<'_>) {
        save_results_db(&self.pool, request).await;
    }
}

async fn save_results_db(pool: &Pool, request: SaveRequest<'_>) {
    let SaveRequest {
        snapshot,
        tool_calls,
        reasoning,
        usage,
        chat_id,
        sub,
        status,
    } = request;

    let mut db_client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            tracing::error!("Error getting database client: {:?}", e);
            return;
        }
    };

    let transaction = match db_client.transaction().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Error starting transaction: {:?}", e);
            return;
        }
    };

    if let Err(e) = db::authz::set_row_level_security_user_id(&transaction, sub.to_string()).await {
        tracing::error!("Error setting row level security: {:?}", e);
        return;
    }

    if let Err(e) = queries::chats::set_chat_status()
        .bind(&transaction, &status, &chat_id)
        .await
    {
        tracing::error!("Error updating chat status: {:?}", e);
        return;
    }

    let mut executable_tool_calls = Vec::new();
    let mut failed_tool_calls = Vec::new();
    let mut stored_tool_calls = Vec::new();
    for mut tool_call in tool_calls.unwrap_or_default() {
        if let Some(error) = malformed_tool_call_error(&tool_call) {
            tracing::warn!(
                tool_name = %tool_call.function.name,
                tool_id = %tool_call.id,
                error = %error,
                "Preserving malformed tool call as a retryable tool error"
            );
            tool_call.function.arguments = serde_json::json!({});
            tool_call.additional_params = None;
            failed_tool_calls.push((tool_call.clone(), error));
        } else {
            executable_tool_calls.push(tool_call.clone());
        }
        stored_tool_calls.push(tool_call);
    }
    let tool_calls_json =
        serialize_assistant_tool_state(Some(stored_tool_calls.as_slice()), reasoning.as_deref());

    if let Ok(chat) = queries::chats::chat()
        .bind(&transaction, &chat_id)
        .one()
        .await
    {
        if status == ChatStatus::Success {
            if let Err(e) = transaction
                .execute(
                    "UPDATE llm.chats
                     SET status = 'Success'
                     WHERE status = 'Pending'
                     AND conversation_id = $1
                     AND role = 'Tool'",
                    &[&chat.conversation_id],
                )
                .await
            {
                tracing::error!("Error updating pending tool chats: {:?}", e);
                return;
            }
        }

        if let Err(e) = queries::chats::new_chat()
            .bind(
                &transaction,
                &chat.conversation_id,
                &chat.model_id,
                &None::<String>,
                &tool_calls_json,
                &snapshot,
                &ChatRole::Assistant,
                &status,
            )
            .one()
            .await
        {
            tracing::error!("Error creating chat: {:?}", e);
            return;
        }

        if status == ChatStatus::Success {
            let (prompt_tokens, completion_tokens) = usage
                .map(|u| (u.input_tokens as i32, u.output_tokens as i32))
                .unwrap_or_else(|| {
                    tracing::warn!("Missing provider token usage, storing zeros");
                    (0, 0)
                });

            let prompt_metric_result = queries::token_usage_metrics::create_token_usage_metric()
                .bind(
                    &transaction,
                    &Some(chat_id),
                    &None::<i32>,
                    &db::TokenUsageType::Prompt,
                    &prompt_tokens,
                    &None::<i32>,
                )
                .one()
                .await;
            if let Err(e) = prompt_metric_result {
                tracing::error!("Error tracking prompt tokens: {:?}", e);
            }

            let completion_metric_result =
                queries::token_usage_metrics::create_token_usage_metric()
                    .bind(
                        &transaction,
                        &Some(chat_id),
                        &None::<i32>,
                        &db::TokenUsageType::Completion,
                        &completion_tokens,
                        &None::<i32>,
                    )
                    .one()
                    .await;
            if let Err(e) = completion_metric_result {
                tracing::error!("Error tracking completion tokens: {:?}", e);
            }
        }

        if status == ChatStatus::Success {
            if !executable_tool_calls.is_empty() {
                let tool_call_results = execute_tool_calls(
                    executable_tool_calls,
                    pool,
                    sub.to_string(),
                    chat.conversation_id,
                    chat.model_id,
                )
                .await;

                for tool_call in tool_call_results {
                    let stored_tool_call_id = tool_call.call.to_string();
                    let result_json = match tool_call.content.first() {
                        Some(ToolResultContent::Text(text)) => text.text.clone(),
                        Some(ToolResultContent::Image(image)) => {
                            match serde_json::to_string(&serde_json::json!({ "image": image })) {
                                Ok(json) => json,
                                Err(e) => {
                                    tracing::error!(
                                        "Failed to serialize tool result image: {:?}",
                                        e
                                    );
                                    return;
                                }
                            }
                        }
                        Some(ToolResultContent::Json { value }) => value.to_string(),
                        None => String::new(),
                    };
                    if let Err(e) = queries::chats::new_chat()
                        .bind(
                            &transaction,
                            &chat.conversation_id,
                            &chat.model_id,
                            &Some(stored_tool_call_id.clone()),
                            &None::<String>,
                            &result_json,
                            &ChatRole::Tool,
                            &ChatStatus::Pending,
                        )
                        .one()
                        .await
                    {
                        tracing::error!("Error creating tool call results chat: {:?}", e);
                        return;
                    }
                }
            }

            for (tool_call, error) in failed_tool_calls {
                let result_json = serde_json::json!({
                    "error": format!("Malformed JSON arguments: {error}"),
                    "retryable": true
                })
                .to_string();
                if let Err(e) = queries::chats::new_chat()
                    .bind(
                        &transaction,
                        &chat.conversation_id,
                        &chat.model_id,
                        &Some(tool_call.id.to_string()),
                        &None::<String>,
                        &result_json,
                        &ChatRole::Tool,
                        &ChatStatus::Pending,
                    )
                    .one()
                    .await
                {
                    tracing::error!("Error creating malformed tool result chat: {:?}", e);
                    return;
                }
            }
        }
    } else {
        tracing::error!("Error retrieving chat");
    }

    if let Err(e) = transaction.commit().await {
        tracing::error!("Error committing transaction: {:?}", e);
    }
}

fn malformed_tool_call_error(tool_call: &ToolCall) -> Option<String> {
    tool_call
        .additional_params
        .as_ref()
        .and_then(|params| params.get("bionic_malformed_tool_call"))
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
}
