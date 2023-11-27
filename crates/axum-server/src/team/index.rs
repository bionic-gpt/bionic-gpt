use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use db::queries;
use db::types;
use db::Pool;

pub async fn index(
    Path(organisation_id): Path<i32>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let organisation = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    let members = queries::organisations::get_users()
        .bind(&transaction, &organisation_id)
        .all()
        .await?;

    let permissions: Vec<types::public::Permission> = queries::rbac::permissions()
        .bind(&transaction, &current_user.user_id, &organisation_id)
        .all()
        .await?;

    let can_manage_team = permissions
        .iter()
        .any(|p| p == &types::public::Permission::ManageTeam);

    let user = queries::users::user()
        .bind(&transaction, &current_user.user_id)
        .one()
        .await?;

    let invites = queries::invitations::get_all()
        .bind(&transaction, &organisation_id)
        .all()
        .await?;

    let team_name = if let Some(team) = &organisation.name {
        format!("Team : {}", team)
    } else {
        "Team : No Name ".to_string()
    };

    Ok(Html(ui_pages::team_members::members::members(
        ui_pages::team_members::members::PageProps {
            invites,
            members,
            organisation,
            user,
            team_name,
            can_manage_team,
        },
    )))
}
