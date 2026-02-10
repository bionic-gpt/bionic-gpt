use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::{queries, Pool};
use web_pages::{api_keys, routes::api_keys::Index};

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.can_use_api_keys() {
        return Err(CustomError::Authorization);
    }

    let api_keys = queries::api_keys::api_keys()
        .bind(&transaction, &team_id_num)
        .all()
        .await?;

    let assistants = queries::prompts::prompts()
        .bind(&transaction, &team_id_num, &db::PromptType::Assistant)
        .all()
        .await?;

    let models = queries::prompts::prompts()
        .bind(&transaction, &team_id_num, &db::PromptType::Model)
        .all()
        .await?;

    // Fetch graph data for the last 7 days
    let token_usage_data = queries::token_usage_metrics::get_daily_token_usage_for_team()
        .bind(&transaction, &team_id_num, &"7")
        .all()
        .await?;

    let api_request_data = queries::token_usage_metrics::get_daily_api_request_count_for_team()
        .bind(&transaction, &team_id_num, &"7")
        .all()
        .await?;

    let html = api_keys::page::page(
        rbac,
        team_id,
        api_keys,
        assistants,
        models,
        token_usage_data,
        api_request_data,
        None,
    );

    Ok(Html(html))
}
