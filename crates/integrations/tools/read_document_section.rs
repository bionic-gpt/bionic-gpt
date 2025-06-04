use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::Pool;
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use rag_engine::unstructured::document_to_chunks;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ReadDocumentSectionParams {
    file_id: i32,
    #[serde(default)]
    section_index: usize,
    #[serde(default)]
    section_count: usize,
}

pub struct ReadDocumentSectionTool {
    pool: Pool,
    sub: Option<String>,
    _conversation_id: Option<i64>,
}

impl ReadDocumentSectionTool {
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
            name: "read_document_section".to_string(),
            description: Some(
                "Reads one or more consecutive text sections from a document attachment and returns them with pagination metadata. Sections represent coherent parts of the original document, automatically extracted and converted to plain text.".to_string(),
            ),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "file_id": {
                        "type": "integer",
                        "description": "ID of the attachment"
                    },
                    "section_index": {
                        "type": "integer",
                        "minimum": 0,
                        "description": "Index of the first section to retrieve (starts at 0)"
                    },
                    "section_count": {
                        "type": "integer",
                        "minimum": 1,
                        "description": "Number of consecutive sections to retrieve (default 1)"
                    }
                },
                "required": ["file_id"]
            })),
        },
    }
}

#[async_trait]
impl ToolInterface for ReadDocumentSectionTool {
    fn get_tool(&self) -> BionicToolDefinition {
        get_tool_definition()
    }

    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        let params: ReadDocumentSectionParams = serde_json::from_str(arguments)
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
        let sections = document_to_chunks(
            bytes,
            &content.file_name,
            500,
            1500,
            true,
            &config.unstructured_endpoint,
        )
        .await
        .map_err(|e| json!({ "error": "Document processing failed", "details": e.to_string() }))?;

        let total_sections = sections.len();
        let start_index = params.section_index.min(total_sections);
        let count = params.section_count.max(1);
        let end_index = (start_index + count).min(total_sections);
        let selected_sections = &sections[start_index..end_index];

        let combined_text = selected_sections
            .iter()
            .map(|chunk| chunk.text.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(json!({
            "content": combined_text,
            "section_index": start_index,
            "section_count": selected_sections.len(),
            "total_sections": total_sections,
            "has_next": end_index < total_sections,
            "has_prev": start_index > 0,
            "mime_type": content.mime_type
        }))
    }
}
