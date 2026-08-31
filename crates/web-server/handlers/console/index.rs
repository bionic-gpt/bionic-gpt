use agent_runtime::user_config::UserConfig;

use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries;
use db::Pool;
use web_pages::console;
use web_pages::routes::console::Index;

pub async fn index(
    Index { team_id }: Index,
    current_user: Jwt,
    user_config: UserConfig,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let prompts = queries::models::all_models()
        .bind(&transaction)
        .all()
        .await?;

    let model_id = if let Some(default_model) = user_config.default_model {
        default_model
    } else {
        prompts.first().unwrap().id
    };

    let prompt = queries::models::model_config()
        .bind(&transaction, &model_id)
        .one()
        .await;

    let prompt = if let Ok(prompt) = prompt {
        prompt
    } else {
        let id = prompts.first().unwrap().id;
        queries::models::model_config()
            .bind(&transaction, &id)
            .one()
            .await?
    };

    let capabilities = queries::capabilities::get_model_capabilities()
        .bind(&transaction, &prompt.id)
        .all()
        .await?;
    let html = console::page::new_conversation(team_id, prompts, prompt, rbac, capabilities);

    Ok(Html(html))
}
