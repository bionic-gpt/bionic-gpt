use db::{
    queries::{inference_metrics, models},
    Pool, Transaction,
};

use crate::errors::CustomError;

// Fetch the usage stats so far and compare with the limits
// if we have gone over the limits return true
pub async fn is_limit_exceeded(
    transaction: &Transaction<'_>,
    model_id: i32,
    user_id: i32,
) -> Result<bool, CustomError> {
    // Get the inference metrics
    let inference_metric = inference_metrics::inference_metric()
        .bind(transaction, &model_id, &user_id)
        .opt()
        .await?;

    let (tpm_recv, tpm_sent) = inference_metric
        .map(|m| (m.tpm_recv, m.tpm_sent))
        .unwrap_or((0, 0));
    let total_tokens = tpm_recv + tpm_sent;

    let model = models::model().bind(transaction, &model_id).one().await?;

    if (model.tpm_limit as i64) > total_tokens {
        Ok(false)
    } else {
        tracing::warn!("Restricting user {} for model {}", user_id, model_id);
        Ok(true)
    }
}

pub async fn is_limit_exceeded_from_pool(
    pool: &Pool,
    model_id: i32,
    user_id: i32,
) -> Result<bool, CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;

    is_limit_exceeded(&transaction, model_id, user_id).await
}
