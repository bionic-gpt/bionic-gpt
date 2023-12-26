use axum::{
    async_trait,
    extract::FromRequestParts,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use db::authz;
use http::request::Parts;
use serde::{Deserialize, Serialize};

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
            given_name: "".to_string(),
            family_name: "".to_string(),
        }
    }
}

const X_FORWARDED_USER: &str = "X-Forwarded-User";
const X_FORWARDED_EMAIL: &str = "X-Forwarded-Email";

// From a request extract our authentication token.
#[async_trait]
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = Response;

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
                    given_name: None,
                    family_name: None,
                };

                return Ok(authentication);
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "Didn't find an authentication header",
        )
            .into_response())
    }
}
