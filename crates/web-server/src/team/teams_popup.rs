use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries;
use db::Pool;
use web_pages::{render_with_props, routes::team::Popup, team_members};

pub async fn index(
    Popup { team_id }: Popup,
    current_user: Authentication,
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
            team.team_name.clone().map(|name| {
                (
                    name,
                    web_pages::routes::team::Index { team_id: team.id }.to_string(),
                )
            })
        })
        .collect();

    let html = render_with_props(
        team_members::team_popup::Page,
        team_members::team_popup::PageProps { teams, team },
    );

    Ok(Html(html))
}
