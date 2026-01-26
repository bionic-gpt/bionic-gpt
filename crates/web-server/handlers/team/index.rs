use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::{routes::team::Index, team};

pub async fn index(
    Index { team_id }: Index,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    let team = queries::teams::team()
        .bind(&transaction, &team_id_num)
        .one()
        .await?;

    let members = queries::teams::get_users()
        .bind(&transaction, &team_id_num)
        .all()
        .await?;

    let user = queries::users::user()
        .bind(&transaction, &rbac.user_id)
        .one()
        .await?;

    let invites = queries::invitations::get_all()
        .bind(&transaction, &team_id_num)
        .all()
        .await?;

    let team_name = if let Some(team) = &team.name {
        format!("Team : {}", team)
    } else {
        "Team : No Name ".to_string()
    };

    let html = team::members::page(rbac, members, invites, team, user, team_name);

    Ok(Html(html))
}
