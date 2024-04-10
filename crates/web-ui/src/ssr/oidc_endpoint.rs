use axum::body::Body;
use axum::{
    response::{IntoResponse, Response},
    Extension,
};
use db::{authz, queries, Pool};
use http::StatusCode;

use super::{Authentication, CustomError};

pub async fn index(
    authentication: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let user = queries::users::user_by_openid_sub()
        .bind(&transaction, &authentication.sub)
        .one()
        .await;

    let authentication: authz::Authentication = authentication.into();

    // Do we have a user with this sub?
    let (user_id, _, _, _) = if let Ok(user) = user {
        (user.id, user.email, user.first_name, user.last_name)
    } else {
        authz::setup_user_if_not_already_registered(&transaction, &authentication).await?
    };

    let team = queries::teams::get_primary_team()
        .bind(&transaction, &user_id)
        .one()
        .await?;

    let console_url = ui_pages::routes::console::index_route(team.id);

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
