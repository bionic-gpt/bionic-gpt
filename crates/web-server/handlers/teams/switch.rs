use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::{routes::teams::Switch, teams};

pub async fn switch(
    Switch { team_id }: Switch,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let current_user_email = current_user.email.clone();
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let team = queries::teams::team()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    let teams = queries::teams::get_teams()
        .bind(&transaction, &rbac.user_id)
        .all()
        .await?;

    let invites = queries::invitations::get_by_user()
        .bind(&transaction)
        .all()
        .await?;

    let html = teams::index::page(rbac, team.id, teams, invites, current_user_email);

    Ok(Html(html))
}
