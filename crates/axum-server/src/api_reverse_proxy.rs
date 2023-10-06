use axum::{
    body::Body,
    extract::State,
    http::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, RequestExt,
};
use http::Uri;
use hyper::client::HttpConnector;

use db::{queries, Pool};
use serde::{Deserialize, Serialize};

type Client = hyper::client::Client<HttpConnector, Body>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    pub model: String,
    pub stream: Option<bool>,
    pub max_tokens: Option<i32>,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
}

pub async fn handler(
    Extension(pool): Extension<Pool>,
    State(client): State<Client>,
    mut req: Request<Body>,
) -> Result<Response, StatusCode> {
    if let Some(api_key) = req.headers().get("Authorization") {
        let mut db_client = pool.get().await.unwrap();
        let transaction = db_client.transaction().await.unwrap();

        let prompt = queries::prompts::prompt_by_api_key()
            .bind(&transaction, &api_key.to_str().unwrap())
            .one()
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let api_key = queries::api_keys::find_api_key()
            .bind(&transaction, &api_key.to_str().unwrap())
            .one()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let path = req.uri().path();
        let path_query = req
            .uri()
            .path_and_query()
            .map(|v| v.as_str())
            .unwrap_or(path);

        let base_url = prompt.base_url;
        let uri = format!("{base_url}{path_query}");

        // If we are completions we need to add the prompt to the request
        if path_query.ends_with("/completions") {
            super::rls::set_row_level_security_user_id(&transaction, api_key.user_id)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            let body: String = req.extract().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            let completion: Completion =
                serde_json::from_str(&body).map_err(|_| StatusCode::BAD_REQUEST)?;

            let messages = crate::prompt::execute_prompt(
                &transaction,
                prompt.id,
                prompt.organisation_id,
                "message.message",
            )
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

            let completion = Completion {
                messages,
                ..completion
            };

            let completion_json =
                serde_json::to_string(&completion).map_err(|_| StatusCode::BAD_REQUEST)?;

            // Create a new request
            let req = Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(completion_json))
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            Ok(client
                .request(req)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into_response())
        } else {
            // Anything that is not completions gets passed direct to the LLM API

            *req.uri_mut() = Uri::try_from(uri).unwrap();

            Ok(client
                .request(req)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into_response())
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
