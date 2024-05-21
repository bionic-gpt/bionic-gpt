use super::super::{Authentication, CustomError};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::queries;
use db::Pool;
use web_pages::{render_with_props, routes::team::Switch, teams};

pub async fn switch(
    Switch { team_id }: Switch,
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

    let teams = queries::teams::get_teams()
        .bind(&transaction, &rbac.user_id)
        .all()
        .await?;

    let invites = queries::invitations::get_by_user()
        .bind(&transaction)
        .all()
        .await?;

    let html = render_with_props(
        teams::index::Page,
        teams::index::PageProps {
            teams,
            team_id: team.id,
            rbac,
            invites,
        },
    );

    Ok(Html(html))
}
