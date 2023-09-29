use axum::{
    body::Body,
    extract::{Path, State},
    http::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use http::Uri;
use hyper::client::HttpConnector;

use db::{queries, Pool};

use crate::authentication::Authentication;

type Client = hyper::client::Client<HttpConnector, Body>;

pub async fn handler(
    Path(chat_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    State(client): State<Client>,
    mut req: Request<hyper::Body>,
) -> Result<Response, StatusCode> {
    let mut db_client = pool.get().await.map_err(|_| StatusCode::BAD_REQUEST)?;
    let transaction = db_client
        .transaction()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    super::rls::set_row_level_security_user(&transaction, &current_user)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let base_url = queries::models::model_host_by_chat_id()
        .bind(&transaction, &chat_id)
        .one()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    dbg!(&path_query);

    let uri = format!("{base_url}/completions");

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
