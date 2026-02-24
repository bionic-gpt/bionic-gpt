use crate::tool_interface::ToolInterface;
use crate::types::ToolDefinition;
use async_trait::async_trait;
use db::{queries, Pool, Transaction};
use serde_json::{json, Value};

pub struct ListDatasetsTool {
    pool: Pool,
    sub: String,
    prompt_id: i32,
}

impl ListDatasetsTool {
    pub fn new(pool: Pool, sub: String, prompt_id: i32) -> Self {
        Self {
            pool,
            sub,
            prompt_id,
        }
    }
}

pub fn get_tool_definition() -> ToolDefinition {
    ToolDefinition {
        name: "list_datasets".to_string(),
        description: "List all datasets connected to this assistant.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {},
            "required": []
        }),
    }
}

async fn list_datasets(
    transaction: &Transaction<'_>,
    prompt_id: i32,
) -> Result<serde_json::Value, serde_json::Value> {
    let datasets = queries::prompts::prompt_datasets()
        .bind(transaction, &prompt_id)
        .all()
        .await
        .map_err(|e| json!({"error": "Failed to get datasets", "details": e.to_string()}))?;

    Ok(json!({
        "datasets": datasets
            .iter()
            .map(|d| json!({"dataset_id": d.dataset_id, "name": d.name}))
            .collect::<Vec<_>>()
    }))
}

#[async_trait]
impl ToolInterface for ListDatasetsTool {
    fn get_tool(&self) -> ToolDefinition {
        get_tool_definition()
    }

    async fn execute(&self, _arguments: &Value) -> Result<serde_json::Value, serde_json::Value> {
        let mut client = self.pool.get().await.map_err(
            |e| json!({"error": "Failed to get database client", "details": e.to_string()}),
        )?;
        let transaction = client.transaction().await.map_err(
            |e| json!({"error": "Failed to start transaction", "details": e.to_string()}),
        )?;

        db::authz::set_row_level_security_user_id(&transaction, self.sub.clone())
            .await
            .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

        let result = list_datasets(&transaction, self.prompt_id).await;

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
    fn test_get_list_datasets_tool() {
        let tool = get_tool_definition();
        assert_eq!(tool.name, "list_datasets");
    }
}
