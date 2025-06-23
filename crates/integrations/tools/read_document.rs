use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::Pool;
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use rag_engine::unstructured::{document_to_chunks, Unstructured};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ReadDocumentParams {
    file_id: i32,
    #[serde(default)]
    section_index: usize,
}

pub struct ReadDocumentTool {
    pool: Pool,
    sub: String,
    conversation_id: i64,
}

impl ReadDocumentTool {
    pub fn new(pool: Pool, sub: String, conversation_id: i64) -> Self {
        Self {
            pool,
            sub,
            conversation_id,
        }
    }
}

pub fn get_tool_definition() -> BionicToolDefinition {
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "read_document".to_string(),
            description:
                "Read sections from a document attachment respecting the model context size."
                    .to_string(),

            parameters: json!({
                "type": "object",
                "properties": {
                    "file_id": {"type": "integer", "description": "ID of the attachment"},
                    "section_index": {"type": "integer", "minimum": 0, "description": "Index of the first section (default 0)"}
                },
                "required": ["file_id"]
            }),
        },
    }
}

fn accumulate_sections(
    sections: &[Unstructured],
    start_index: usize,
    max_tokens: i32,
) -> (String, usize, bool) {
    let mut tokens_so_far = 0;
    let mut text_parts: Vec<String> = Vec::new();
    let mut end_index = start_index;

    while end_index < sections.len() {
        let section = &sections[end_index];
        let section_tokens = openai_api::token_count::token_count_from_string(&section.text);
        dbg!(&section.text, tokens_so_far, section_tokens, max_tokens);
        if tokens_so_far + section_tokens > max_tokens {
            break;
        }
        tokens_so_far += section_tokens;
        text_parts.push(section.text.clone());
        end_index += 1;
    }

    tracing::debug!("Accumulated {} tokens", tokens_so_far);

    (
        text_parts.join("\n\n"),
        end_index - start_index,
        end_index < sections.len(),
    )
}

#[async_trait]
impl ToolInterface for ReadDocumentTool {
    fn get_tool(&self) -> BionicToolDefinition {
        get_tool_definition()
    }

    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        let params: ReadDocumentParams = serde_json::from_str(arguments)
            .map_err(|e| json!({"error": "Invalid parameters", "details": e.to_string()}))?;

        let mut client = self.pool.get().await.map_err(
            |e| json!({"error": "Failed to get DB connection", "details": e.to_string()}),
        )?;
        let transaction = client.transaction().await.map_err(
            |e| json!({"error": "Failed to start transaction", "details": e.to_string()}),
        )?;

        db::authz::set_row_level_security_user_id(&transaction, self.sub.clone())
            .await
            .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

        let context_size = db::queries::conversations::conversation_context_size()
            .bind(&transaction, &self.conversation_id)
            .one()
            .await
            .map_err(|e| {
                json!({
                    "error": "Failed to fetch conversation",
                    "details": e.to_string()
                })
            })?
            .context_size;

        let max_tokens = context_size / 2;

        let content = db::queries::attachments::get_content()
            .bind(&transaction, &params.file_id)
            .one()
            .await
            .map_err(
                |e| json!({"error": "Failed to get attachment content", "details": e.to_string()}),
            )?;

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
        .map_err(|e| json!({"error": "Document processing failed", "details": e.to_string()}))?;

        let start = params.section_index.min(sections.len());
        let (text, sections_read, has_more) = accumulate_sections(&sections, start, max_tokens);

        Ok(json!({
            "text": text,
            "sections_read": sections_read,
            "has_more": has_more
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rag_engine::unstructured::MetaData;

    fn dummy(text: &str) -> Unstructured {
        Unstructured {
            type_of: String::new(),
            element_id: String::new(),
            metadata: MetaData {
                filename: String::new(),
                filetype: String::new(),
                page_number: None,
            },
            text: text.to_string(),
        }
    }

    #[test]
    fn test_get_read_document_tool() {
        let tool = get_tool_definition();
        assert_eq!(tool.function.name, "read_document");
    }

    #[test]
    fn test_accumulate_sections_limit() {
        let sections = vec![dummy("one"), dummy("two three"), dummy("four five six")];
        let tokens = openai_api::token_count::token_count_from_string("one");
        let tokens = tokens + openai_api::token_count::token_count_from_string("two three");
        let (text, count, has_more) = accumulate_sections(&sections, 0, tokens);
        assert_eq!(count, 2);
        assert!(has_more);
        assert!(text.contains("one"));
    }
}
