use crate::tool_interface::ToolInterface;
use crate::types::ToolDefinition;
use async_trait::async_trait;
use db::{queries, Pool, Transaction};
use rig::client::EmbeddingsClient;
use rig::embeddings::EmbeddingModel;
use rig::providers::{ollama, openai};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct SearchContextParams {
    query: String,
    #[serde(default)]
    limit: Option<i32>,
}

pub struct SearchContextTool {
    pool: Pool,
    sub: String,
    conversation_id: i64,
    prompt_id: i32,
}

impl SearchContextTool {
    pub fn new(pool: Pool, sub: String, conversation_id: i64, prompt_id: i32) -> Self {
        Self {
            pool,
            sub,
            conversation_id,
            prompt_id,
        }
    }
}

pub fn get_tool_definition() -> ToolDefinition {
    ToolDefinition {
        name: "search_context".to_string(),
        description: "Search the knowledge base for text related to the given query and return relevant document chunks.".to_string(),
        parameters: json!({
                "type": "object",
                "properties": {
                    "query": {"type": "string", "description": "The search query"},
                    "limit": {"type": "integer", "description": "Maximum number of chunks to return"}
                },
                "required": ["query"]
            }),
    }
}

fn trim_to_context_length(input: &str, context_length: i32) -> String {
    if input.is_empty() {
        return String::new();
    }
    let effective_context_length = if context_length <= 0 {
        256
    } else {
        context_length
    };
    let char_count = input.chars().count() as i32;
    if char_count <= effective_context_length {
        return input.to_string();
    }
    input
        .chars()
        .take(effective_context_length as usize)
        .collect()
}

async fn get_embeddings_via_rig(
    input: &str,
    api_end_point: &str,
    model: &str,
    context_length: i32,
    api_key: Option<&str>,
) -> Result<Vec<f32>, String> {
    let text = String::from_utf8_lossy(input.as_bytes()).to_string();
    let trimmed_text = trim_to_context_length(&text, context_length);

    let normalized_base_url = api_end_point
        .strip_suffix("/embeddings")
        .or_else(|| api_end_point.strip_suffix("/v1/embeddings"))
        .map(|s| s.trim_end_matches('/').to_string())
        .unwrap_or_else(|| api_end_point.trim_end_matches('/').to_string());

    let embedding = if let Some(key) = api_key.filter(|k| !k.trim().is_empty()) {
        let client = openai::Client::builder(key)
            .base_url(&normalized_base_url)
            .build();
        client
            .embedding_model(model)
            .embed_text(&trimmed_text)
            .await
            .map_err(|e| e.to_string())?
    } else {
        let client = ollama::Client::builder()
            .base_url(&normalized_base_url)
            .build();
        client
            .embedding_model(model)
            .embed_text(&trimmed_text)
            .await
            .map_err(|e| e.to_string())?
    };

    Ok(embedding.vec.into_iter().map(|v| v as f32).collect())
}

async fn search_context(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    conversation_id: i64,
    query: &str,
    limit: i32,
) -> Result<serde_json::Value, serde_json::Value> {
    let team_row = transaction
        .query_one(
            "SELECT team_id FROM llm.conversations WHERE id = $1",
            &[&conversation_id],
        )
        .await
        .map_err(|e| json!({"error": "Failed to get conversation", "details": e.to_string()}))?;
    let team_id: i32 = team_row.get(0);

    let prompt = queries::prompts::prompt()
        .bind(transaction, &prompt_id, &team_id)
        .one()
        .await
        .map_err(|e| json!({"error": "Failed to fetch prompt", "details": e.to_string()}))?;

    let (base_url, model, api_key) = match (
        prompt.embeddings_base_url,
        prompt.embeddings_model,
        prompt.embeddings_api_key,
    ) {
        (Some(url), Some(model), api_key) => (url, model, api_key),
        _ => {
            return Err(json!({"error": "Prompt missing embeddings configuration"}));
        }
    };

    let embeddings = get_embeddings_via_rig(
        query,
        &base_url,
        &model,
        prompt.embeddings_context_size.unwrap_or(256),
        api_key.as_deref(),
    )
    .await
    .map_err(|e| json!({"error": "Failed to get embeddings", "details": e.to_string()}))?;

    let context = db::get_related_context(transaction, prompt_id, limit, embeddings)
        .await
        .map_err(|e| json!({"error": "Failed to search context", "details": e.to_string()}))?;

    for chunk in &context {
        queries::chats_chunks::create_chunks_chats()
            .bind(transaction, &chunk.chunk_id, &conversation_id)
            .await
            .map_err(
                |e| json!({"error": "Failed to record chunk usage", "details": e.to_string()}),
            )?;
    }

    let chunks_json: Vec<_> = context
        .into_iter()
        .map(|c| json!({"id": c.chunk_id, "text": c.chunk_text}))
        .collect();

    Ok(json!({"chunks": chunks_json}))
}

#[async_trait]
impl ToolInterface for SearchContextTool {
    fn get_tool(&self) -> ToolDefinition {
        get_tool_definition()
    }

    async fn execute(
        &self,
        arguments: &serde_json::Value,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let params: SearchContextParams = serde_json::from_value(arguments.clone())
            .map_err(|e| json!({"error": "Invalid parameters", "details": e.to_string()}))?;

        let limit = params.limit.unwrap_or(5);

        let mut client =
            self.pool.get().await.map_err(
                |e| json!({"error": "Failed to get DB client", "details": e.to_string()}),
            )?;
        let transaction = client.transaction().await.map_err(
            |e| json!({"error": "Failed to start transaction", "details": e.to_string()}),
        )?;

        db::authz::set_row_level_security_user_id(&transaction, self.sub.clone())
            .await
            .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

        let result = search_context(
            &transaction,
            self.prompt_id,
            self.conversation_id,
            &params.query,
            limit,
        )
        .await;

        if result.is_ok() {
            transaction
                .commit()
                .await
                .map_err(|e| json!({"error": "Failed to commit", "details": e.to_string()}))?;
        } else {
            transaction.rollback().await.ok();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_search_context_tool() {
        let tool = get_tool_definition();
        assert_eq!(tool.name, "search_context");
    }
}
