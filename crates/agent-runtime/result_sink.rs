use async_trait::async_trait;
use db::{queries, ChatRole, ChatStatus, Pool};
use tool_runtime::{execute_tool_calls, ToolCall};

#[async_trait]
pub(crate) trait ResultSink: Send + Sync {
    async fn save(
        &self,
        snapshot: &str,
        tool_calls: Option<Vec<ToolCall>>,
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
        chat_id: i32,
        sub: &str,
        status: ChatStatus,
    ) {
        save_results_db(&self.pool, snapshot, tool_calls, chat_id, sub, status).await;
    }
}

async fn save_results_db(
    pool: &Pool,
    snapshot: &str,
    tool_calls: Option<Vec<ToolCall>>,
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
    let completion_tokens = crate::token_count::token_count_from_string(snapshot);

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

        if let Err(e) = queries::token_usage_metrics::create_token_usage_metric()
            .bind(
                &transaction,
                &Some(chat_id),
                &None::<i32>,
                &db::TokenUsageType::Completion,
                &completion_tokens,
                &None::<i32>,
            )
            .one()
            .await
        {
            tracing::error!("Error tracking completion tokens: {:?}", e);
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
                    let result_json = match serde_json::to_string(&tool_call.result) {
                        Ok(json) => json,
                        Err(e) => {
                            tracing::error!("Failed to serialize tool result: {:?}", e);
                            return;
                        }
                    };

                    let tool_chat_status = if tool_call.result.get("error").is_some() {
                        ChatStatus::Error
                    } else {
                        ChatStatus::Pending
                    };

                    if let Err(e) = queries::chats::new_chat()
                        .bind(
                            &transaction,
                            &chat.conversation_id,
                            &chat.prompt_id,
                            &Some(tool_call.id),
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
