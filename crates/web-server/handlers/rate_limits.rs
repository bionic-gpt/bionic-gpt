// Consolidated rate_limits.rs

use crate::{CustomError, Jwt};
use axum::response::Html;
use axum::Router;
use axum::{extract::Extension, response::IntoResponse};
use axum_extra::extract::Form;
use axum_extra::routing::RouterExt;
use db::{authz, queries, ModelType, Pool};
use serde::Deserialize;
use validator::Validate;
use web_pages::{
    rate_limits,
    routes::rate_limits::{Delete, Index, Upsert},
};

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
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

    if !rbac.can_manage_limits() {
        return Err(CustomError::Authorization);
    }

    let rate_limits = queries::rate_limits::rate_limits()
        .bind(&transaction)
        .all()
        .await?;

    let models = queries::models::models()
        .bind(&transaction, &ModelType::LLM)
        .all()
        .await?;

    // Fetch system-wide graph data for the last 7 days
    let token_usage_data = queries::token_usage_metrics::get_daily_token_usage_system_wide()
        .bind(&transaction, &"7")
        .all()
        .await?;

    let api_request_data = queries::token_usage_metrics::get_daily_api_request_count_system_wide()
        .bind(&transaction, &"7")
        .all()
        .await?;

    let html = rate_limits::index::page(
        rbac,
        team_id,
        rate_limits,
        models,
        token_usage_data,
        api_request_data,
    );

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
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_manage_limits() {
        return Err(CustomError::Authorization);
    }

    queries::rate_limits::delete()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::rate_limits::Index { team_id }.to_string(),
        "Rate Limit Deleted",
    )
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct RateLimitForm {
    pub id: Option<i32>,
    pub api_key_id: i32,
    pub tpm_limit: i32,
    pub rpm_limit: i32,
}

pub async fn upsert_action(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<RateLimitForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_manage_limits() {
        return Err(CustomError::Authorization);
    }

    match (form.validate(), form.id) {
        (Ok(_), Some(_id)) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::rate_limits::Index { team_id }.to_string(),
            "Not Implemented",
        )
        .into_response()),
        (Ok(_), None) => {
            // The form is valid save to the database
            queries::rate_limits::new()
                .bind(
                    &transaction,
                    &form.api_key_id,
                    &form.tpm_limit,
                    &form.rpm_limit,
                )
                .one()
                .await?;

            transaction.commit().await?;

            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::rate_limits::Index { team_id }.to_string(),
                "Rate Limit Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::rate_limits::Index { team_id }.to_string(),
            "Problem with Rate Limit Validation",
        )
        .into_response()),
    }
}
