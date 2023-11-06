use axum::{
    body::Body,
    extract::Path,
    http::Request,
    response::{IntoResponse, Response},
    Extension,
};
use http::HeaderName;
use hyper::client;

use db::{queries, Pool};
use hyper_rustls::ConfigBuilderExt;

use crate::{api_reverse_proxy::Completion, authentication::Authentication, errors::CustomError};

pub async fn handler(
    Path(chat_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Response, CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;

    super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let model = queries::models::model_host_by_chat_id()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    let chat = queries::chats::chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    let conversation = queries::conversations::get_conversation_from_chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &chat.prompt_id, &conversation.organisation_id)
        .one()
        .await?;

    let max_tokens = if model.base_url.starts_with("https://inference.gig") {
        Some(4000)
    } else {
        None
    };

    let messages = crate::prompt::execute_prompt(
        &transaction,
        prompt.id,
        conversation.organisation_id,
        &chat.user_request,
    )
    .await?;

    let json_messages = serde_json::to_string(&messages)?;

    queries::chats::update_prompt()
        .bind(&transaction, &json_messages, &chat_id)
        .await?;

    transaction.commit().await?;

    let completion = Completion {
        model: model.name,
        stream: Some(true),
        max_tokens,
        temperature: Some(0.7),
        messages,
    };

    let completion_json = serde_json::to_string(&completion)?;

    // Create a new request
    let mut req = Request::post(format!("{}/chat/completions", model.base_url))
        .header("content-type", "application/json")
        .body(Body::from(completion_json))?;

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

    // Build the hyper client from the HTTPS connector.
    let client: client::Client<_, hyper::Body> = client::Client::builder().build(https);

    Ok(client.request(req).await?.into_response())
}
