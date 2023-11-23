use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries;
use db::Pool;

pub async fn index(
    current_user: Authentication,
    Path(organisation_id): Path<i32>,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let teams = queries::organisations::get_teams()
        .bind(&transaction, &current_user.user_id)
        .all()
        .await?;

    let organisation = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    let teams: Vec<(String, String)> = teams
        .iter()
        .filter_map(|team| {
            team.organisation_name
                .clone()
                .map(|name| (name, ui_components::routes::team::index_route(team.id)))
        })
        .collect();

    Ok(Html(ui_components::team_members::team_popup::team_popup(
        ui_components::team_members::team_popup::PageProps {
            teams,
            organisation,
        },
    )))
}
