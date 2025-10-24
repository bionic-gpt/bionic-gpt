use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    auth::ConnectionString,
    error::{ApiError, ApiResult},
    models::GetTopQueriesResponse,
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

pub async fn top_queries(
    State(_state): State<AppState>,
    ConnectionString(_conn): ConnectionString,
    Query(params): Query<TopQueriesParams>,
) -> ApiResult<Json<GetTopQueriesResponse>> {
    let _ = (params.interval_minutes, params.limit);
    Err(ApiError::internal("not implemented"))
}
