use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::authz;
use db::queries::{datasets, documents};
use db::Pool;

pub async fn index(
    Path((team_id, dataset_id)): Path<(i32, i32)>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let documents = documents::documents()
        .bind(&transaction, &dataset_id)
        .all()
        .await?;

    let dataset = datasets::dataset()
        .bind(&transaction, &dataset_id)
        .one()
        .await?;

    Ok(Html(ui_pages::documents::index(
        ui_pages::documents::index::PageProps {
            team_id,
            rbac,
            dataset,
            documents,
        },
    )))
}
