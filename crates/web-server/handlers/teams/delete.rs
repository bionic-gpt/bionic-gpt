use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::IntoResponse};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::routes::teams::Delete;

pub async fn delete(
    Delete { team_id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    queries::teams::delete()
        .bind(&transaction, &team_id)
        .await?;

    transaction.commit().await?;

    let transaction = client.transaction().await?;
    let team = queries::teams::get_primary_team()
        .bind(&transaction, &permissions.user_id)
        .one()
        .await?;
    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::console::Index { team_id: team.id }.to_string(),
        "Team Deleted",
    )
}
