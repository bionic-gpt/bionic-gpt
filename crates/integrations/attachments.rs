use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::{Pool, Transaction};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use serde_json::{json, Value};
use tracing::{debug, error, info, instrument, trace, warn};

/// A tool that provides access to file attachments
pub struct AttachmentsTool {
    pool: Pool,
    sub: Option<String>,
    conversation_id: Option<i64>,
}

impl AttachmentsTool {
    pub fn new(pool: Pool, sub: Option<String>, conversation_id: Option<i64>) -> Self {
        debug!(
            "Creating new AttachmentsTool with sub: {:?}, conversation_id: {:?}",
            sub, conversation_id
        );
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
        debug!("Getting tool definition for AttachmentsTool");
        // Return the list_attachments tool definition
        // Note: This means each AttachmentsTool instance only handles one tool
        // In a real implementation, we might want to handle multiple tools
        get_list_attachments_tool()
    }

    #[instrument(skip(self, arguments), fields(conversation_id = ?self.conversation_id, sub = ?self.sub))]
    async fn execute(&self, arguments: &str) -> Result<String, String> {
        info!("Executing attachment tool with arguments: {}", arguments);
        // Pass sub and conversation_id to execute_attachments_tool
        let result = execute_attachments_tool(
            arguments,
            &self.pool,
            self.sub.clone(),
            self.conversation_id,
        )
        .await;

        match &result {
            Ok(_) => debug!("Attachment tool execution successful"),
            Err(e) => error!("Attachment tool execution failed: {}", e),
        }

        result
    }
}

/// Returns a Tool definition for the list_attachments tool
#[instrument]
pub fn get_list_attachments_tool() -> BionicToolDefinition {
    trace!("Creating list_attachments tool definition");
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
#[instrument]
pub fn get_read_attachment_tool() -> BionicToolDefinition {
    trace!("Creating read_attachment tool definition");
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
#[instrument(skip(pool), fields(sub = ?sub, conversation_id = ?conversation_id))]
pub async fn execute_attachments_tool(
    arguments: &str,
    pool: &Pool,
    sub: Option<String>,
    conversation_id: Option<i64>,
) -> Result<String, String> {
    info!("Executing attachment tool with arguments: {}", arguments);

    let args: Value = match serde_json::from_str(arguments) {
        Ok(v) => {
            debug!("Successfully parsed arguments");
            v
        }
        Err(e) => {
            error!("Failed to parse arguments: {}", e);
            return Err(format!("Failed to parse arguments: {}", e));
        }
    };

    // Determine which function to call based on the function name in the arguments
    let function_name = args["name"].as_str().unwrap_or("");
    debug!("Function name: {}", function_name);

    // Create transaction
    debug!("Getting database client");
    let mut client = match pool.get().await {
        Ok(client) => {
            debug!("Successfully got database client");
            client
        }
        Err(e) => {
            error!("Failed to get database client: {}", e);
            return Err(format!("Failed to get client: {}", e));
        }
    };

    debug!("Creating transaction");
    let transaction = match client.transaction().await {
        Ok(transaction) => {
            debug!("Successfully created transaction");
            transaction
        }
        Err(e) => {
            error!("Failed to create transaction: {}", e);
            return Err(format!("Failed to create transaction: {}", e));
        }
    };

    // Set row-level security if sub is provided
    if let Some(sub) = &sub {
        debug!("Setting row-level security for user: {}", sub);
        if let Err(e) = db::authz::set_row_level_security_user_id(&transaction, sub.clone()).await {
            error!("Failed to set row level security: {}", e);
            return Err(format!("Failed to set row level security: {}", e));
        }
    }

    // Execute the appropriate function
    debug!("Executing function: {}", function_name);
    let result = match function_name {
        "list_attachments" => {
            // Use the provided conversation_id
            if let Some(conv_id) = conversation_id {
                debug!("Listing attachments for conversation: {}", conv_id);
                list_attachments(&transaction, conv_id).await
            } else {
                warn!("Missing conversation_id for list_attachments");
                Err("Missing conversation_id".to_string())
            }
        }
        "read_attachment" => {
            let file_id = match args["file_id"].as_str() {
                Some(id) => {
                    debug!("Reading attachment with file_id: {}", id);
                    id
                }
                None => {
                    warn!("Missing file_id parameter");
                    return Err("Missing file_id parameter".to_string());
                }
            };

            let file_id = match file_id.parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    warn!("Invalid file_id: {}", e);
                    return Err(format!("Invalid file_id: {}", e));
                }
            };

            let offset = args["offset"].as_u64().unwrap_or(0) as usize;
            let max_bytes = args["max_bytes"].as_u64();
            debug!(
                "Reading attachment with offset: {}, max_bytes: {:?}",
                offset, max_bytes
            );

            read_attachment(&transaction, file_id, offset, max_bytes).await
        }
        _ => {
            warn!("Unknown function: {}", function_name);
            Err(format!("Unknown function: {}", function_name))
        }
    };

    // Commit transaction
    debug!("Committing transaction");
    match transaction.commit().await {
        Ok(_) => debug!("Successfully committed transaction"),
        Err(e) => {
            error!("Failed to commit transaction: {}", e);
            return Err(format!("Failed to commit transaction: {}", e));
        }
    }

    match &result {
        Ok(_r) => info!("Attachment tool execution completed successfully"),
        Err(e) => warn!("Attachment tool execution failed: {}", e),
    }

    result
}

