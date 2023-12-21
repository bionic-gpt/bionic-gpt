use axum::response::IntoResponse;
use http::header::{AUTHORIZATION, REFERER};
use http::{HeaderMap, Request};
use hyper::{client, Body};
use hyper_rustls::ConfigBuilderExt;

pub async fn index(headers: HeaderMap) -> String {
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
                .header(
                    REFERER,
                    "http://localhost:7710/realms/bionic-gpt/protocol/openid-connect/userinfo",
                )
                .body(Body::empty())
                .unwrap();

        dbg!(&req);

        let client: client::Client<_, hyper::Body> = client::Client::builder().build(https);
        let resp = client.request(req).await.unwrap().into_response();

        dbg!(&resp);
    }

    "Hello".to_string()
}
