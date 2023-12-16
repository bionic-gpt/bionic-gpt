use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use db::queries;
use db::types;
use db::Pool;

pub async fn index(
    Path(team_id): Path<i32>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::teams::team()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    let members = queries::teams::get_users()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let permissions: Vec<types::public::Permission> = queries::rbac::permissions()
        .bind(&transaction, &current_user.user_id, &team_id)
        .all()
        .await?;

    let can_manage_team = permissions
        .iter()
        .any(|p| p == &types::public::Permission::InvitePeopleToTeam);

    let user = queries::users::user()
        .bind(&transaction, &current_user.user_id)
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

    Ok(Html(ui_pages::team_members::members::members(
        ui_pages::team_members::members::PageProps {
            invites,
            is_sys_admin: rbac.is_sys_admin,
            members,
            team,
            user,
            team_name,
            can_manage_team,
        },
    )))
}
