use axum::{
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_extra::routing::TypedPath;
use db::Pool;
use serde::Deserialize;

use crate::{Authentication, CustomError};

#[derive(TypedPath, Deserialize)]
#[typed_path("/")]
pub struct BarricadeEndpoint {}

#[derive(TypedPath, Deserialize)]
#[typed_path("/app/post_registration")]
pub struct PostRegistrationEndpoint {}

pub async fn post_registration(
    PostRegistrationEndpoint {}: PostRegistrationEndpoint,
    authentication: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    tracing::debug!("{:?}", authentication);
    crate::oidc_endpoint::setup_user(&pool, authentication).await
}

pub async fn index(BarricadeEndpoint {}: BarricadeEndpoint) -> impl IntoResponse {
    Redirect::permanent("/auth/sign_in")
}
