use axum::{
    body::Body,
    extract::State,
    http::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use http::Uri;
use hyper::client::HttpConnector;

use db::{queries, Pool};

type Client = hyper::client::Client<HttpConnector, Body>;

pub async fn handler(
    Extension(pool): Extension<Pool>,
    State(client): State<Client>,
    mut req: Request<hyper::Body>,
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
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        super::rls::set_row_level_security_user_id(&transaction, api_key.user_id)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let _generated_prompt = crate::prompt::execute_prompt(
            &transaction,
            prompt.id,
            prompt.organisation_id,
            "message.message",
        )
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

        let path = req.uri().path();
        let path_query = req
            .uri()
            .path_and_query()
            .map(|v| v.as_str())
            .unwrap_or(path);

        dbg!(&path_query);

        let base_url = prompt.base_url;
        let uri = format!("{base_url}{path_query}");

        *req.uri_mut() = Uri::try_from(uri).unwrap();

        Ok(client
            .request(req)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .into_response())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
