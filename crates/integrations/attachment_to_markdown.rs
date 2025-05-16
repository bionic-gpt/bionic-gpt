use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::{Pool, Transaction};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use rag_engine::unstructured::document_to_markdown;
use serde::Deserialize;
use serde_json::json;
use tracing;

/// Parameters for the read_attachment tool
#[derive(Debug, Deserialize)]
struct AttachmentToMarkdownParams {
    /// ID of the attachment to read
    file_id: i32,
    /// Byte offset at which to start reading (default 0)
    #[serde(default)]
    offset: usize,
    /// Maximum number of bytes to return
    max_bytes: Option<u64>,
}

/// A tool that provides access to read file attachments
pub struct AttachmentToMarkdownTool {
    pool: Pool,
    sub: Option<String>,
    conversation_id: Option<i64>,
}

impl AttachmentToMarkdownTool {
    pub fn new(pool: Pool, sub: Option<String>, conversation_id: Option<i64>) -> Self {
        tracing::debug!(
            "Creating new AttachmentToMarkdownTool with sub: {:?}, conversation_id: {:?}",
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
impl ToolInterface for AttachmentToMarkdownTool {
    fn get_tool(&self) -> BionicToolDefinition {
        tracing::debug!("Getting tool definition for AttachmentToMarkdown Tool");
        get_attachment_to_markdown_tool()
    }

    #[tracing::instrument(skip(self, arguments), fields(conversation_id = ?self.conversation_id, sub = ?self.sub))]
    async fn execute(&self, arguments: &str) -> Result<String, String> {
        tracing::info!("Executing tool with arguments: {}", arguments);

        // Deserialize directly to our struct
        let params: AttachmentToMarkdownParams = match serde_json::from_str(arguments) {
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
            Ok(_r) => tracing::info!("Read attachment tool execution completed successfully"),
            Err(e) => tracing::warn!("Read attachment tool execution failed: {}", e),
        }

        result
    }
}

/// Returns a Tool definition for the attachment_to_markdown tool
pub fn get_attachment_to_markdown_tool() -> BionicToolDefinition {
    tracing::trace!("Creating attachment_to_markdown tool definition");
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "attachment_to_markdown".to_string(),
            description: Some(
                "Converts an attachment to markdown, works with PDF's and other documents. \
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
                        "description": "Character offset at which to start reading (default 0)"
                    },
                    "max_bytes": {
                        "type": "integer",
                        "minimum": 1,
                        "description": "Maximum number of UTF-8 characters to return"
                    }
                },
                "required": ["file_id"]
            })),
        },
    }
}

/// Helper function to convert mime type to file extension
fn mime_type_to_extension(mime_type: &str) -> String {
    match mime_type {
        "application/pdf" => "pdf",
        "text/plain" => "txt",
        "text/markdown" => "md",
        "text/html" => "html",
        "application/msword" => "doc",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => "docx",
        // Add more mime types as needed
        _ => "bin",
    }
    .to_string()
}

/// Read attachment content and convert to markdown
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

    // Extract filename from mime_type or use a default
    let file_name = format!(
        "attachment_{}.{}",
        id,
        mime_type_to_extension(&content.mime_type)
    );

    // Convert to markdown using document_to_markdown
    tracing::debug!("Converting attachment to markdown");
    match document_to_markdown(slice.to_vec(), &file_name).await {
        Ok(markdown) => {
            tracing::info!("Successfully converted attachment to markdown");
            Ok(markdown)
        }
        Err(e) => {
            tracing::error!("Failed to convert attachment to markdown: {}", e);

            // Fallback to original text conversion if markdown conversion fails
            let text = match String::from_utf8(slice.to_vec()) {
                Ok(text) => {
                    tracing::debug!("Falling back to UTF-8 text conversion");
                    text
                }
                Err(_) => {
                    tracing::debug!("Could not convert bytes to UTF-8, treating as binary data");
                    format!("Binary data: {} bytes", slice.len())
                }
            };

            Ok(text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_attachment_to_markdown_tool() {
        let tool = get_attachment_to_markdown_tool();
        assert_eq!(tool.function.name, "attachment_to_markdown");
    }
}
