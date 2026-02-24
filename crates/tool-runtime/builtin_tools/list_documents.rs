use crate::types::ToolDefinition;
use db::{Pool, Transaction};
use rig::tool::{ToolDyn, ToolError};
use rig::wasm_compat::WasmBoxedFuture;
use serde_json::{json, Value};
use tracing;

/// A tool that lists all documents available to the user in this chat session or knowledge base.
pub struct ListDocumentsTool {
    pool: Pool,
    sub: String,
    conversation_id: i64,
}

impl ListDocumentsTool {
    pub fn new(pool: Pool, sub: String, conversation_id: i64) -> Self {
        tracing::debug!(
            "Creating new ListDocumentsTool with sub: {:?}, conversation_id: {:?}",
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

/// Returns the tool definition for list_documents
pub fn get_tool_definition() -> ToolDefinition {
    ToolDefinition {
        name: "list_documents".to_string(),
        description:
            "Use this tool to list all documents attached in the current conversation. Always call this before attempting to read or summarize a document. Do not guess file IDs. This returns real 'file_id' values that are required for calling 'read_document'."
                .to_string(),
        parameters: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
    }
}

#[tracing::instrument(skip(tool, arguments), fields(conversation_id = ?tool.conversation_id, sub = ?tool.sub))]
async fn execute_list_documents(
    tool: &ListDocumentsTool,
    arguments: &Value,
) -> Result<serde_json::Value, serde_json::Value> {
    tracing::info!(
        "Executing list_documents tool with arguments: {}",
        arguments
    );

    // Create transaction
    let mut client = match tool.pool.get().await {
        Ok(client) => client,
        Err(e) => {
            return Err(json!({
                "error": "Failed to get database client",
                "details": e.to_string()
            }));
        }
    };

    let transaction = match client.transaction().await {
        Ok(transaction) => transaction,
        Err(e) => {
            return Err(json!({
                "error": "Failed to create transaction",
                "details": e.to_string()
            }));
        }
    };

    // Set row-level security
    if let Err(e) = db::authz::set_row_level_security_user_id(&transaction, tool.sub.clone()).await
    {
        return Err(json!({
            "error": "Failed to set row level security",
            "details": e.to_string()
        }));
    }

    // Use the conversation ID to get documents
    let result = list_documents(&transaction, tool.conversation_id).await;

    // Commit transaction
    if let Err(e) = transaction.commit().await {
        return Err(json!({
            "error": "Failed to commit transaction",
            "details": e.to_string()
        }));
    }

    result
}

impl ToolDyn for ListDocumentsTool {
    fn name(&self) -> String {
        get_tool_definition().name
    }

    fn definition(&self, _prompt: String) -> WasmBoxedFuture<'_, ToolDefinition> {
        Box::pin(async move {
            tracing::debug!("Getting tool definition for ListDocumentsTool");
            get_tool_definition()
        })
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let arguments: Value = serde_json::from_str(&args).map_err(ToolError::JsonError)?;
            let result = execute_list_documents(self, &arguments)
                .await
                .map_err(|err| {
                    ToolError::ToolCallError(Box::new(std::io::Error::other(err.to_string())))
                })?;
            serde_json::to_string(&result).map_err(ToolError::JsonError)
        })
    }
}

/// Lists documents for a given conversation
#[tracing::instrument(skip(transaction))]
async fn list_documents(
    transaction: &Transaction<'_>,
    conversation_id: i64,
) -> Result<serde_json::Value, serde_json::Value> {
    let attachments = match db::queries::attachments::get_by_conversation()
        .bind(transaction, &conversation_id)
        .all()
        .await
    {
        Ok(attachments) => attachments,
        Err(e) => {
            return Err(json!({
                "error": "Failed to get documents",
                "details": e.to_string()
            }));
        }
    };

    let result = json!({
        "documents": attachments.iter().map(|a| {
            json!({
                "file_id": a.id,
                "name": a.file_name,
                "size": a.file_size,
                "mime_type": a.mime_type
            })
        }).collect::<Vec<_>>()
    });

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_list_documents_tool() {
        let tool = get_tool_definition();
        assert_eq!(tool.name, "list_documents");
    }
}
