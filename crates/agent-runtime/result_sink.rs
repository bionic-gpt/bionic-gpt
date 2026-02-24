use async_trait::async_trait;
use db::{queries, ChatRole, ChatStatus, Pool};
use rig::completion::Usage;
use tool_runtime::{execute_tool_calls, ToolCall, ToolResultContent};

#[async_trait]
pub(crate) trait ResultSink: Send + Sync {
    async fn save(
        &self,
        snapshot: &str,
        tool_calls: Option<Vec<ToolCall>>,
        usage: Option<Usage>,
        chat_id: i32,
        sub: &str,
        status: ChatStatus,
    );
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
    async fn save(
        &self,
        snapshot: &str,
        tool_calls: Option<Vec<ToolCall>>,
        usage: Option<Usage>,
        chat_id: i32,
        sub: &str,
        status: ChatStatus,
    ) {
        save_results_db(
            &self.pool, snapshot, tool_calls, usage, chat_id, sub, status,
        )
        .await;
    }
}

async fn save_results_db(
    pool: &Pool,
    snapshot: &str,
    tool_calls: Option<Vec<ToolCall>>,
    usage: Option<Usage>,
    chat_id: i32,
    sub: &str,
    status: ChatStatus,
) {
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

    let tool_calls_json = serde_json::to_string(&tool_calls).ok();

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
                &chat.prompt_id,
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
            if let Some(tool_calls) = tool_calls {
                let tool_call_results = execute_tool_calls(
                    tool_calls,
                    pool,
                    sub.to_string(),
                    chat.conversation_id,
                    chat.prompt_id,
                )
                .await;

                for tool_call in tool_call_results {
                    let tool_call_id = tool_call.id.clone();
                    let result_json = match tool_call.content.first() {
                        ToolResultContent::Text(text) => text.text,
                        ToolResultContent::Image(image) => {
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
                    };
                    let result_value = serde_json::from_str::<serde_json::Value>(&result_json)
                        .unwrap_or_else(|_| serde_json::json!({ "content": result_json }));

                    let tool_chat_status = if result_value.get("error").is_some() {
                        ChatStatus::Error
                    } else {
                        ChatStatus::Pending
                    };

                    if let Err(e) = queries::chats::new_chat()
                        .bind(
                            &transaction,
                            &chat.conversation_id,
                            &chat.prompt_id,
                            &Some(tool_call_id.clone()),
                            &None::<String>,
                            &result_json,
                            &ChatRole::Tool,
                            &tool_chat_status,
                        )
                        .one()
                        .await
                    {
                        tracing::error!("Error creating tool call results chat: {:?}", e);
                        return;
                    }
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
