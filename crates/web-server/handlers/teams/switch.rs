use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::queries;
use db::Pool;
use std::collections::HashMap;
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
    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let teams = queries::teams::get_teams()
        .bind(&transaction, &rbac.user_id)
        .all()
        .await?;

    let invites = queries::invitations::get_by_user()
        .bind(&transaction)
        .all()
        .await?;

    let mut member_counts = HashMap::new();
    for team_entry in &teams {
        let member_count = queries::teams::get_users()
            .bind(&transaction, &team_entry.id)
            .all()
            .await?
            .len();
        member_counts.insert(team_entry.id, member_count);
    }

    let html = teams::page::page(
        rbac,
        team_id,
        teams,
        invites,
        current_user_email,
        member_counts,
    );

    Ok(Html(html))
}
