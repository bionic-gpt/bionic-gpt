use axum::response::IntoResponse;
use http::header::AUTHORIZATION;
use http::{HeaderMap, Request};
use hyper::{client, Body};
use hyper_rustls::ConfigBuilderExt;

pub async fn index(headers: HeaderMap) -> impl IntoResponse {
    let tls = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_webpki_roots()
        .with_no_client_auth();

    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls)
        .https_or_http()
        .enable_http1()
        .build();

    if let Some(auth) = headers.get(AUTHORIZATION) {
        let auth = auth.to_str().unwrap();

        dbg!(&auth);

        let req =
            Request::get("http://keycloak:7710/realms/bionic-gpt/protocol/openid-connect/userinfo")
                .header(AUTHORIZATION, auth)
                .body(Body::empty())
                .unwrap();

        let client: client::Client<_, hyper::Body> = client::Client::builder().build(https);
        let resp = client.request(req).await.unwrap().into_response();

        return resp.into_body().into_response();
    }

    "Hello".to_string().into_response()
}
