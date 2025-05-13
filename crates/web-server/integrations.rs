use super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Form;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, queries, Json, Pool};
use validator::Validate;
use web_pages::integrations::upsert::IntegrationForm;
use web_pages::routes::integrations::{Delete, Index, Upsert};

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

    let integrations = queries::integrations::integrations()
        .bind(&transaction)
        .all()
        .await?;

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
    Form(integration_form): Form<IntegrationForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let integration_type = db::IntegrationType::OpenAPI;

    let integration_status = db::IntegrationStatus::AwaitingConfiguration;

    let none: Option<Json<String>> = None;

    match (integration_form.validate(), integration_form.id) {
        (Ok(_), Some(integration_id)) => {
            // The form is valid, update the integration
            queries::integrations::update()
                .bind(
                    &transaction,
                    &integration_form.name,
                    &none,
                    &none,
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
                    &none,
                    &none,
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
