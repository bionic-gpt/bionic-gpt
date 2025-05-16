use super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Form;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, queries, Json, Pool};
use integrations;
use validator::Validate;
use web_pages::integrations::upsert::IntegrationForm;
use web_pages::routes::integrations::{Delete, Index, Upsert};

/// Fetches and parses an OpenAPI specification from a URL.
///
/// This function will:
/// 1. Fetch the content from the provided URL
/// 2. Parse it as JSON using the oas3 crate
/// 3. Return the parsed specification as JSON or an error
async fn fetch_and_parse_openapi(base_url: &str) -> Result<Json<oas3::OpenApiV3Spec>, String> {
    // Fetch the content from the base_url
    let response = match reqwest::get(base_url).await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Failed to fetch OpenAPI spec: {}", e)),
    };

    dbg!(&response);

    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch OpenAPI spec: HTTP {}",
            response.status()
        ));
    }

    let content = match response.text().await {
        Ok(text) => text,
        Err(e) => return Err(format!("Failed to read response: {}", e)),
    };

    // Parse as JSON
    match oas3::from_json(&content) {
        Ok(spec) => {
            // Successfully parsed as JSON
            Ok(Json(spec))
        }
        Err(e) => Err(format!("Invalid OpenAPI JSON: {}", e)),
    }
}

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
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

    let external_integrations = queries::integrations::integrations()
        .bind(&transaction)
        .all()
        .await?;

    let integrations = integrations::get_integrations(external_integrations);

    let html = web_pages::integrations::index::page(team_id, rbac, integrations);

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

    super::layout::redirect_and_snackbar(
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

    // Fetch and parse the OpenAPI specification
    let (definition, integration_status) =
        match fetch_and_parse_openapi(&integration_form.base_url).await {
            Ok(spec) => (Some(spec), db::IntegrationStatus::Configured),
            Err(error) => {
                // If there's an error, return to the form with the error message
                integration_form.error = Some(error);
                let html =
                    web_pages::integrations::upsert::page(team_id, permissions, integration_form);
                return Ok(Html(html).into_response());
            }
        };

    let none: Option<Json<String>> = None;

    match (integration_form.validate(), integration_form.id) {
        (Ok(_), Some(integration_id)) => {
            // The form is valid, update the integration
            queries::integrations::update()
                .bind(
                    &transaction,
                    &integration_form.name,
                    &none,       // configuration
                    &definition, // definition
                    &integration_type,
                    &integration_status,
                    &integration_id,
                )
                .await?;

            transaction.commit().await?;

            Ok(super::layout::redirect_and_snackbar(
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
                    &integration_form.name,
                    &none,       // configuration
                    &definition, // definition
                    &integration_type,
                    &integration_status,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(super::layout::redirect_and_snackbar(
                &web_pages::routes::integrations::Index { team_id }.to_string(),
                "Integration Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(super::layout::redirect_and_snackbar(
            &web_pages::routes::integrations::Index { team_id }.to_string(),
            "Problem with Integration Validation",
        )
        .into_response()),
    }
}
