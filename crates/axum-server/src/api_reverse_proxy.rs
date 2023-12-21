use axum::{
    body::Body,
    http::Request,
    response::{IntoResponse, Response},
    Extension, RequestExt,
};
use http::{HeaderName, Uri};
use hyper::client;

use db::{queries, Pool};
use hyper_rustls::ConfigBuilderExt;
use serde::{Deserialize, Serialize};

use crate::errors::CustomError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

pub async fn handler(
    Extension(pool): Extension<Pool>,
    mut req: Request<Body>,
) -> Result<Response, CustomError> {
    if let Some(api_key) = req.headers().get("Authorization") {
        let api_key = api_key.to_str().unwrap().replace("Bearer ", "");
        let mut db_client = pool.get().await.unwrap();
        let transaction = db_client.transaction().await.unwrap();

        let prompt = queries::prompts::prompt_by_api_key()
            .bind(&transaction, &api_key)
            .one()
            .await?;

        let api_key = queries::api_keys::find_api_key()
            .bind(&transaction, &api_key)
            .one()
            .await
            .map_err(|_| CustomError::Authentication("Invalid API Key".to_string()))?;

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

        // If we are completions we need to add the prompt to the request
        if path_query.ends_with("/completions") {
            transaction
                .query(
                    &format!("SET LOCAL row_level_security.user_id = {}", api_key.user_id),
                    &[],
                )
                .await?;

            let body: String = req
                .extract()
                .await
                .map_err(|_| CustomError::FaultySetup("Error extracting".to_string()))?;
            let completion: Completion = serde_json::from_str(&body)?;

            let messages = crate::prompt::execute_prompt(
                &transaction,
                prompt.id,
                prompt.team_id,
                None,
                completion.messages,
            )
            .await?;

            let completion = Completion {
                messages,
                ..completion
            };

            let completion_json = serde_json::to_string(&completion)?;

            // Create a new request
            req = Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(completion_json))?;
        } else {
            // Anything that is not completions gets passed direct to the LLM API
            *req.uri_mut() = Uri::try_from(uri).unwrap();
        }

        tracing::info!("{:?}", &req);

        // Do we have an API key, then add it.
        if let Some(api_key) = model.api_key {
            req.headers_mut().append(
                HeaderName::from_static("authorization"),
                format!("Bearer {}", api_key).parse().unwrap(),
            );
        }

        let tls = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_webpki_roots()
            .with_no_client_auth();

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls)
            .https_or_http()
            .enable_http1()
            .build();

        // Give the client the option to use TLS if required
        let client: client::Client<_, hyper::Body> = client::Client::builder().build(https);

        Ok(client.request(req).await?.into_response())
    } else {
        Err(CustomError::Authentication(
            "You neeed an API key".to_string(),
        ))
    }
}
