use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::{Pool, Transaction};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use serde_json::{json, Value};

/// A tool that provides access to file attachments
pub struct AttachmentsTool {
    pool: Pool,
    sub: Option<String>,
    conversation_id: Option<i64>,
}

impl AttachmentsTool {
    pub fn new(pool: Pool, sub: Option<String>, conversation_id: Option<i64>) -> Self {
        Self {
            pool,
            sub,
            conversation_id,
        }
    }
}

#[async_trait]
impl ToolInterface for AttachmentsTool {
    fn get_tool(&self) -> BionicToolDefinition {
        // Return the list_attachments tool definition
        // Note: This means each AttachmentsTool instance only handles one tool
        // In a real implementation, we might want to handle multiple tools
        get_list_attachments_tool()
    }

    async fn execute(&self, arguments: &str) -> Result<String, String> {
        // Pass sub and conversation_id to execute_attachments_tool
        execute_attachments_tool(
            arguments,
            &self.pool,
            self.sub.clone(),
            self.conversation_id,
        )
        .await
    }
}

/// Returns a Tool definition for the list_attachments tool
pub fn get_list_attachments_tool() -> BionicToolDefinition {
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

/// Returns a Tool definition for the read_attachment tool
pub fn get_read_attachment_tool() -> BionicToolDefinition {
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "read_attachment".to_string(),
            description: Some(
                "Return raw bytes (UTF-8 text if decodable) from an attachment. \
                Use offset + max_bytes to limit the response and stay within context."
                    .to_string(),
            ),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "file_id": {
                        "type": "string",
                        "description": "ID of the attachment to read"
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

/// Execute the attachments tool with the given arguments
pub async fn execute_attachments_tool(
    arguments: &str,
    pool: &Pool,
    sub: Option<String>,
    conversation_id: Option<i64>,
) -> Result<String, String> {
    let args: Value =
        serde_json::from_str(arguments).map_err(|e| format!("Failed to parse arguments: {}", e))?;

    // Determine which function to call based on the function name in the arguments
    let function_name = args["name"].as_str().unwrap_or("");

    // Create transaction
    let mut client = pool
        .get()
        .await
        .map_err(|e| format!("Failed to get client: {}", e))?;

    let transaction = client
        .transaction()
        .await
        .map_err(|e| format!("Failed to create transaction: {}", e))?;

    // Set row-level security if sub is provided
    if let Some(sub) = sub {
        db::authz::set_row_level_security_user_id(&transaction, sub)
            .await
            .map_err(|e| format!("Failed to set row level security: {}", e))?;
    }

    // Execute the appropriate function
    let result = match function_name {
        "list_attachments" => {
            // Use the provided conversation_id
            if let Some(conv_id) = conversation_id {
                list_attachments(&transaction, conv_id).await
            } else {
                Err("Missing conversation_id".to_string())
            }
        }
        "read_attachment" => {
            let file_id = args["file_id"]
                .as_str()
                .ok_or_else(|| "Missing file_id parameter".to_string())?;

            let file_id = file_id
                .parse::<i32>()
                .map_err(|e| format!("Invalid file_id: {}", e))?;

            let offset = args["offset"].as_u64().unwrap_or(0) as usize;
            let max_bytes = args["max_bytes"].as_u64();

            read_attachment(&transaction, file_id, offset, max_bytes).await
        }
        _ => Err(format!("Unknown function: {}", function_name)),
    };

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    result
}

/// List all attachments
async fn list_attachments(
    transaction: &Transaction<'_>,
    conversation_id: i64,
) -> Result<String, String> {
    // Get all attachments
    let attachments = db::queries::attachments::get_by_conversation()
        .bind(transaction, &conversation_id)
        .all()
        .await
        .map_err(|e| format!("Failed to get attachments: {}", e))?;

    // Convert to JSON
    let result = json!({
        "attachments": attachments.iter().map(|a| {
            json!({
                "id": a.id,
                "name": a.file_name,
                "size": a.file_size,
                "mime_type": a.mime_type
            })
        }).collect::<Vec<_>>()
    });

    Ok(result.to_string())
}

/// Read attachment content
async fn read_attachment(
    transaction: &Transaction<'_>,
    id: i32,
    offset: usize,
    max_bytes: Option<u64>,
) -> Result<String, String> {
    // Get attachment content
    let content = db::queries::attachments::get_content()
        .bind(transaction, &id)
        .one()
        .await
        .map_err(|e| format!("Failed to get attachment content: {}", e))?;

    // Get the bytes
    let bytes = content.object_data;

    // Apply offset and max_bytes
    let start = std::cmp::min(offset, bytes.len());
    let end = if let Some(max) = max_bytes {
        std::cmp::min(start + max as usize, bytes.len())
    } else {
        bytes.len()
    };

    let slice = &bytes[start..end];

    // Try to convert to UTF-8 text
    let text = String::from_utf8(slice.to_vec())
        .unwrap_or_else(|_| format!("Binary data: {} bytes", slice.len()));

    // Return as JSON
    let result = json!({
        "content": text,
        "mime_type": content.mime_type,
        "is_binary": text.starts_with("Binary data:"),
        "total_size": bytes.len(),
        "offset": start,
        "length": end - start
    });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_list_attachments_tool() {
        let tool = get_list_attachments_tool();
        assert_eq!(tool.function.name, "list_attachments");
    }

    #[test]
    fn test_get_read_attachment_tool() {
        let tool = get_read_attachment_tool();
        assert_eq!(tool.function.name, "read_attachment");
    }
}
