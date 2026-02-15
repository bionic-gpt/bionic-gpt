use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::{authz, queries, ModelType, Pool};
use web_pages::rate_limits;
use web_pages::routes::rate_limits::Index;

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.can_manage_limits() {
        return Err(CustomError::Authorization);
    }

    let rate_limits = queries::rate_limits::rate_limits()
        .bind(&transaction)
        .all()
        .await?;

    let models = queries::models::models()
        .bind(&transaction, &ModelType::LLM)
        .all()
        .await?;

    let token_usage_data = queries::token_usage_metrics::get_daily_token_usage_system_wide()
        .bind(&transaction, &"7")
        .all()
        .await?;

    let api_request_data = queries::token_usage_metrics::get_daily_api_request_count_system_wide()
        .bind(&transaction, &"7")
        .all()
        .await?;

    let html = rate_limits::page::page(
        rbac,
        team_id,
        rate_limits,
        models,
        token_usage_data,
        api_request_data,
    );

    Ok(Html(html))
}
