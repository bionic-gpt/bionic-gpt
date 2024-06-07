use axum::{response::IntoResponse, Extension};
use axum_extra::routing::TypedPath;
use db::{queries::inference_metrics, Pool};
use serde::Deserialize;

use crate::CustomError;

#[derive(TypedPath, Deserialize)]
#[typed_path("/app/metrics")]
pub struct Metrics {}

pub async fn track_metrics(
    Metrics {}: Metrics,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;

    let inference_metrics = inference_metrics::inference_metrics()
        .bind(&transaction)
        .all()
        .await?;

    let mut prometheus_metrics = "".to_string();

    for im in inference_metrics {
        prometheus_metrics.push_str(&format!(
            "tokens_per_second_sent{{model=\"{}\"}} {}",
            im.model_id, im.tpm_sent
        ))
    }

    Ok(prometheus_metrics)
}
