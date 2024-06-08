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

    let inference_metrics = inference_metrics::inference_models()
        .bind(&transaction)
        .all()
        .await?;

    let mut prometheus_metrics = "".to_string();

    for im in &inference_metrics {
        prometheus_metrics.push_str(&format!(
            "tokens_send_in_the_last_minute{{model=\"{}\"}} {}\n",
            im.model_name, im.tpm_sent
        ))
    }

    for im in inference_metrics {
        prometheus_metrics.push_str(&format!(
            "tokens_received_in_the_last_minute{{model=\"{}\"}} {}\n",
            im.model_name, im.tpm_recv
        ))
    }

    Ok(prometheus_metrics)
}
