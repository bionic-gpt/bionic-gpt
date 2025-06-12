use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::{authz, queries, Pool};
use web_pages::{my_assistants, routes::prompts::MyAssistants};

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
