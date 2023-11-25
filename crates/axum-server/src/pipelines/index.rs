use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::{queries, Pool};

pub async fn index(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    crate::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let pipelines = queries::document_pipelines::document_pipelines()
        .bind(&transaction, &organisation_id)
        .all()
        .await?;

    let datasets = queries::datasets::datasets()
        .bind(&transaction)
        .all()
        .await?;

    Ok(Html(ui_pages::pipelines::index::index(
        ui_pages::pipelines::index::PageProps {
            pipelines,
            datasets,
            organisation_id,
        },
    )))
}
