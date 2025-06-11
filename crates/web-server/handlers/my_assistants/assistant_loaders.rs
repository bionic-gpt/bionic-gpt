use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::{authz, queries, Pool};
use web_pages::{
    my_assistants,
    routes::prompts::{MyAssistants, View},
};

pub async fn my_assistants(
    MyAssistants { team_id }: MyAssistants,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let prompts = queries::prompts::my_prompts()
        .bind(&transaction, &db::PromptType::Assistant)
        .all()
        .await?;

    let html = my_assistants::index::page(team_id, rbac, prompts);

    Ok(Html(html))
}

pub async fn view_assistant(
    View { team_id, prompt_id }: View,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let datasets = queries::prompts::prompt_datasets()
        .bind(&transaction, &prompt_id)
        .all()
        .await?;

    let integrations = queries::prompt_integrations::prompt_integrations()
        .bind(&transaction, &prompt_id)
        .all()
        .await?;

    let html = my_assistants::view::page(team_id, rbac, prompt, datasets, integrations);

    Ok(Html(html))
}
