use axum::body::Body;
use axum::{
    response::{IntoResponse, Response},
    Extension,
};
use axum_extra::routing::TypedPath;
use db::{authz, queries, Pool};
use http::StatusCode;
use serde::Deserialize;

use crate::{CustomError, Jwt};

#[derive(TypedPath, Deserialize)]
#[typed_path("/")]
pub struct OidcEndpoint {}

pub async fn index(
    OidcEndpoint {}: OidcEndpoint,
    authentication: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    setup_user(&pool, authentication).await
}

pub async fn setup_user(
    pool: &Pool,
    authentication: Jwt,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let user = queries::users::user_by_openid_sub()
        .bind(&transaction, &authentication.sub)
        .one()
        .await;

    let authentication: authz::Authentication = authentication.into();

    // Do we have a user with this sub?
    let (user_id, _, _, _, _) = if let Ok(user) = user {
        (
            user.id,
            user.email,
            user.first_name,
            user.last_name,
            user.system_admin,
        )
    } else {
        authz::setup_user_if_not_already_registered(&transaction, &authentication).await?
    };

    let team = queries::teams::get_primary_team()
        .bind(&transaction, &user_id)
        .one()
        .await?;

    let console_url = web_pages::routes::console::Index { team_id: team.id }.to_string();

    let _rbac = authz::get_permissions(&transaction, &authentication, team.id).await?;

    transaction.commit().await?;

    let builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("location", console_url)
        .body(Body::empty());
    let response =
        builder.map_err(|_| CustomError::FaultySetup("Could not build redirect".to_string()))?;
    Ok(response)
}
