use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::rls;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries;
use db::Pool;

pub async fn index(
    current_user: Authentication,
    Path(team_id): Path<i32>,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _is_sys_admin = rls::set_row_level_security_user(&transaction, &current_user).await?;

    let teams = queries::teams::get_teams()
        .bind(&transaction, &current_user.user_id)
        .all()
        .await?;

    let team = queries::teams::team()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    let teams: Vec<(String, String)> = teams
        .iter()
        .filter_map(|team| {
            team.team_name
                .clone()
                .map(|name| (name, ui_pages::routes::team::index_route(team.id)))
        })
        .collect();

    Ok(Html(ui_pages::team_members::team_popup::team_popup(
        ui_pages::team_members::team_popup::PageProps { teams, team },
    )))
}
