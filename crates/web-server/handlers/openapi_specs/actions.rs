use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::{Html, IntoResponse},
};
use db::{authz, queries, Json, OpenapiSpecCategory, Pool};
use validator::Validate;
use web_pages::openapi_specs::upsert::OpenapiSpecForm;
use web_pages::routes::openapi_specs::{Delete, Upsert};

use super::super::integrations::helpers::parse_openapi_spec_json_value;

fn spec_string_at<'a>(spec: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    spec.get("info")?
        .get(key)?
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn spec_nested_string_at<'a>(
    spec: &'a serde_json::Value,
    first_key: &str,
    second_key: &str,
) -> Option<&'a str> {
    spec.get("info")?
        .get(first_key)?
        .get(second_key)?
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn slugify(value: &str) -> Option<String> {
    let mut slug = String::new();
    let mut previous_was_separator = false;

    for character in value.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            previous_was_separator = false;
        } else if !previous_was_separator && !slug.is_empty() {
            slug.push('-');
            previous_was_separator = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        None
    } else {
        Some(slug)
    }
}

fn derive_slug_from_spec(spec: &serde_json::Value) -> Option<String> {
    spec_string_at(spec, "x-bionic-slug")
        .or_else(|| spec_string_at(spec, "bionic-slug"))
        .or_else(|| spec_string_at(spec, "title"))
        .and_then(slugify)
}

fn derive_title_from_spec(spec: &serde_json::Value) -> Option<String> {
    spec_string_at(spec, "title").map(str::to_string)
}

fn derive_description_from_spec(spec: &serde_json::Value) -> Option<String> {
    spec_string_at(spec, "description").map(str::to_string)
}

fn derive_logo_url_from_spec(spec: &serde_json::Value) -> Option<String> {
    spec_nested_string_at(spec, "x-logo", "url")
        .or_else(|| spec_nested_string_at(spec, "logo", "url"))
        .map(str::to_string)
}

fn parse_category(category: &str) -> OpenapiSpecCategory {
    match category {
        "WebSearch" => OpenapiSpecCategory::WebSearch,
        _ => OpenapiSpecCategory::Application,
    }
}

fn should_auto_select_system_spec(category: OpenapiSpecCategory, is_active: bool) -> bool {
    category == OpenapiSpecCategory::WebSearch && is_active
}

