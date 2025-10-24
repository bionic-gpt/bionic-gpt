use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    extract::TypedHeader,
    headers::{authorization::Bearer, Authorization},
};

use crate::error::ApiError;

#[derive(Debug, Clone)]
pub struct ConnectionString(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for ConnectionString
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| ApiError::unauthorized("missing bearer token"))?;

        let raw = bearer.token().trim();
        if raw.is_empty() {
            return Err(ApiError::unauthorized("empty bearer token"));
        }

        Ok(ConnectionString(raw.to_owned()))
    }
}
