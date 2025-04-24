use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use axum_extra::extract::cookie::Cookie;
use serde::{Deserialize, Serialize};

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
        let cookies_header = parts.headers.get("cookie");

        if let Some(header_value) = cookies_header {
            if let Ok(cookies_str) = header_value.to_str() {
                for cookie_str in cookies_str.split(';') {
                    if let Ok(cookie) = Cookie::parse_encoded(cookie_str.trim()) {
                        if cookie.name() == "user_config" {
                            if let Ok(config) = serde_json::from_str(cookie.value()) {
                                return Ok(config);
                            }
                        }
                    }
                }
            }
        }

        // Fallback to default
        Ok(UserConfig {
            default_prompt: None,
        })
    }
}
