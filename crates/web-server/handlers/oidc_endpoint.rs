use axum::body::Body;
use axum::{
    response::{IntoResponse, Response},
    Extension,
};
use axum_extra::routing::TypedPath;
use db::{authz, queries, Licence, ModelType, Pool};
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

    let _rbac = authz::get_permissions(&transaction, &authentication, team.id).await?;

    let llm_models = queries::models::models()
        .bind(&transaction, &ModelType::LLM)
        .all()
        .await?;
    let embeddings_models = queries::models::models()
        .bind(&transaction, &ModelType::Embeddings)
        .all()
        .await?;
    let setup_required = llm_models.is_empty() || embeddings_models.is_empty();

    let team_slug = team.slug.clone();
    let default_console_url = web_pages::routes::console::Index {
        team_id: team_slug.clone(),
    }
    .to_string();
    let mut redirect_url = Licence::global()
        .redirect_url
        .as_ref()
        .map(|template| template.replace("{team_id}", &team_slug))
        .filter(|url| !url.is_empty())
        .unwrap_or(default_console_url);

    if setup_required {
        redirect_url = web_pages::routes::models::Index { team_id: team_slug }.to_string();
    }

    transaction.commit().await?;

    let builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("location", redirect_url)
        .body(Body::empty());
    let response =
        builder.map_err(|_| CustomError::FaultySetup("Could not build redirect".to_string()))?;
    Ok(response)
}
