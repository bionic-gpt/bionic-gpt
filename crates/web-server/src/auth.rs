use core::str;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use base64::{decode_config, URL_SAFE_NO_PAD};
use db::authz;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const X_FORWARDED_ACCESS_TOKEN: &str = "X-Forwarded-Access-Token";

#[derive(Serialize, Deserialize, Debug)]
pub struct Authentication {
    pub sub: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

impl From<Authentication> for authz::Authentication {
    fn from(val: Authentication) -> Self {
        authz::Authentication {
            sub: val.sub,
            email: val.email,
            given_name: val.given_name,
            family_name: val.family_name,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let access_token = parts.headers.get(X_FORWARDED_ACCESS_TOKEN);

        if let Some(access_token) = access_token {
            if let Ok(token) = access_token.to_str() {
                let jwt_parts: Vec<&str> = token.split('.').collect();

                if jwt_parts.len() == 3 {
                    if let Ok(payload) = decode_config(jwt_parts[1], URL_SAFE_NO_PAD) {
                        if let Ok(payload_str) = str::from_utf8(&payload) {
                            let json_value: Result<Value, _> = serde_json::from_str(payload_str);
                            if let Ok(json_value) = json_value {
                                if let Some(sub) = json_value.get("sub").and_then(|v| v.as_str()) {
                                    if let Some(email) =
                                        json_value.get("email").and_then(|v| v.as_str())
                                    {
                                        let given_name = json_value
                                            .get("given_name")
                                            .and_then(|v| v.as_str())
                                            .map(String::from);
                                        let family_name = json_value
                                            .get("family_name")
                                            .and_then(|v| v.as_str())
                                            .map(String::from);

                                        let authentication = Authentication {
                                            sub: sub.to_string(),
                                            email: email.to_string(),
                                            given_name,
                                            family_name,
                                        };

                                        return Ok(authentication);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "Didn't find an authentication header",
        ))
    }
}
