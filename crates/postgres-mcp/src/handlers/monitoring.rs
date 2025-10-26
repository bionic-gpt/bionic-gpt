use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    auth::ConnectionString,
    db,
    error::{ApiError, ApiResult},
    models::{GetTopQueriesResponse, TopQuery},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct TopQueriesParams {
    #[serde(default = "default_interval")]
    pub interval_minutes: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_interval() -> i64 {
    60
}

fn default_limit() -> i64 {
    10
}

const MAX_INTERVAL_MINUTES: i64 = 7 * 24 * 60;
const MAX_TOP_QUERIES: i64 = 50;

pub async fn top_queries(
    State(_state): State<AppState>,
    ConnectionString(conn): ConnectionString,
    Query(params): Query<TopQueriesParams>,
) -> ApiResult<Json<GetTopQueriesResponse>> {
    let client = db::connect(&conn).await?;
    let interval = params.interval_minutes.clamp(1, MAX_INTERVAL_MINUTES);
    let limit = params.limit.clamp(1, MAX_TOP_QUERIES);

    let has_extension: bool = client
        .query_one(
            "SELECT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'pg_stat_statements')",
            &[],
        )
        .await?
        .get(0);

    if !has_extension {
        return Err(ApiError::internal(
            "pg_stat_statements extension is not installed on the target database",
        ));
    }

    let has_last_exec_time: bool = client
        .query_one(
            "SELECT EXISTS (
                SELECT 1
                FROM information_schema.columns
                WHERE table_schema = 'public'
                  AND table_name = 'pg_stat_statements'
                  AND column_name = 'last_exec_time'
            )",
            &[],
        )
        .await?
        .get(0);

    let rows = if has_last_exec_time {
        client
            .query(
                "SELECT query,
                        calls,
                        total_time,
                        mean_time,
                        rows,
                        shared_blks_hit,
                        shared_blks_read
                 FROM pg_stat_statements
                 WHERE last_exec_time >= now() - make_interval(mins => $1::double precision)
                 ORDER BY total_time DESC
                 LIMIT $2",
                &[&(interval as f64), &limit],
            )
            .await?
    } else {
        client
            .query(
                "SELECT query,
                        calls,
                        total_time,
                        mean_time,
                        rows,
                        shared_blks_hit,
                        shared_blks_read
                 FROM pg_stat_statements
                 ORDER BY total_time DESC
                 LIMIT $1",
                &[&limit],
            )
            .await?
    };

    let queries = rows
        .into_iter()
        .map(|row| TopQuery {
            query: row.get::<_, String>("query"),
            calls: row.get::<_, i64>("calls"),
            total_time_ms: row.get::<_, f64>("total_time"),
            mean_time_ms: row.get::<_, f64>("mean_time"),
            rows: Some(row.get::<_, i64>("rows")),
            shared_blks_hit: Some(row.get::<_, i64>("shared_blks_hit")),
            shared_blks_read: Some(row.get::<_, i64>("shared_blks_read")),
        })
        .collect();

    Ok(Json(GetTopQueriesResponse { queries }))
}
