use super::super::{CustomError, Jwt};
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use db::authz;
use db::queries;
use db::Pool;

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let team = queries::teams::team()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    let user = queries::users::user()
        .bind(&transaction, &rbac.user_id)
        .one()
        .await?;

    Ok(Html(web_pages::profile::profile(user, team.id, rbac)))
}
