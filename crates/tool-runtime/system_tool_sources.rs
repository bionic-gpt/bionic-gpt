use crate::openapi_tool_factory::BionicOpenAPI;
use crate::tool_auth::StaticTokenProvider;
use crate::types::ToolDefinition;
use db::{queries, OpenapiSpec, OpenapiSpecCategory, Pool};
use rig::tool::ToolDyn;
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

    if !is_usable_selected_spec(&spec, category) {
        return Ok(None);
    }

    load_spec_with_api_key(transaction, spec).await
}

async fn load_single_active_spec(
    transaction: &db::Transaction<'_>,
    category: OpenapiSpecCategory,
) -> Result<Option<SelectedSpec>, db::TokioPostgresError> {
    let specs = queries::openapi_specs::by_category()
        .bind(transaction, &category)
        .all()
        .await?;

    let Some(spec) = single_active_spec(specs) else {
        return Ok(None);
    };

    load_spec_with_api_key(transaction, spec).await
}

async fn load_spec_with_api_key(
    transaction: &db::Transaction<'_>,
    spec: OpenapiSpec,
) -> Result<Option<SelectedSpec>, db::TokioPostgresError> {
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

fn is_usable_selected_spec(spec: &OpenapiSpec, category: OpenapiSpecCategory) -> bool {
    spec.category == category && spec.is_active
}

fn single_active_spec(specs: Vec<OpenapiSpec>) -> Option<OpenapiSpec> {
    let mut active_specs = specs.into_iter().filter(|spec| spec.is_active);
    let spec = active_specs.next()?;

    if active_specs.next().is_some() {
        return None;
    }

    Some(spec)
}

async fn load_selected_helpers(
    pool: &Pool,
) -> Result<Vec<(BionicOpenAPI, Option<String>)>, String> {
    let mut client = pool.get().await.map_err(|e| e.to_string())?;
    let transaction = client.transaction().await.map_err(|e| e.to_string())?;

    let mut helpers = Vec::new();
    for category in [OpenapiSpecCategory::WebSearch] {
        let selected = match load_selected_spec(&transaction, category)
            .await
            .map_err(|e| e.to_string())?
        {
            Some(selected) => Some(selected),
            None => load_single_active_spec(&transaction, category)
                .await
                .map_err(|e| e.to_string())?,
        };

        if let Some(selected) = selected {
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

pub async fn get_system_openapi_tools(pool: &Pool) -> Result<Vec<Arc<dyn ToolDyn>>, String> {
    let mut tools = Vec::new();
    for (openapi, api_key) in load_selected_helpers(pool).await? {
        let token_provider = api_key.map(|key| Arc::new(StaticTokenProvider::new(key)) as Arc<_>);
        let mut openapi_tools = openapi.create_tools(token_provider)?;
        tools.append(&mut openapi_tools);
    }

    Ok(tools)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn spec(id: i32, category: OpenapiSpecCategory, is_active: bool) -> OpenapiSpec {
        OpenapiSpec {
            id,
            slug: format!("spec-{id}"),
            title: format!("Spec {id}"),
            description: None,
            spec: json!({
                "openapi": "3.0.0",
                "info": {"title": format!("Spec {id}"), "version": "1.0.0"},
                "paths": {}
            }),
            logo_url: None,
            category,
            is_active,
            created_at: "2026-08-20T00:00:00Z".to_string(),
            updated_at: "2026-08-20T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn selected_spec_must_match_category_and_be_active() {
        assert!(is_usable_selected_spec(
            &spec(1, OpenapiSpecCategory::WebSearch, true),
            OpenapiSpecCategory::WebSearch
        ));
        assert!(!is_usable_selected_spec(
            &spec(1, OpenapiSpecCategory::Application, true),
            OpenapiSpecCategory::WebSearch
        ));
        assert!(!is_usable_selected_spec(
            &spec(1, OpenapiSpecCategory::WebSearch, false),
            OpenapiSpecCategory::WebSearch
        ));
    }

    #[test]
    fn single_active_spec_returns_only_active_spec() {
        let selected = single_active_spec(vec![
            spec(1, OpenapiSpecCategory::WebSearch, false),
            spec(2, OpenapiSpecCategory::WebSearch, true),
        ]);

        assert_eq!(selected.map(|spec| spec.id), Some(2));
    }

    #[test]
    fn single_active_spec_returns_none_for_no_active_specs() {
        let selected = single_active_spec(vec![
            spec(1, OpenapiSpecCategory::WebSearch, false),
            spec(2, OpenapiSpecCategory::WebSearch, false),
        ]);

        assert!(selected.is_none());
    }

    #[test]
    fn single_active_spec_returns_none_for_multiple_active_specs() {
        let selected = single_active_spec(vec![
            spec(1, OpenapiSpecCategory::WebSearch, true),
            spec(2, OpenapiSpecCategory::WebSearch, true),
        ]);

        assert!(selected.is_none());
    }
}
