use crate::token_count;
use crate::types::ToolDefinition;
use db::Pool;
use rag_engine::unstructured::{document_to_chunks, Unstructured};
use rig::tool::{ToolDyn, ToolError};
use rig::wasm_compat::WasmBoxedFuture;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
struct ReadDocumentParams {
    file_id: Option<i32>,
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

pub fn get_tool_definition() -> ToolDefinition {
    ToolDefinition {
        name: "read_document".to_string(),
        description:
            "Reads the content of a document attachment. You must provide a valid 'file_id' from 'list_documents'. Never guess or hard-code the ID. The tool returns one or more sections from the document starting at the 'section_index' (default is 0). Always pass the file_id as an integer. Include an 'id' field in the tool call JSON structure."
                .to_string(),
        parameters: json!({
                "type": "object",
                "properties": {
                    "file_id": {"type": "integer", "description": "The ID of the document to read. Must be obtained from 'list_documents'."},
                    "section_index": {"type": "integer", "minimum": 0, "description": "Section index to start reading from. Default is 0."}
                },
                "required": []
            }),
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
        let section_tokens = token_count::token_count_from_string(&section.text);
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

async fn execute_read_document(
    tool: &ReadDocumentTool,
    arguments: &Value,
) -> Result<serde_json::Value, serde_json::Value> {
    let params: ReadDocumentParams = serde_json::from_value(arguments.clone())
        .map_err(|e| json!({"error": "Invalid parameters", "details": e.to_string()}))?;

    let mut client =
        tool.pool.get().await.map_err(
            |e| json!({"error": "Failed to get DB connection", "details": e.to_string()}),
        )?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, tool.sub.clone())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let context_size = db::queries::conversations::conversation_context_size()
        .bind(&transaction, &tool.conversation_id)
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

    let content = if let Some(file_id) = params.file_id {
        db::queries::attachments::get_content()
            .bind(&transaction, &file_id)
            .one()
            .await
            .map_err(
                |e| json!({"error": "Failed to get attachment content", "details": e.to_string()}),
            )?
    } else {
        match db::queries::attachments::get_latest_content()
            .bind(&transaction, &tool.conversation_id)
            .opt()
            .await
            .map_err(
                |e| json!({"error": "Failed to get attachment content", "details": e.to_string()}),
            )? {
            Some(content) => content,
            None => return Err(json!({"error": "No attachments found"})),
        }
    };

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

impl ToolDyn for ReadDocumentTool {
    fn name(&self) -> String {
        get_tool_definition().name
    }

    fn definition(&self, _prompt: String) -> WasmBoxedFuture<'_, ToolDefinition> {
        Box::pin(async move { get_tool_definition() })
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let arguments: Value = serde_json::from_str(&args).map_err(ToolError::JsonError)?;
            let result = execute_read_document(self, &arguments)
                .await
                .map_err(|err| {
                    ToolError::ToolCallError(Box::new(std::io::Error::other(err.to_string())))
                })?;
            serde_json::to_string(&result).map_err(ToolError::JsonError)
        })
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
        assert_eq!(tool.name, "read_document");
    }

    #[test]
    fn test_accumulate_sections_limit() {
        let sections = vec![dummy("one"), dummy("two three"), dummy("four five six")];
        let tokens = token_count::token_count_from_string("one");
        let tokens = tokens + token_count::token_count_from_string("two three");
        let (text, count, has_more) = accumulate_sections(&sections, 0, tokens);
        assert_eq!(count, 2);
        assert!(has_more);
        assert!(text.contains("one"));
    }

    #[test]
    fn test_params_optional_file_id_none() {
        let json = r#"{"section_index": 2}"#;
        let params: ReadDocumentParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.file_id, None);
        assert_eq!(params.section_index, 2);
    }

    #[test]
    fn test_params_optional_file_id_some() {
        let json = r#"{"file_id": 7}"#;
        let params: ReadDocumentParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.file_id, Some(7));
        assert_eq!(params.section_index, 0);
    }
}
