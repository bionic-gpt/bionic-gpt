use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::{Pool, Transaction};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use serde_json::json;
use tracing;

/// A tool that provides access to list file attachments
pub struct ListAttachmentsTool {
    pool: Pool,
    sub: Option<String>,
    conversation_id: Option<i64>,
}

impl ListAttachmentsTool {
    pub fn new(pool: Pool, sub: Option<String>, conversation_id: Option<i64>) -> Self {
        tracing::debug!(
            "Creating new ListAttachmentsTool with sub: {:?}, conversation_id: {:?}",
            sub,
            conversation_id
        );
        Self {
            pool,
            sub,
            conversation_id,
        }
    }
}

#[async_trait]
impl ToolInterface for ListAttachmentsTool {
    fn get_tool(&self) -> BionicToolDefinition {
        tracing::debug!("Getting tool definition for ListAttachmentsTool");
        get_list_attachments_tool()
    }

    #[tracing::instrument(skip(self, arguments), fields(conversation_id = ?self.conversation_id, sub = ?self.sub))]
    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        tracing::info!(
            "Executing list_attachments tool with arguments: {}",
            arguments
        );

        // Create transaction
        tracing::debug!("Getting database client");
        let mut client = match self.pool.get().await {
            Ok(client) => {
                tracing::debug!("Successfully got database client");
                client
            }
            Err(e) => {
                tracing::error!("Failed to get database client: {}", e);
                return Err(serde_json::json!({
                    "error": "Failed to get database client",
                    "details": e.to_string()
                }));
            }
        };

        tracing::debug!("Creating transaction");
        let transaction = match client.transaction().await {
            Ok(transaction) => {
                tracing::debug!("Successfully created transaction");
                transaction
            }
            Err(e) => {
                tracing::error!("Failed to create transaction: {}", e);
                return Err(serde_json::json!({
                    "error": "Failed to create transaction",
                    "details": e.to_string()
                }));
            }
        };

        // Set row-level security if sub is provided
        if let Some(sub) = &self.sub {
            tracing::debug!("Setting row-level security for user: {}", sub);
            if let Err(e) =
                db::authz::set_row_level_security_user_id(&transaction, sub.clone()).await
            {
                tracing::error!("Failed to set row level security: {}", e);
                return Err(serde_json::json!({
                    "error": "Failed to set row level security",
                    "details": e.to_string()
                }));
            }
        }

        // Use the provided conversation_id
        let result = if let Some(conv_id) = self.conversation_id {
            tracing::debug!("Listing attachments for conversation: {}", conv_id);
            list_attachments(&transaction, conv_id).await
        } else {
            tracing::warn!("Missing conversation_id for list_attachments");
            Err(serde_json::json!({
                "error": "Missing conversation_id"
            }))
        };

        // Commit transaction
        tracing::debug!("Committing transaction");
        match transaction.commit().await {
            Ok(_) => tracing::debug!("Successfully committed transaction"),
            Err(e) => {
                tracing::error!("Failed to commit transaction: {}", e);
                return Err(serde_json::json!({
                    "error": "Failed to commit transaction",
                    "details": e.to_string()
                }));
            }
        }

        match &result {
            Ok(_r) => tracing::info!("List attachments tool execution completed successfully"),
            Err(e) => tracing::warn!("List attachments tool execution failed: {}", e),
        }

        result
    }
}

/// Returns a Tool definition for the list_attachments tool
pub fn get_list_attachments_tool() -> BionicToolDefinition {
    tracing::trace!("Creating list_attachments tool definition");
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "list_attachments".to_string(),
            description: Some("Return metadata for all files the user has attached".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {},
                "required": []
            })),
        },
    }
}

/// List all attachments
#[tracing::instrument(skip(transaction))]
async fn list_attachments(
    transaction: &Transaction<'_>,
    conversation_id: i64,
) -> Result<serde_json::Value, serde_json::Value> {
    tracing::info!("Listing attachments for conversation: {}", conversation_id);

    // Get all attachments
    tracing::debug!("Querying database for attachments");
    let attachments = match db::queries::attachments::get_by_conversation()
        .bind(transaction, &conversation_id)
        .all()
        .await
    {
        Ok(attachments) => {
            tracing::debug!("Found {} attachments", attachments.len());
            attachments
        }
        Err(e) => {
            tracing::error!("Failed to get attachments: {}", e);
            return Err(serde_json::json!({
                "error": "Failed to get attachments",
                "details": e.to_string()
            }));
        }
    };

    // Convert to JSON
    tracing::debug!("Converting attachments to JSON");
    let result = json!({
        "attachments": attachments.iter().map(|a| {
            tracing::debug!("Processing attachment: id={}, name={}, size={}", a.id, a.file_name, a.file_size);
            json!({
                "file_id": a.id,
                "name": a.file_name,
                "size": a.file_size,
                "mime_type": a.mime_type
            })
        }).collect::<Vec<_>>()
    });

    tracing::info!("Successfully listed {} attachments", attachments.len());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_list_attachments_tool() {
        let tool = get_list_attachments_tool();
        assert_eq!(tool.function.name, "list_attachments");
    }
}
