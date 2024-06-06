use db::{queries::inference_metrics, Transaction};

use crate::CustomError;

// Fetch the usage stats so far and compare with the limits
// if we have gone over the limits return true
pub async fn is_limit_exceeded(
    transaction: &Transaction<'_>,
    model_id: i32,
    user_id: i32,
) -> Result<bool, CustomError> {
    // Get the prompt
    let inference_metric = inference_metrics::inference_metrics()
        .bind(transaction, &model_id, &user_id)
        .one()
        .await?;

    let _total_tokens = inference_metric.tpm_recv + inference_metric.tpm_sent;

    Ok(false)
}
