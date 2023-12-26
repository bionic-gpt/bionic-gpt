use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::authz;
use db::Pool;
use db::{queries, ModelType};

pub async fn index(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, current_user.into(), team_id).await?;

    let prompts = queries::prompts::prompts()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let datasets = queries::datasets::datasets()
        .bind(&transaction)
        .all()
        .await?;

    let models = queries::models::models()
        .bind(&transaction, &ModelType::LLM)
        .all()
        .await?;

    Ok(Html(ui_pages::prompts::index(
        ui_pages::prompts::index::PageProps {
            team_id,
            rbac,
            prompts,
            datasets,
            models,
        },
    )))
}
