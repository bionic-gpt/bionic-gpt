use axum::{extract::State, Json};

use crate::{
    auth::ConnectionString,
    error::{ApiError, ApiResult},
    models::{ExecuteSqlRequest, ExecuteSqlResponse, ExplainQueryRequest, ExplainQueryResponse},
    state::AppState,
};

pub async fn execute_sql(
    State(_state): State<AppState>,
    ConnectionString(_conn): ConnectionString,
    Json(_request): Json<ExecuteSqlRequest>,
) -> ApiResult<Json<ExecuteSqlResponse>> {
    Err(ApiError::internal("not implemented"))
}

pub async fn explain_query(
    State(_state): State<AppState>,
    ConnectionString(_conn): ConnectionString,
    Json(_request): Json<ExplainQueryRequest>,
) -> ApiResult<Json<ExplainQueryResponse>> {
    Err(ApiError::internal("not implemented"))
}
