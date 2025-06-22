use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::{queries, Pool, Transaction};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use serde::Deserialize;
use serde_json::json;
use tracing;

#[derive(Debug, Deserialize)]
struct ListDatasetFilesParams {
    dataset_id: i32,
}

pub struct ListDatasetFilesTool {
    pool: Pool,
    sub: String,
}

impl ListDatasetFilesTool {
    pub fn new(pool: Pool, sub: String) -> Self {
        Self { pool, sub }
    }
}

pub fn get_tool_definition() -> BionicToolDefinition {
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "list_dataset_files".to_string(),
            description: Some("List all files within a specific dataset.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "dataset_id": {"type": "integer", "description": "ID of the dataset"}
                },
                "required": ["dataset_id"]
            })),
        },
    }
}

async fn list_files(
    transaction: &Transaction<'_>,
    dataset_id: i32,
) -> Result<serde_json::Value, serde_json::Value> {
    let docs = queries::documents::documents()
        .bind(transaction, &dataset_id)
        .all()
        .await
        .map_err(|e| json!({"error": "Failed to get documents", "details": e.to_string()}))?;

    Ok(json!({
        "files": docs
            .iter()
            .map(|d| json!({
                "document_id": d.id,
                "name": d.file_name,
                "size": d.content_size,
                "batches": d.batches
            }))
            .collect::<Vec<_>>()
    }))
}

#[async_trait]
impl ToolInterface for ListDatasetFilesTool {
    fn get_tool(&self) -> BionicToolDefinition {
        get_tool_definition()
    }

    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        let params: ListDatasetFilesParams = serde_json::from_str(arguments)
            .map_err(|e| json!({"error": "Invalid parameters", "details": e.to_string()}))?;

        let mut client = self.pool.get().await.map_err(
            |e| json!({"error": "Failed to get database client", "details": e.to_string()}),
        )?;
        let transaction = client.transaction().await.map_err(
            |e| json!({"error": "Failed to start transaction", "details": e.to_string()}),
        )?;

        db::authz::set_row_level_security_user_id(&transaction, self.sub.clone())
            .await
            .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

        let result = list_files(&transaction, params.dataset_id).await;

        if result.is_ok() {
            transaction.commit().await.map_err(
                |e| json!({"error": "Failed to commit transaction", "details": e.to_string()}),
            )?;
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
    fn test_get_list_dataset_files_tool() {
        let tool = get_tool_definition();
        assert_eq!(tool.function.name, "list_dataset_files");
    }
}
