use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Form;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, queries, Json, Pool, Visibility};
use integrations::bionic_openapi::BionicOpenAPI;
use serde::Deserialize;
use validator::Validate;
use web_pages::integrations::upsert::IntegrationForm;
use web_pages::routes::integrations::View;
use web_pages::routes::integrations::{ConfigureApiKey, Delete, Edit, Index, New};

#[derive(Deserialize, Validate, Debug)]
pub struct ApiKeyForm {
    #[validate(length(min = 1, message = "API key is required"))]
    pub api_key: String,
    pub visibility: String,
}

pub async fn configure_api_key_action(
    ConfigureApiKey {
        team_id,
        integration_id,
    }: ConfigureApiKey,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(api_key_form): Form<ApiKeyForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Parse visibility
    let visibility = match api_key_form.visibility.as_str() {
        "Private" => Visibility::Private,
        "Team" => Visibility::Team,
        _ => Visibility::Private,
    };

    // Validate the form
    match api_key_form.validate() {
        Ok(_) => {
            // Insert new connection
            let _connection_id = queries::connections::insert_api_key_connection()
                .bind(
                    &transaction,
                    &integration_id,
                    &team_id,
                    &visibility,
                    &api_key_form.api_key,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::integrations::Index { team_id }.to_string(),
                "API Key configured successfully",
            ))
        }
        Err(_) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::integrations::Index { team_id }.to_string(),
            "Invalid API key configuration",
        )),
    }
}

pub fn routes() -> Router {
    Router::new()
        // Loaders
        .typed_get(loader)
        .typed_get(view_loader)
        .typed_get(new_loader)
        .typed_get(edit_loader)
        // Actions
        .typed_post(new_action)
        .typed_post(edit_action)
        .typed_post(delete_action)
        .typed_post(configure_api_key_action)
}

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
    let integrations: Vec<(BionicOpenAPI, i32)> = integrations
        .iter()
        .filter_map(|integration| {
            if let Some(definition) = &integration.definition {
                if let Ok(bionic_openapi) = BionicOpenAPI::new(definition) {
                    Some((bionic_openapi, integration.id))
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

pub async fn view_loader(
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

    let (logo_url, description, tool_definitions) =
        if let Some(definition) = &integration.definition {
            match BionicOpenAPI::new(definition) {
                Ok(openapi_helper) => {
                    let logo_url = openapi_helper.get_logo_url();
                    let description = openapi_helper.get_description();
                    let integration_tools = openapi_helper.create_tool_definitions();
                    (logo_url, description, integration_tools.tool_definitions)
                }
                Err(_) => {
                    // If parsing fails, use defaults
                    (String::new(), None, vec![])
                }
            }
        } else {
            (String::new(), None, vec![])
        };

    let html = web_pages::integrations::view::view(
        team_id,
        rbac,
        integration,
        logo_url,
        description,
        tool_definitions,
    );

    Ok(Html(html))
}

pub async fn new_loader(
    New { team_id }: New,
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

pub async fn edit_loader(
    Edit { team_id, id }: Edit,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integration = queries::integrations::integration()
        .bind(&transaction, &id)
        .one()
        .await?;

    let integration_form = if let Some(definition) = &integration.definition {
        IntegrationForm {
            id: Some(integration.id),
            openapi_spec: serde_json::to_string(&definition).unwrap_or("".to_string()),
            error: None,
        }
    } else {
        IntegrationForm::default()
    };

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

pub async fn edit_action(
    Edit { team_id, id }: Edit,
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

    match integration_form.validate() {
        Ok(_) => {
            // The form is valid, update the integration
            queries::integrations::update()
                .bind(
                    &transaction,
                    &integration_name,
                    &configuration, // configuration
                    &definition,    // definition
                    &integration_type,
                    &integration_status,
                    &id,
                )
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::integrations::Index { team_id }.to_string(),
                "Integration Updated",
            )
            .into_response())
        }
        Err(_) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::integrations::Index { team_id }.to_string(),
            "Problem with Integration Validation",
        )
        .into_response()),
    }
}

pub async fn new_action(
    New { team_id }: New,
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

    match integration_form.validate() {
        Ok(_) => {
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
        Err(_) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::integrations::Index { team_id }.to_string(),
            "Problem with Integration Validation",
        )
        .into_response()),
    }
}
