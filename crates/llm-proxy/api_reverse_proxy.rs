use axum::{body::Body, http::Request, response::Response, Extension};
use http::header;

use super::LLMHandler;
use db::{queries, Pool};

use crate::errors::CustomError;

// Reverse proxy all LLM API calls directly to the model
// This handles the calls that are NOT /v1/chat/completions
pub async fn handler(
    LLMHandler { path: _ }: LLMHandler,
    Extension(pool): Extension<Pool>,
    req: Request<Body>,
) -> Result<Response, CustomError> {
    if let Some(api_key_header) = req.headers().get("Authorization") {
        let api_key_value = api_key_header
            .to_str()
            .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?
            .replace("Bearer ", "");
        let mut db_client = pool.get().await?;
        let transaction = db_client.transaction().await?;

        // Check this first, if we have a false API key then return auth error
        let api_key_record = queries::api_keys::find_api_key()
            .bind(&transaction, &api_key_value)
            .one()
            .await
            .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?;

        let prompt_id = api_key_record
            .prompt_id
            .ok_or_else(|| CustomError::Authentication("Invalid API Key".to_string()))?;

        let model_id = api_key_record
            .model_id
            .ok_or_else(|| CustomError::FaultySetup("API key missing model".to_string()))?;

        let model = queries::models::model()
            .bind(&transaction, &model_id)
            .one()
            .await?;

        let prompt = queries::prompts::prompt()
            .bind(&transaction, &prompt_id, &api_key_record.team_id)
            .one()
            .await?;

        let path = req.uri().path();
        let path_query = req
            .uri()
            .path_and_query()
            .map(|v| v.as_str())
            .unwrap_or(path);

        let mut headers = header::HeaderMap::new();
        if let Some(model_api_key) = model.api_key {
            let api_key = format!("Bearer {}", model_api_key);
            headers.insert(
                "Authorization",
                header::HeaderValue::from_str(&api_key)
                    .map_err(|e| CustomError::FaultySetup(e.to_string()))?,
            );
        }
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_str("application/json")
                .map_err(|e| CustomError::FaultySetup(e.to_string()))?,
        );

        let client = reqwest::Client::builder();
        let client = client.default_headers(headers);
        let client = client
            .build()
            .map_err(|e| CustomError::FaultySetup(e.to_string()))?;

        let base_url = prompt.base_url.replace("/v1", "");
        let uri = format!("{base_url}{path_query}");

        let reqwest_response = if req.method().to_string().to_lowercase() == "post" {
            client.post(uri).send().await?
        } else {
            client.get(uri).send().await?
        };

        let response_builder = Response::builder().status(reqwest_response.status().as_u16());
        Ok(response_builder
            .body(Body::from_stream(reqwest_response.bytes_stream()))
            .map_err(|e| CustomError::FaultySetup(e.to_string()))?)
    } else {
        Err(CustomError::Authentication(
            "You need an API key".to_string(),
        ))
    }
}
