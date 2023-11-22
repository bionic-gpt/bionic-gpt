use axum::{routing::get, Router};

use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::Pool;
use ui_components::training;

use ui_components::routes::training::INDEX;

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

    Ok(Html(training::index(training::PageProps {
        organisation_id: team_id,
    })))
}
