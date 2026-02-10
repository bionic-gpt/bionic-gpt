use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::{queries, Pool};
use web_pages::{automations, routes::automations::Index};

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let prompts = queries::prompts::my_prompts()
        .bind(&transaction, &db::PromptType::Automation)
        .all()
        .await?;

    let html = automations::page::page(team_id, rbac, prompts);

    Ok(Html(html))
}
