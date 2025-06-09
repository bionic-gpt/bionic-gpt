use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::{Html, IntoResponse};
use axum::Form;
use db::{authz, queries, Json, Pool};
use validator::Validate;
use web_pages::integrations::upsert::IntegrationForm;
use web_pages::routes::integrations::{Delete, Edit, New};

use super::helpers::parse_openapi_spec;

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

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::integrations::Index { team_id }.to_string(),
        "Integration Deleted",
    ))
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
