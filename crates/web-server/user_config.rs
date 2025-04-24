use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pub default_prompt: Option<i32>,
}

impl<S> FromRequestParts<S> for UserConfig
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jar = parts
            .extensions
            .get::<CookieJar>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Missing cookie jar"))?;

        if let Some(cookie) = jar.get("user_config") {
            if let Ok(config) = serde_json::from_str(cookie.value()) {
                return Ok(config);
            }
        }

        // fallback to default if missing or deserialization fails
        Ok(UserConfig {
            default_prompt: None,
        })
    }
}
