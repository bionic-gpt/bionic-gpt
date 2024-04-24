use super::super::{Authentication, CustomError};
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::authz;
use db::{queries, Pool};
use web_pages::{pipelines, render_with_props};

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let pipelines = queries::document_pipelines::document_pipelines()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let datasets = queries::datasets::datasets()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let html = render_with_props(
        pipelines::index::Page,
        pipelines::index::PageProps {
            pipelines,
            datasets,
            team_id,
            rbac,
        },
    );

    Ok(Html(html))
}
