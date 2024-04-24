use super::super::{Authentication, CustomError};
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::authz;
use db::queries::{datasets, models};
use db::{ModelType, Pool};

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let datasets = datasets::datasets()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let models = models::models()
        .bind(&transaction, &ModelType::Embeddings)
        .all()
        .await?;


    let html = web_pages::render_with_props(
        web_pages::datasets::index::Page,
        web_pages::datasets::index::PageProps {
            team_id,
            rbac,
            datasets,
            models,
        },
    );

    Ok(Html(html))
}
