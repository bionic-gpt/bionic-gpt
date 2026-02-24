use crate::types::ToolDefinition;
use db::{queries, Pool, Transaction};
use rig::tool::{ToolDyn, ToolError};
use rig::wasm_compat::WasmBoxedFuture;
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

async fn execute_list_datasets(
    tool: &ListDatasetsTool,
) -> Result<serde_json::Value, serde_json::Value> {
    let mut client =
        tool.pool.get().await.map_err(
            |e| json!({"error": "Failed to get database client", "details": e.to_string()}),
        )?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, tool.sub.clone())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let result = list_datasets(&transaction, tool.prompt_id).await;

    if result.is_ok() {
        transaction.commit().await.map_err(
            |e| json!({"error": "Failed to commit transaction", "details": e.to_string()}),
        )?;
    } else {
        transaction.rollback().await.ok();
    }

    result
}

impl ToolDyn for ListDatasetsTool {
    fn name(&self) -> String {
        get_tool_definition().name
    }

    fn definition(&self, _prompt: String) -> WasmBoxedFuture<'_, ToolDefinition> {
        Box::pin(async move { get_tool_definition() })
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let _arguments: Value = serde_json::from_str(&args).map_err(ToolError::JsonError)?;
            let result = execute_list_datasets(self).await.map_err(|err| {
                ToolError::ToolCallError(Box::new(std::io::Error::other(err.to_string())))
            })?;
            serde_json::to_string(&result).map_err(ToolError::JsonError)
        })
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
