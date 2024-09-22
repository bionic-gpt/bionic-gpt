use crate::auth::Authentication;
use crate::CustomError;
use axum::body::Body;
use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use axum::{Extension, RequestExt};
use db::queries::models;
use db::{ModelType, Pool};
use http::{HeaderMap, StatusCode};
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

            // Extract status code
            let status = StatusCode::from_u16(response.status().as_u16()).map_err(|e| {
                tracing::error!("Error generating status code: {:?}", e);
                CustomError::FaultySetup("Error generating status code".to_string())
            })?;

            // Extract headers from reqwest response
            let mut headers = HeaderMap::new();
            for (key, value) in response.headers() {
                headers.insert(key, value.clone());
            }

            // Extract body
            let body_bytes = response.bytes().await?;
            let body = body_bytes.to_vec(); // Convert body to Vec<u8> (Axum uses hyper)

            // Build axum response
            let response = (status, headers, body).into_response();

            Ok(response)
        }
        Err(err) => Err(CustomError::FaultySetup(err.to_string())),
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
            .post(format!("{}/chat/completions", model.base_url))
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
    } else {
        client
            .post(format!("{}/chat/completions", model.base_url))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
    };
    Ok(request)
}
