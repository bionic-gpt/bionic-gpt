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

use db::Pool;

type Client = hyper::client::Client<HttpConnector, Body>;

pub async fn handler(
    Extension(_pool): Extension<Pool>,
    State(client): State<Client>,
    mut req: Request<hyper::Body>,
) -> Result<Response, StatusCode> {
    if let Some(user_id) = req.headers().get("x-user-id") {
        if let Ok(user_id) = user_id.to_str() {
            if let Ok(user_id) = user_id.parse::<i32>() {
                dbg!(user_id);
            }
        }
    }

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    dbg!(&path_query);

    let uri = format!("http://llm-api:8080{path_query}");

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
