use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::Pool;
use db::{queries, ModelType};
use web_pages::{prompts, render_with_props, routes::prompts::Index};

pub async fn index(
    Index { team_id }: Index,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let prompts = queries::prompts::prompts()
        .bind(&transaction, &team_id, &db::PromptType::Assistant)
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

    let categories = queries::categories::categories()
        .bind(&transaction)
        .all()
        .await?;

    let html = render_with_props(
        prompts::index::Page,
        prompts::index::PageProps {
            team_id,
            rbac,
            prompts,
            datasets,
            models,
            categories,
        },
    );

    Ok(Html(html))
}
