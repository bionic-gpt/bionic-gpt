use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::IntoResponse};
use axum_extra::extract::Form;
use db::{authz, queries, Pool};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::rate_limits::{Delete, Upsert};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct RateLimitForm {
    pub id: Option<i32>,
    pub api_key_id: i32,
    pub tpm_limit: i32,
    pub rpm_limit: i32,
}

pub async fn action_delete(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

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

pub async fn action_upsert(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<RateLimitForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

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
