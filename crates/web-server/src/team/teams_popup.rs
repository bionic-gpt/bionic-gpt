use super::super::{Authentication, CustomError};
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::authz;
use db::queries;
use db::Pool;

pub async fn index(
    current_user: Authentication,
    Path(team_id): Path<i32>,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let teams = queries::teams::get_teams()
        .bind(&transaction, &rbac.user_id)
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
                .map(|name| (name, web_pages::routes::team::index_route(team.id)))
        })
        .collect();

    Ok(Html(web_pages::team_members::team_popup::team_popup(
        web_pages::team_members::team_popup::PageProps { teams, team },
    )))
}
