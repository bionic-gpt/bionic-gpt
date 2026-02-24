use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use axum_extra::extract::cookie::Cookie;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pub default_prompt: Option<i32>,
    pub enabled_tools: Option<Vec<String>>, // Store tool names that are enabled
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
            enabled_tools: None,
        })
    }
}

/// Helper function to create a user_config cookie that's accessible from any path
pub fn create_user_config_cookie(
    config: &UserConfig,
) -> Result<Cookie<'static>, serde_json::Error> {
    let json = serde_json::to_string(config)?;
    let mut cookie = Cookie::new("user_config", json);
    cookie.set_path("/"); // Set the cookie path to root so it's accessible from any path
    cookie.set_http_only(true);
    cookie.set_same_site(axum_extra::extract::cookie::SameSite::Strict);
    Ok(cookie)
}
