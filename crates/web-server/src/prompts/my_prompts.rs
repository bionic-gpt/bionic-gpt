use crate::config::Config;

use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::Pool;
use db::{queries, ModelType};
use web_pages::{prompts, render_with_props, routes::prompts::MyPrompts};

pub async fn my_prompts(
    MyPrompts { team_id }: MyPrompts,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let prompts = queries::prompts::my_prompts()
        .bind(&transaction, &db::PromptType::Assistant)
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
        prompts::my_prompts::Page,
        prompts::my_prompts::PageProps {
            team_id,
            rbac,
            prompts,
            datasets,
            models,
            categories,
            is_saas: config.saas,
        },
    );

    Ok(Html(html))
}
