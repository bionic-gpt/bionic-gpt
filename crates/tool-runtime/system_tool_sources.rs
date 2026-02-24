use crate::openapi_tool_factory::BionicOpenAPI;
use crate::tool_auth::StaticTokenProvider;
use crate::tool_interface::ToolInterface;
use crate::types::ToolDefinition;
use db::{queries, OpenapiSpec, OpenapiSpecCategory, Pool};
use std::sync::Arc;

#[derive(Clone, Debug)]
struct SelectedSpec {
    spec: OpenapiSpec,
    api_key: Option<String>,
}

async fn load_selected_spec(
    transaction: &db::Transaction<'_>,
    category: OpenapiSpecCategory,
) -> Result<Option<SelectedSpec>, db::TokioPostgresError> {
    let selection = queries::openapi_spec_selections::selection()
        .bind(transaction, &category)
        .opt()
        .await?;

    let Some(selection) = selection else {
        return Ok(None);
    };

    let spec = queries::openapi_specs::by_id()
        .bind(transaction, &selection.openapi_spec_id)
        .one()
        .await?;

    if !spec.is_active {
        return Ok(None);
    }

    let api_key = queries::openapi_spec_api_keys::api_key()
        .bind(transaction, &spec.id)
        .opt()
        .await?
        .and_then(|row| row.api_key);

    Ok(Some(SelectedSpec { spec, api_key }))
}

fn build_openapi_helpers(
    selected: SelectedSpec,
) -> Result<(BionicOpenAPI, Option<String>), String> {
    let openapi =
        BionicOpenAPI::new(&selected.spec.spec).map_err(|e| format!("Spec parse failed: {e}"))?;

    let requires_api_key = openapi.has_api_key_security();
    if requires_api_key && selected.api_key.is_none() {
        return Err("API key not configured".to_string());
    }

    Ok((openapi, selected.api_key))
}

async fn load_selected_helpers(
    pool: &Pool,
) -> Result<Vec<(BionicOpenAPI, Option<String>)>, String> {
    let mut client = pool.get().await.map_err(|e| e.to_string())?;
    let transaction = client.transaction().await.map_err(|e| e.to_string())?;

    let mut helpers = Vec::new();
    for category in [
        OpenapiSpecCategory::WebSearch,
        OpenapiSpecCategory::CodeSandbox,
    ] {
        if let Some(selected) = load_selected_spec(&transaction, category)
            .await
            .map_err(|e| e.to_string())?
        {
            if let Ok(helper) = build_openapi_helpers(selected) {
                helpers.push(helper);
            }
        }
    }

    Ok(helpers)
}

pub async fn get_system_openapi_tool_definitions(
    pool: &Pool,
) -> Result<Vec<ToolDefinition>, String> {
    let mut definitions = Vec::new();
    for (openapi, _) in load_selected_helpers(pool).await? {
        let mut tools = openapi.create_tool_definitions().tool_definitions;
        definitions.append(&mut tools);
    }

    Ok(definitions)
}

pub async fn get_system_openapi_tools(pool: &Pool) -> Result<Vec<Arc<dyn ToolInterface>>, String> {
    let mut tools = Vec::new();
    for (openapi, api_key) in load_selected_helpers(pool).await? {
        let token_provider = api_key.map(|key| Arc::new(StaticTokenProvider::new(key)) as Arc<_>);
        let mut openapi_tools = openapi.create_tools(token_provider)?;
        tools.append(&mut openapi_tools);
    }

    Ok(tools)
}
