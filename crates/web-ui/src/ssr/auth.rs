use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{
        StatusCode,
        request::Parts,
    }
};
use serde::{Serialize, Deserialize};
use db::authz;

const X_FORWARDED_USER: &str = "X-Forwarded-User";
const X_FORWARDED_EMAIL: &str = "X-Forwarded-Email";

#[derive(Serialize, Deserialize, Debug)]
pub struct Authentication {
    pub sub: String,
    pub email: String,
}

impl From<Authentication> for authz::Authentication {
    fn from(val: Authentication) -> Self {
        authz::Authentication {
            sub: val.sub,
            email: val.email,
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
        let forwarded_user = parts.headers.get(X_FORWARDED_USER);
        let forwarded_email = parts.headers.get(X_FORWARDED_EMAIL);

        if let (Some(user), Some(email)) = (forwarded_user, forwarded_email) {
            let user = user.to_str();
            let email = email.to_str();

            if let (Ok(sub), Ok(email)) = (user, email) {
                let authentication = Authentication {
                    sub: sub.to_string(),
                    email: email.to_string(),
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