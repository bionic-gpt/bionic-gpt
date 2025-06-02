use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Form;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, queries, Json, Pool};
use integrations::external_integration::create_tool_definitions_from_spec;
use validator::Validate;
use web_pages::integrations::upsert::IntegrationForm;
use web_pages::integrations::IntegrationOas3;
use web_pages::routes::integrations::View;
use web_pages::routes::integrations::{Delete, Index, Upsert};

/// Parses an OpenAPI specification from JSON string.
///
/// This function will:
/// 1. Parse the provided JSON string using the oas3 crate
/// 2. Return the parsed specification as JSON or an error
fn parse_openapi_spec(spec_json: &str) -> Result<Json<oas3::OpenApiV3Spec>, String> {
    match oas3::from_json(spec_json) {
        Ok(spec) => Ok(Json(spec)),
        Err(e) => Err(format!("Invalid OpenAPI JSON: {}", e)),
    }
}

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_get(view)
        .typed_get(new_edit_action)
        .typed_post(upsert_action)
        .typed_post(delete_action)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integrations = queries::integrations::integrations()
        .bind(&transaction)
        .all()
        .await?;

    // Get the Open API Spec
    let integrations: Vec<IntegrationOas3> = integrations
        .iter()
        .filter_map(|integration| {
            if let Some(definition) = &integration.definition {
                if let Ok(spec) = oas3::from_json(definition.to_string()) {
                    Some(IntegrationOas3 {
                        spec,
                        integration: integration.clone(),
                    })
                } else {
                    None
                }
            } else {
                tracing::error!("This integration doesn't have a definition");
                None
            }
        })
        .collect();

    let html = web_pages::integrations::index::page(team_id, rbac, integrations);

    Ok(Html(html))
}

pub async fn view(
    View { team_id, id }: View,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integration = db::queries::integrations::integration()
        .bind(&transaction, &id)
        .one()
        .await?;

    let tool_definitions = if let Some(definition) = &integration.definition {
        if let Ok(spec) = oas3::from_json(definition.to_string()) {
            create_tool_definitions_from_spec(spec).tool_definitions
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let logo_url = integration
        .definition
        .as_ref()
        .and_then(|def| oas3::from_json(def.to_string()).ok())
        .map(|spec| web_pages::integrations::get_logo_url(&spec.info.extensions))
        .unwrap_or_else(|| {
            web_pages::integrations::get_logo_url(&std::collections::BTreeMap::new())
        });

    let html =
        web_pages::integrations::view::view(team_id, rbac, integration, logo_url, tool_definitions);

    Ok(Html(html))
}

pub async fn new_edit_action(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integration_form = IntegrationForm::default();

    let html = web_pages::integrations::upsert::page(team_id, rbac, integration_form);

    Ok(Html(html))
}

pub async fn delete_action(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    queries::integrations::delete()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::integrations::Index { team_id }.to_string(),
        "Integration Deleted",
    )
}

pub async fn upsert_action(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(mut integration_form): Form<IntegrationForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integration_type = db::IntegrationType::OpenAPI;

    // Parse the OpenAPI specification from the provided JSON
    let (definition, integration_status, integration_name) =
        match parse_openapi_spec(&integration_form.openapi_spec) {
            Ok(spec) => {
                // Extract the name from the OpenAPI spec's info.title field
                let name = spec.0.info.title.clone();
                (Some(spec), db::IntegrationStatus::Configured, name)
            }
            Err(error) => {
                // If there's an error, return to the form with the error message
                integration_form.error = Some(error);
                let html =
                    web_pages::integrations::upsert::page(team_id, permissions, integration_form);
                return Ok(Html(html).into_response());
            }
        };

    // No configuration needed since we're not storing base URL anymore
    let configuration: Option<Json<serde_json::Value>> = None;

    match (integration_form.validate(), integration_form.id) {
        (Ok(_), Some(integration_id)) => {
            // The form is valid, update the integration
            queries::integrations::update()
                .bind(
                    &transaction,
                    &integration_name,
                    &configuration, // configuration
                    &definition,    // definition
                    &integration_type,
                    &integration_status,
                    &integration_id,
                )
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::integrations::Index { team_id }.to_string(),
                "Integration Updated",
            )
            .into_response())
        }
        (Ok(_), None) => {
            // The form is valid, create a new integration
            queries::integrations::insert()
                .bind(
                    &transaction,
                    &integration_name,
                    &configuration, // configuration
                    &definition,    // definition
                    &integration_type,
                    &integration_status,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::integrations::Index { team_id }.to_string(),
                "Integration Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::integrations::Index { team_id }.to_string(),
            "Problem with Integration Validation",
        )
        .into_response()),
    }
}
