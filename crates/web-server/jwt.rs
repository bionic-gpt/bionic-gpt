use core::str;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use base64::{decode_config, URL_SAFE_NO_PAD};
use db::authz;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const X_FORWARDED_ACCESS_TOKEN: &str = "X-Forwarded-Access-Token";
const X_FORWARDED_USER: &str = "X-Forwarded-User";
const X_FORWARDED_EMAIL: &str = "X-Forwarded-Email";
const DANGER_JWT_OVERRIDE: &str = "DANGER_JWT_OVERRIDE";

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt {
    pub sub: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

// Convert it to something the db crate understands
impl From<Jwt> for authz::Authentication {
    fn from(val: Jwt) -> Self {
        authz::Authentication {
            sub: val.sub,
            email: val.email,
            given_name: val.given_name,
            family_name: val.family_name,
        }
    }
}

impl<S> FromRequestParts<S> for Jwt
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let access_token = if let Ok(override_token) = std::env::var(DANGER_JWT_OVERRIDE) {
            Some(override_token)
        } else {
            // Fall back to header, convert &str to String
            parts
                .headers
                .get(X_FORWARDED_ACCESS_TOKEN)
                .and_then(|header| header.to_str().ok())
                .map(|s| s.to_string())
        };
        let forwarded_user = parts.headers.get(X_FORWARDED_USER);
        let forwarded_email = parts.headers.get(X_FORWARDED_EMAIL);

        if let Some(access_token) = access_token {
            let jwt_parts: Vec<&str> = access_token.split('.').collect();

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

                                    let authentication = Jwt {
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
        } else if let (Some(user), Some(email)) = (forwarded_user, forwarded_email) {
            let user = user.to_str();
            let email = email.to_str();

            if let (Ok(sub), Ok(email)) = (user, email) {
                let authentication = Jwt {
                    sub: sub.to_string(),
                    email: email.to_string(),
                    given_name: None,
                    family_name: None,
                };

                return Ok(authentication);
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "Didn't find an authentication header",
        ))
    }
}
