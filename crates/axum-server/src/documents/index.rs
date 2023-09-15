use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries::{datasets, documents};
use db::Pool;

pub async fn index(
    Path((team_id, dataset_id)): Path<(i32, i32)>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let docs = documents::documents()
        .bind(&transaction, &dataset_id)
        .all()
        .await?;

    let dataset = datasets::dataset()
        .bind(&transaction, &dataset_id)
        .one()
        .await?;

    Ok(Html(ui_components::documents::index(
        team_id,
        ui_components::routes::documents::upload_route(team_id, dataset_id),
        dataset,
        docs,
    )))
}
