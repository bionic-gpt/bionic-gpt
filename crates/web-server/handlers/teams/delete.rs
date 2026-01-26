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
    let (permissions, team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    queries::teams::delete()
        .bind(&transaction, &team_id_num)
        .await?;

    transaction.commit().await?;

    let transaction = client.transaction().await?;
    let team = queries::teams::get_primary_team()
        .bind(&transaction, &permissions.user_id)
        .one()
        .await?;
    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::teams::Switch { team_id: team.slug }.to_string(),
        "Team Deleted",
    )
}
