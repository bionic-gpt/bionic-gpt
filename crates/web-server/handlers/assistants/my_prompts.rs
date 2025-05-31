use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries;
use db::Pool;
use web_pages::{assistants, routes::prompts::MyPrompts};

pub async fn my_prompts(
    MyPrompts { team_id }: MyPrompts,
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

    let html = assistants::my_prompts::page(team_id, rbac, prompts);

    Ok(Html(html))
}
