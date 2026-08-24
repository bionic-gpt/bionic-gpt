use db::{queries, OpenapiSpec, OpenapiSpecCategory, Pool};
use serde_json::Value;
use std::collections::HashMap;

const SERVER_OVERRIDES_ENV: &str = "BIONIC_OPENAPI_SERVER_OVERRIDES";

pub fn openapi_server_overrides() -> HashMap<String, String> {
    let Ok(raw) = std::env::var(SERVER_OVERRIDES_ENV) else {
        return HashMap::new();
    };

    match parse_server_overrides(&raw) {
        Ok(overrides) => overrides,
        Err(error) => {
            tracing::warn!(%error, "ignoring invalid OpenAPI server overrides");
            HashMap::new()
        }
    }
}

fn parse_server_overrides(raw: &str) -> Result<HashMap<String, String>, String> {
    let values: HashMap<String, Value> = serde_json::from_str(raw)
        .map_err(|error| format!("{SERVER_OVERRIDES_ENV} must be a JSON object: {error}"))?;
    let mut overrides = HashMap::new();

    for (slug, value) in values {
        let url = value
            .as_str()
            .map(str::trim)
            .filter(|url| !url.is_empty())
            .ok_or_else(|| format!("server override for {slug} must be a non-empty string"))?;
        overrides.insert(slug, url.to_string());
    }

    Ok(overrides)
}

#[derive(Clone, Debug)]
struct SelectedSpec {
    spec: OpenapiSpec,
    api_key: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SystemOpenapiSpec {
    pub spec: OpenapiSpec,
    pub api_key: Option<String>,
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

pub async fn load_system_openapi_specs(pool: &Pool) -> Result<Vec<SystemOpenapiSpec>, String> {
    let mut client = pool.get().await.map_err(|e| e.to_string())?;
    let transaction = client.transaction().await.map_err(|e| e.to_string())?;
    let mut specs = Vec::new();

    for spec in queries::openapi_specs::active_system()
        .bind(&transaction)
        .all()
        .await
        .map_err(|e| e.to_string())?
    {
        if let Some(selected) = load_spec_with_api_key(&transaction, spec)
            .await
            .map_err(|e| e.to_string())?
        {
            specs.push(SystemOpenapiSpec {
                spec: selected.spec,
                api_key: selected.api_key,
            });
        }
    }

    let web_search = match load_selected_spec(&transaction, OpenapiSpecCategory::WebSearch)
        .await
        .map_err(|e| e.to_string())?
    {
        Some(selected) => Some(selected),
        None => load_single_active_spec(&transaction, OpenapiSpecCategory::WebSearch)
            .await
            .map_err(|e| e.to_string())?,
    };
    if let Some(selected) = web_search {
        if !specs
            .iter()
            .any(|existing| existing.spec.id == selected.spec.id)
        {
            specs.push(SystemOpenapiSpec {
                spec: selected.spec,
                api_key: selected.api_key,
            });
        }
    }

    Ok(specs)
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
            is_system: false,
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
    fn parses_server_overrides() {
        let overrides = parse_server_overrides(r#"{"typst":"http://localhost:3200"}"#).unwrap();
        assert_eq!(
            overrides.get("typst"),
            Some(&"http://localhost:3200".to_string())
        );
    }

    #[test]
    fn rejects_invalid_server_overrides() {
        assert!(parse_server_overrides("[]").is_err());
        assert!(parse_server_overrides(r#"{"typst":123}"#).is_err());
        assert!(parse_server_overrides(r#"{"typst":"  "}"#).is_err());
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