pub async fn action_upsert(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(mut form): Form<OpenapiSpecForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    // Trim whitespace from inputs
    form.category = form.category.trim().to_string();
    form.spec = form.spec.trim().to_string();

    if let Err(validation) = form.validate() {
        form.error = Some(format!("Validation error: {}", validation));
        let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
        return Ok(Html(html).into_response());
    }

    let parsed_spec = match parse_openapi_spec_json_value(&form.spec) {
        Ok(value) => value,
        Err(error) => {
            form.error = Some(error);
            let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
            return Ok(Html(html).into_response());
        }
    };
    form.slug = match derive_slug_from_spec(&parsed_spec) {
        Some(slug) => slug,
        None => {
            form.error = Some(
                "Could not derive a slug from the OpenAPI spec. Add info.x-bionic-slug or a non-empty info.title."
                    .to_string(),
            );
            let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
            return Ok(Html(html).into_response());
        }
    };
    form.title = match derive_title_from_spec(&parsed_spec) {
        Some(title) => title,
        None => {
            form.error = Some(
                "Could not derive a title from the OpenAPI spec. Add a non-empty info.title."
                    .to_string(),
            );
            let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
            return Ok(Html(html).into_response());
        }
    };
    form.description = derive_description_from_spec(&parsed_spec).unwrap_or_default();
    form.logo_url = derive_logo_url_from_spec(&parsed_spec).unwrap_or_default();

    let description_param = if form.description.is_empty() {
        None
    } else {
        Some(form.description.as_str())
    };

    let logo_url_param = if form.logo_url.is_empty() {
        None
    } else {
        Some(form.logo_url.as_str())
    };

    let category = parse_category(&form.category);
    let spec_json = Json(parsed_spec);

    let result: Result<i32, db::TokioPostgresError> = if let Some(id) = form.id {
        queries::openapi_specs::update()
            .bind(
                &transaction,
                &form.slug,
                &form.title,
                &description_param,
                &spec_json,
                &logo_url_param,
                &category,
                &form.is_active,
                &id,
            )
            .await
            .map(|_| id)
    } else {
        queries::openapi_specs::insert()
            .bind(
                &transaction,
                &form.slug,
                &form.title,
                &description_param,
                &spec_json,
                &logo_url_param,
                &category,
                &form.is_active,
            )
            .one()
            .await
    };

    let spec_id = match result {
        Ok(spec_id) => spec_id,
        Err(error) => {
            if let Some(db_error) = error.as_db_error() {
                if db_error.code().code() == "23505" {
                    form.error =
                        Some("Slug already exists. Please choose another one.".to_string());
                    let html = web_pages::openapi_specs::upsert::page(team_id, rbac, form);
                    return Ok(Html(html).into_response());
                }
            }
            return Err(CustomError::from(error));
        }
    };

    if should_auto_select_system_spec(category, form.is_active) {
        queries::openapi_spec_selections::set_selection()
            .bind(&transaction, &OpenapiSpecCategory::WebSearch, &spec_id)
            .await?;
    }

    transaction.commit().await?;

    let message = if form.id.is_some() {
        "OpenAPI spec updated"
    } else {
        "OpenAPI spec created"
    };

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::openapi_specs::Index { team_id }.to_string(),
        message,
    )
    .into_response())
}
pub async fn action_delete(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    queries::openapi_specs::delete()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::openapi_specs::Index { team_id }.to_string(),
        "OpenAPI spec deleted",
    )
    .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn derive_slug_prefers_x_bionic_slug() {
        let spec = json!({
            "info": {
                "title": "Calendar API",
                "bionic-slug": "calendar",
                "x-bionic-slug": "websearch"
            }
        });

        assert_eq!(derive_slug_from_spec(&spec), Some("websearch".to_string()));
    }

    #[test]
    fn derive_slug_falls_back_to_legacy_bionic_slug() {
        let spec = json!({
            "info": {
                "title": "Calendar API",
                "bionic-slug": "google-calendar"
            }
        });

        assert_eq!(
            derive_slug_from_spec(&spec),
            Some("google-calendar".to_string())
        );
    }

    #[test]
    fn derive_slug_slugifies_title() {
        let spec = json!({
            "info": {
                "title": "Enterprise Email API"
            }
        });

        assert_eq!(
            derive_slug_from_spec(&spec),
            Some("enterprise-email-api".to_string())
        );
    }

    #[test]
    fn derive_slug_returns_none_for_unusable_values() {
        let spec = json!({
            "info": {
                "title": "  !!!  "
            }
        });

        assert_eq!(derive_slug_from_spec(&spec), None);
    }

    #[test]
    fn derive_title_reads_openapi_info_title() {
        let spec = json!({
            "info": {
                "title": "Enterprise Email API"
            }
        });

        assert_eq!(
            derive_title_from_spec(&spec),
            Some("Enterprise Email API".to_string())
        );
    }

    #[test]
    fn derive_title_rejects_blank_title() {
        let spec = json!({
            "info": {
                "title": "  "
            }
        });

        assert_eq!(derive_title_from_spec(&spec), None);
    }

    #[test]
    fn derive_description_reads_openapi_info_description() {
        let spec = json!({
            "info": {
                "description": "Inbox eval API."
            }
        });

        assert_eq!(
            derive_description_from_spec(&spec),
            Some("Inbox eval API.".to_string())
        );
    }

    #[test]
    fn derive_logo_prefers_x_logo() {
        let spec = json!({
            "info": {
                "x-logo": {"url": "https://example.com/x-logo.svg"},
                "logo": {"url": "https://example.com/logo.svg"}
            }
        });

        assert_eq!(
            derive_logo_url_from_spec(&spec),
            Some("https://example.com/x-logo.svg".to_string())
        );
    }

    #[test]
    fn derive_logo_falls_back_to_legacy_logo() {
        let spec = json!({
            "info": {
                "logo": {"url": "https://example.com/logo.svg"}
            }
        });

        assert_eq!(
            derive_logo_url_from_spec(&spec),
            Some("https://example.com/logo.svg".to_string())
        );
    }

    #[test]
    fn auto_selects_active_web_search_specs() {
        assert!(should_auto_select_system_spec(
            OpenapiSpecCategory::WebSearch,
            true
        ));
    }

    #[test]
    fn does_not_auto_select_inactive_or_application_specs() {
        assert!(!should_auto_select_system_spec(
            OpenapiSpecCategory::WebSearch,
            false
        ));
        assert!(!should_auto_select_system_spec(
            OpenapiSpecCategory::Application,
            true
        ));
    }
}
