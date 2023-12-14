use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use db::queries;
use db::Pool;

pub async fn switch(
    Path(team_id): Path<i32>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::teams::team()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    let teams = queries::teams::get_teams()
        .bind(&transaction, &current_user.user_id)
        .all()
        .await?;

    Ok(Html(ui_pages::teams::teams(teams, team.id, is_sys_admin)))
}
