use axum::{
    async_trait,
    extract::FromRequestParts,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use http::request::Parts;

#[derive(Debug)]
pub struct Authentication {
    pub user_id: i32,
}

// From a request extract our authentication token.
#[async_trait]
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_id) = parts.headers.get("x-user-id") {
            if let Ok(user_id) = user_id.to_str() {
                if let Ok(user_id) = user_id.parse::<i32>() {
                    return Ok(Authentication { user_id });
                }
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "x-user-id not found or unparseable as i32",
        )
            .into_response())
    }
}
