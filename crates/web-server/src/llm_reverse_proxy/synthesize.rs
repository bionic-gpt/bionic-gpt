use crate::auth::Authentication;
use crate::CustomError;
use axum::body::Body;
use axum::extract::Request;
use axum::response::Response;
use axum::{Extension, RequestExt};
use db::queries::models;
use db::{ModelType, Pool};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    RequestBuilder,
};

use super::UISynthesize;

// Called from the front end to generate text to speech
pub async fn synthesize(
    UISynthesize {}: UISynthesize,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    req: Request<Body>,
) -> Result<Response<Body>, CustomError> {
    let body: String = req
        .extract()
        .await
        .map_err(|_| CustomError::FaultySetup("Error extracting".to_string()))?;

    match create_request(&pool, &current_user, body).await {
        Ok(request) => {
            // Non-streaming logic: generate the full response and return it
            let response = request.send().await.map_err(|e| {
                tracing::error!("Error calling model: {:?}", e);
                CustomError::FaultySetup("Error calling model".to_string())
            })?;

            let response_builder = Response::builder().status(response.status().as_u16());
            Ok(response_builder
                .body(Body::from_stream(response.bytes_stream()))
                // This unwrap is fine because the body is empty here
                .unwrap())
        }
        Err(err) => {
            dbg!(&err);
            Err(CustomError::FaultySetup(err.to_string()))
        }
    }
}

// Create the request that we'll send to reqwest to create an SSE stream of incoming
// chat completions.
async fn create_request(
    pool: &Pool,
    current_user: &Authentication,
    body: String,
) -> Result<RequestBuilder, CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;
    db::authz::set_row_level_security_user_id(&transaction, current_user.sub.to_string()).await?;
    let model = models::models()
        .bind(&transaction, &ModelType::TextToSpeech)
        .one()
        .await?;
    let client = reqwest::Client::new();
    let request = if let Some(api_key) = model.api_key {
        client
            .post(format!("{}/audio/speech", model.base_url))
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
    } else {
        client
            .post(format!("{}/audio/speech", model.base_url))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
    };
    Ok(request)
}
