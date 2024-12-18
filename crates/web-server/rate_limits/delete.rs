use super::super::{CustomError, Jwt};
use axum::{extract::Extension, response::IntoResponse};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::routes::rate_limits::Delete;

pub async fn delete(
    Delete { id, team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    queries::rate_limits::delete()
        .bind(&transaction, &id)
        .await?;

    transaction.commit().await?;

    super::super::layout::redirect_and_snackbar(
        &web_pages::routes::rate_limits::Index { team_id }.to_string(),
        "Rate Limit Deleted",
    )
}
