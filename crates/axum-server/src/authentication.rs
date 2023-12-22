use axum::{
    async_trait,
    extract::FromRequestParts,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use http::header::AUTHORIZATION;
use http::request::Parts;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Authentication {
    pub sub: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
}

// From a request extract our authentication token.
#[async_trait]
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth_header) = parts.headers.get(AUTHORIZATION) {
            if let Ok(auth_header) = auth_header.to_str() {
                let client = Client::new();
                let response = client
                    .get("http://keycloak:7710/realms/bionic-gpt/protocol/openid-connect/userinfo")
                    .header(AUTHORIZATION, auth_header)
                    .send()
                    .await
                    .map_err(|_| (StatusCode::UNAUTHORIZED, "Issue calling OP").into_response())?;

                let result = response.json::<Authentication>().await.map_err(|_| {
                    (StatusCode::UNAUTHORIZED, "Problem parsing results").into_response()
                })?;

                return Ok(result);
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "Didn't find an authentication header",
        )
            .into_response())
    }
}
