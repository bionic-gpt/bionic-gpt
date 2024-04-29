use axum::{
    body::Body,
    http::Request,
    response::{IntoResponse, Response},
    Extension,
};
use http::{HeaderName, Uri};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};

use super::LLMHandler;
use db::{queries, Pool};

use crate::errors::CustomError;

// Reverse proxy all LLM API calls directly to the model
pub async fn handler(
    LLMHandler { path: _ }: LLMHandler,
    Extension(pool): Extension<Pool>,
    mut req: Request<Body>,
) -> Result<Response, CustomError> {
    if let Some(api_key) = req.headers().get("Authorization") {
        let api_key = api_key.to_str().unwrap().replace("Bearer ", "");
        let mut db_client = pool.get().await.unwrap();
        let transaction = db_client.transaction().await.unwrap();

        // Check this first, if we have a false API key then return auth error
        let api_key = queries::api_keys::find_api_key()
            .bind(&transaction, &api_key)
            .one()
            .await
            .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?;

        let prompt = queries::prompts::prompt_by_api_key()
            .bind(&transaction, &api_key.api_key)
            .one()
            .await?;

        let model = queries::models::model()
            .bind(&transaction, &prompt.model_id)
            .one()
            .await?;

        let path = req.uri().path();
        let path_query = req
            .uri()
            .path_and_query()
            .map(|v| v.as_str())
            .unwrap_or(path);

        let base_url = prompt.base_url.replace("/v1", "");
        let uri = format!("{base_url}{path_query}");
        *req.uri_mut() = Uri::try_from(uri).unwrap();

        tracing::info!("{:?}", &req);

        // Do we have an API key, then add it.
        if let Some(api_key) = model.api_key {
            req.headers_mut().append(
                HeaderName::from_static("authorization"),
                format!("Bearer {}", api_key).parse().unwrap(),
            );
        }

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .build();

        // Give the client the option to use TLS if required
        let client = Client::builder(TokioExecutor::new()).build(https);

        Ok(client.request(req).await?.into_response())
    } else {
        Err(CustomError::Authentication(
            "You need an API key".to_string(),
        ))
    }
}
