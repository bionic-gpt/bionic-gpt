use axum::{
    extract::{Query, State},
    Json,
};

use crate::{
    auth::ConnectionString,
    error::{ApiError, ApiResult},
    models::{
        AnalyzeDbHealthResponse, AnalyzeIndexesResponse, AnalyzeQueryIndexesRequest,
        AnalyzeWorkloadIndexesRequest,
    },
    state::AppState,
};

pub async fn analyze_workload_indexes(
    State(_state): State<AppState>,
    ConnectionString(_conn): ConnectionString,
    Json(request): Json<AnalyzeWorkloadIndexesRequest>,
) -> ApiResult<Json<AnalyzeIndexesResponse>> {
    let _ = request;
    Err(ApiError::internal("not implemented"))
}

pub async fn analyze_query_indexes(
    State(_state): State<AppState>,
    ConnectionString(_conn): ConnectionString,
    Json(request): Json<AnalyzeQueryIndexesRequest>,
) -> ApiResult<Json<AnalyzeIndexesResponse>> {
    let _ = request;
    Err(ApiError::internal("not implemented"))
}

pub async fn db_health(
    State(_state): State<AppState>,
    ConnectionString(_conn): ConnectionString,
    Query(params): Query<DbHealthParams>,
) -> ApiResult<Json<AnalyzeDbHealthResponse>> {
    let _ = params.include_diagnostics;
    Err(ApiError::internal("not implemented"))
}

#[derive(Debug, serde::Deserialize)]
pub struct DbHealthParams {
    #[serde(default)]
    pub include_diagnostics: bool,
}
