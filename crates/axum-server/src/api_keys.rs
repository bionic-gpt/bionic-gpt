use axum::{routing::get, Router};

use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::Pool;

use ui_components::routes::api_keys::INDEX;

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index))
}

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    Ok(Html(ui_components::api_keys::api_keys(team_id)))
}
