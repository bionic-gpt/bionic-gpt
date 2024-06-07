use axum::{response::IntoResponse, Extension};
use axum_extra::routing::TypedPath;
use db::Pool;
use serde::Deserialize;

use crate::CustomError;

#[derive(TypedPath, Deserialize)]
#[typed_path("/app/metrics")]
pub struct Metrics {}

pub async fn track_metrics(
    Metrics {}: Metrics,
    Extension(_pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    Ok("Test".to_string())
}
