use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::Pool;
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use rag_engine::unstructured::document_to_chunks;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct AttachmentChunkParams {
    file_id: i32,
    #[serde(default)]
    chunk_index: usize,
    #[serde(default)]
    chunk_count: usize,
}

pub struct AttachmentChunkTool {
    pool: Pool,
    sub: Option<String>,
    _conversation_id: Option<i64>,
}

impl AttachmentChunkTool {
    pub fn new(pool: Pool, sub: Option<String>, conversation_id: Option<i64>) -> Self {
        Self {
            pool,
            sub,
            _conversation_id: conversation_id,
        }
    }
}

pub fn get_tool_definition() -> BionicToolDefinition {
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "get_attachment_text_chunks".to_string(),
            description: Some("Reads one or more consecutive text chunks from a document attachment and returns them with pagination metadata. Chunks are intelligently extracted and represent coherent sections of the original text.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "file_id": {
                        "type": "integer",
                        "description": "ID of the attachment"
                    },
                    "chunk_index": {
                        "type": "integer",
                        "minimum": 0,
                        "description": "Index of the first chunk to retrieve (starts at 0)"
                    },
                    "chunk_count": {
                        "type": "integer",
                        "minimum": 1,
                        "description": "Number of consecutive chunks to retrieve (default 1)"
                    }
                },
                "required": ["file_id"]
            })),
        },
    }
}

#[async_trait]
impl ToolInterface for AttachmentChunkTool {
    fn get_tool(&self) -> BionicToolDefinition {
        get_tool_definition()
    }

    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        let params: AttachmentChunkParams = serde_json::from_str(arguments)
            .map_err(|e| json!({ "error": "Invalid parameters", "details": e.to_string() }))?;

        let mut client = self.pool.get().await.map_err(
            |e| json!({ "error": "Failed to get DB connection", "details": e.to_string() }),
        )?;

        let transaction = client.transaction().await.map_err(
            |e| json!({ "error": "Failed to start transaction", "details": e.to_string() }),
        )?;

        if let Some(sub) = &self.sub {
            db::authz::set_row_level_security_user_id(&transaction, sub.clone())
                .await
                .map_err(|e| json!({ "error": "Failed to set RLS", "details": e.to_string() }))?;
        }

        let content = db::queries::attachments::get_content()
            .bind(&transaction, &params.file_id)
            .one()
            .await
            .map_err(|e| json!({ "error": "Failed to get attachment content", "details": e.to_string() }))?;

        let bytes = content.object_data;

        let config = rag_engine::config::Config::new();
        let chunks = document_to_chunks(
            bytes,
            &content.file_name,
            500,
            1500,
            true,
            &config.unstructured_endpoint,
        )
        .await
        .map_err(|e| json!({ "error": "Chunking failed", "details": e.to_string() }))?;

        let total_chunks = chunks.len();
        let start_index = params.chunk_index.min(total_chunks);
        let count = params.chunk_count.max(1);
        let end_index = (start_index + count).min(total_chunks);
        let selected_chunks = &chunks[start_index..end_index];

        let combined_text = selected_chunks
            .iter()
            .map(|chunk| chunk.text.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(json!({
            "content": combined_text,
            "chunk_index": start_index,
            "chunk_count": selected_chunks.len(),
            "total_chunks": total_chunks,
            "has_next": end_index < total_chunks,
            "has_prev": start_index > 0,
            "mime_type": content.mime_type
        }))
    }
}
