use super::super::{Authentication, CustomError};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::queries;
use db::types;
use db::Pool;
use web_pages::{render_with_props, routes::team::Index, team};

pub async fn index(
    Index { team_id }: Index,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let team = queries::teams::team()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    let members = queries::teams::get_users()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let permissions: Vec<types::public::Permission> = queries::rbac::permissions()
        .bind(&transaction, &rbac.user_id, &team_id)
        .all()
        .await?;

    let can_manage_team = permissions
        .iter()
        .any(|p| p == &types::public::Permission::InvitePeopleToTeam);

    let user = queries::users::user()
        .bind(&transaction, &rbac.user_id)
        .one()
        .await?;

    let invites = queries::invitations::get_all()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let team_name = if let Some(team) = &team.name {
        format!("Team : {}", team)
    } else {
        "Team : No Name ".to_string()
    };

    let html = render_with_props(
        team::members::Page,
        team::members::PageProps {
            invites,
            rbac,
            members,
            team,
            user,
            team_name,
            can_manage_team,
        },
    );

    Ok(Html(html))
}
