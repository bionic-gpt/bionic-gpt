use crate::{CustomError, Jwt};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
};
use axum_extra::{extract::Form, routing::RouterExt};
use db::authz;
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

use axum::Router;
use web_pages::routes::profile::{Profile, SetDetails as SetDetailsRoute};

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_post(set_details_action)
}

fn index_route(team_slug: &str) -> String {
    Profile {
        team_id: team_slug.to_string(),
    }
    .to_string()
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct SetDetails {
    #[validate(length(min = 1, message = "The first name is mandatory"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "The last name is mandatory"))]
    pub last_name: String,
}

pub async fn loader(
    Profile { team_id: team_slug }: Profile,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_slug).await?;

    let user = queries::users::user()
        .bind(&transaction, &rbac.user_id)
        .one()
        .await?;

    Ok(Html(web_pages::profile::profile(user, team_slug, rbac)))
}

pub async fn set_details_action(
    SetDetailsRoute { team_id: team_slug }: SetDetailsRoute,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(set_name): Form<SetDetails>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_slug).await?;

    queries::users::set_name()
        .bind(
            &transaction,
            &set_name.first_name,
            &set_name.last_name,
            &rbac.user_id,
        )
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(&index_route(&team_slug), "Details Updated")
}
