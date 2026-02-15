use crate::CustomError;
use db::queries;

pub struct ApiKeysPageData {
    pub api_keys: Vec<db::ApiKey>,
    pub assistants: Vec<db::Prompt>,
    pub models: Vec<db::Prompt>,
    pub token_usage_data: Vec<db::queries::token_usage_metrics::DailyTokenUsage>,
    pub api_request_data: Vec<db::queries::token_usage_metrics::DailyApiRequests>,
}

pub async fn load_api_keys_page_data(
    transaction: &db::Transaction<'_>,
    team_id_num: i32,
) -> Result<ApiKeysPageData, CustomError> {
    let api_keys = queries::api_keys::api_keys()
        .bind(transaction, &team_id_num)
        .all()
        .await?;

    let assistants = queries::prompts::prompts()
        .bind(transaction, &team_id_num, &db::PromptType::Assistant)
        .all()
        .await?;

    let models = queries::prompts::prompts()
        .bind(transaction, &team_id_num, &db::PromptType::Model)
        .all()
        .await?;

    let token_usage_data = queries::token_usage_metrics::get_daily_token_usage_for_team()
        .bind(transaction, &team_id_num, &"7")
        .all()
        .await?;

    let api_request_data = queries::token_usage_metrics::get_daily_api_request_count_for_team()
        .bind(transaction, &team_id_num, &"7")
        .all()
        .await?;

    Ok(ApiKeysPageData {
        api_keys,
        assistants,
        models,
        token_usage_data,
        api_request_data,
    })
}
