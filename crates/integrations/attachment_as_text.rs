use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::{Pool, Transaction};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use serde::Deserialize;
use serde_json::json;
use tracing;

/// Parameters for the read_attachment tool
#[derive(Debug, Deserialize)]
struct AttachmentAsTextParams {
    /// ID of the attachment to read
    file_id: i32,
    /// Byte offset at which to start reading (default 0)
    #[serde(default)]
    offset: usize,
    /// Maximum number of bytes to return
    max_bytes: Option<u64>,
}

/// A tool that provides access to read file attachments
pub struct AttachmentAsTextTool {
    pool: Pool,
    sub: Option<String>,
    conversation_id: Option<i64>,
}

impl AttachmentAsTextTool {
    pub fn new(pool: Pool, sub: Option<String>, conversation_id: Option<i64>) -> Self {
        tracing::debug!(
            "Creating new AttachmentAsTextTool with sub: {:?}, conversation_id: {:?}",
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
impl ToolInterface for AttachmentAsTextTool {
    fn get_tool(&self) -> BionicToolDefinition {
        tracing::debug!("Getting tool definition for AttachmentAsTextTool");
        get_attachment_as_text_tool()
    }

    #[tracing::instrument(skip(self, arguments), fields(conversation_id = ?self.conversation_id, sub = ?self.sub))]
    async fn execute(&self, arguments: &str) -> Result<String, String> {
        tracing::info!(
            "Executing attachment_as_text tool with arguments: {}",
            arguments
        );

        // Deserialize directly to our struct
        let params: AttachmentAsTextParams = match serde_json::from_str(arguments) {
            Ok(p) => {
                tracing::debug!("Successfully parsed arguments: {:?}", p);
                p
            }
            Err(e) => {
                tracing::error!("Failed to parse arguments: {}", e);
                return Err(format!("Failed to parse arguments: {}", e));
            }
        };

        // Create transaction
        tracing::debug!("Getting database client");
        let mut client = match self.pool.get().await {
            Ok(client) => {
                tracing::debug!("Successfully got database client");
                client
            }
            Err(e) => {
                tracing::error!("Failed to get database client: {}", e);
                return Err(format!("Failed to get client: {}", e));
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
                return Err(format!("Failed to create transaction: {}", e));
            }
        };

        // Set row-level security if sub is provided
        if let Some(sub) = &self.sub {
            tracing::debug!("Setting row-level security for user: {}", sub);
            if let Err(e) =
                db::authz::set_row_level_security_user_id(&transaction, sub.clone()).await
            {
                tracing::error!("Failed to set row level security: {}", e);
                return Err(format!("Failed to set row level security: {}", e));
            }
        }

        tracing::debug!(
            "Reading attachment with file_id: {}, offset: {}, max_bytes: {:?}",
            params.file_id,
            params.offset,
            params.max_bytes
        );

        let result = read_attachment(
            &transaction,
            params.file_id,
            params.offset,
            params.max_bytes,
        )
        .await;

        // Commit transaction
        tracing::debug!("Committing transaction");
        match transaction.commit().await {
            Ok(_) => tracing::debug!("Successfully committed transaction"),
            Err(e) => {
                tracing::error!("Failed to commit transaction: {}", e);
                return Err(format!("Failed to commit transaction: {}", e));
            }
        }

        match &result {
            Ok(_r) => tracing::info!("Attachment as text tool execution completed successfully"),
            Err(e) => tracing::warn!("Attachment as text tool execution failed: {}", e),
        }

        result
    }
}

/// Returns a Tool definition for the read_attachment tool
pub fn get_attachment_as_text_tool() -> BionicToolDefinition {
    tracing::trace!("Creating attachment_as_text tool definition");
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "attachment_as_text".to_string(),
            description: Some(
                "Return the text of an attachment, works well with text based attachments such as source code or .txt. \
                Use offset + max_bytes to limit the response and stay within context."
                    .to_string(),
            ),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "file_id": {
                        "type": "integer",
                        "description": "ID of the attachment to read this is the file_id in the results of a call to list_attachments"
                    },
                    "offset": {
                        "type": "integer",
                        "minimum": 0,
                        "description": "Byte offset at which to start reading (default 0)"
                    },
                    "max_bytes": {
                        "type": "integer",
                        "minimum": 1,
                        "description": "Maximum number of bytes to return"
                    }
                },
                "required": ["file_id"]
            })),
        },
    }
}

/// Read attachment content
#[tracing::instrument(skip(transaction))]
async fn read_attachment(
    transaction: &Transaction<'_>,
    id: i32,
    offset: usize,
    max_bytes: Option<u64>,
) -> Result<String, String> {
    tracing::info!(
        "Reading attachment id: {}, offset: {}, max_bytes: {:?}",
        id,
        offset,
        max_bytes
    );

    // Get attachment content
    tracing::debug!("Querying database for attachment content");
    let content = match db::queries::attachments::get_content()
        .bind(transaction, &id)
        .one()
        .await
    {
        Ok(content) => {
            tracing::debug!("Successfully retrieved attachment content");
            content
        }
        Err(e) => {
            tracing::error!("Failed to get attachment content: {}", e);
            return Err(format!("Failed to get attachment content: {}", e));
        }
    };

    // Get the bytes
    let bytes = content.object_data;
    tracing::debug!("Attachment total size: {} bytes", bytes.len());

    // Apply offset and max_bytes
    let start = std::cmp::min(offset, bytes.len());
    let end = if let Some(max) = max_bytes {
        std::cmp::min(start + max as usize, bytes.len())
    } else {
        bytes.len()
    };
    tracing::debug!(
        "Reading bytes from {} to {} (length: {})",
        start,
        end,
        end - start
    );

    let slice = &bytes[start..end];

    // Try to convert to UTF-8 text
    let text = match String::from_utf8(slice.to_vec()) {
        Ok(text) => {
            tracing::debug!("Successfully converted bytes to UTF-8 text");
            text
        }
        Err(_) => {
            tracing::debug!("Could not convert bytes to UTF-8, treating as binary data");
            format!("Binary data: {} bytes", slice.len())
        }
    };

    // Return as JSON
    let is_binary = text.starts_with("Binary data:");
    tracing::debug!("Attachment is binary: {}", is_binary);

    let result = json!({
        "content": text,
        "mime_type": content.mime_type,
        "is_binary": is_binary,
        "total_size": bytes.len(),
        "offset": start,
        "length": end - start
    });

    tracing::info!("Successfully read attachment: {} bytes", end - start);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_read_attachment_tool() {
        let tool = get_attachment_as_text_tool();
        assert_eq!(tool.function.name, "read_attachment");
    }
}
