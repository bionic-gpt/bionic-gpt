use db::create_pool;
use serde_json::json;
use tokio_postgres::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("APP_DATABASE_URL")?;
    let hostname_url = std::env::var("HOSTNAME_URL").unwrap_or_default();
    let ip_address = std::env::var("IP_ADDRESS").unwrap_or_default();

    let pool = create_pool(&database_url);
    let client = pool.get().await?;

    let user_count: i64 = client
        .query_one("SELECT COUNT(id) FROM users", &[])
        .await?
        .get(0);

    let assistant_count: i64 = client
        .query_one(
            "SELECT COUNT(id) FROM prompts WHERE prompt_type = 'Assistant'",
            &[],
        )
        .await?
        .get(0);

    let tokens_prompt: i64 = client
        .query_one(
            "SELECT COALESCE(SUM(tokens),0) FROM token_usage_metrics WHERE type = 'Prompt' AND created_at >= NOW() - INTERVAL '30 days'",
            &[],
        )
        .await?
        .get(0);

    let tokens_completion: i64 = client
        .query_one(
            "SELECT COALESCE(SUM(tokens),0) FROM token_usage_metrics WHERE type = 'Completion' AND created_at >= NOW() - INTERVAL '30 days'",
            &[],
        )
        .await?
        .get(0);

    let days_since_first_registration: i64 = client
        .query_one(
            "SELECT EXTRACT(DAY FROM current_timestamp - MIN(created_at))::int FROM users",
            &[],
        )
        .await?
        .get(0);

    let report = json!({
        "hostname_url": hostname_url,
        "ip_address": ip_address,
        "user_count": user_count,
        "assistant_count": assistant_count,
        "tokens_prompt_last_month": tokens_prompt,
        "tokens_completion_last_month": tokens_completion,
        "days_since_first_registration": days_since_first_registration
    });

    println!("{}", serde_json::to_string_pretty(&report)?);

    Ok(())
}