/// List all attachments
#[instrument(skip(transaction))]
async fn list_attachments(
    transaction: &Transaction<'_>,
    conversation_id: i64,
) -> Result<String, String> {
    info!("Listing attachments for conversation: {}", conversation_id);

    // Get all attachments
    debug!("Querying database for attachments");
    let attachments = match db::queries::attachments::get_by_conversation()
        .bind(transaction, &conversation_id)
        .all()
        .await
    {
        Ok(attachments) => {
            debug!("Found {} attachments", attachments.len());
            attachments
        }
        Err(e) => {
            error!("Failed to get attachments: {}", e);
            return Err(format!("Failed to get attachments: {}", e));
        }
    };

    // Convert to JSON
    debug!("Converting attachments to JSON");
    let result = json!({
        "attachments": attachments.iter().map(|a| {
            debug!("Processing attachment: id={}, name={}, size={}", a.id, a.file_name, a.file_size);
            json!({
                "id": a.id,
                "name": a.file_name,
                "size": a.file_size,
                "mime_type": a.mime_type
            })
        }).collect::<Vec<_>>()
    });

    info!("Successfully listed {} attachments", attachments.len());
    Ok(result.to_string())
}

/// Read attachment content
#[instrument(skip(transaction))]
async fn read_attachment(
    transaction: &Transaction<'_>,
    id: i32,
    offset: usize,
    max_bytes: Option<u64>,
) -> Result<String, String> {
    info!(
        "Reading attachment id: {}, offset: {}, max_bytes: {:?}",
        id, offset, max_bytes
    );

    // Get attachment content
    debug!("Querying database for attachment content");
    let content = match db::queries::attachments::get_content()
        .bind(transaction, &id)
        .one()
        .await
    {
        Ok(content) => {
            debug!("Successfully retrieved attachment content");
            content
        }
        Err(e) => {
            error!("Failed to get attachment content: {}", e);
            return Err(format!("Failed to get attachment content: {}", e));
        }
    };

    // Get the bytes
    let bytes = content.object_data;
    debug!("Attachment total size: {} bytes", bytes.len());

    // Apply offset and max_bytes
    let start = std::cmp::min(offset, bytes.len());
    let end = if let Some(max) = max_bytes {
        std::cmp::min(start + max as usize, bytes.len())
    } else {
        bytes.len()
    };
    debug!(
        "Reading bytes from {} to {} (length: {})",
        start,
        end,
        end - start
    );

    let slice = &bytes[start..end];

    // Try to convert to UTF-8 text
    let text = match String::from_utf8(slice.to_vec()) {
        Ok(text) => {
            debug!("Successfully converted bytes to UTF-8 text");
            text
        }
        Err(_) => {
            debug!("Could not convert bytes to UTF-8, treating as binary data");
            format!("Binary data: {} bytes", slice.len())
        }
    };

    // Return as JSON
    let is_binary = text.starts_with("Binary data:");
    debug!("Attachment is binary: {}", is_binary);

    let result = json!({
        "content": text,
        "mime_type": content.mime_type,
        "is_binary": is_binary,
        "total_size": bytes.len(),
        "offset": start,
        "length": end - start
    });

    info!("Successfully read attachment: {} bytes", end - start);
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
