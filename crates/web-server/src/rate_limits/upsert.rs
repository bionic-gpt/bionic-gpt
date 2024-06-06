use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use db::authz;
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::rate_limits::Upsert;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct RateLimitForm {
    pub id: Option<i32>,
    pub api_key_id: i32,
    pub tpm_limit: i32,
    pub rpm_limit: i32,
}

pub async fn upsert(
    Upsert { team_id }: Upsert,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(form): Form<RateLimitForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    match (form.validate(), form.id) {
        (Ok(_), Some(_id)) => Ok(super::super::layout::redirect_and_snackbar(
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

            Ok(super::super::layout::redirect_and_snackbar(
                &web_pages::routes::rate_limits::Index { team_id }.to_string(),
                "Rate Limit Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(super::super::layout::redirect_and_snackbar(
            &web_pages::routes::rate_limits::Index { team_id }.to_string(),
            "Problem with Rate Limit Validation",
        )
        .into_response()),
    }
}
