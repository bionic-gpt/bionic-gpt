use crate::{CustomError, Jwt};
use axum::extract::{Extension, Query};
use axum::response::Redirect;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, queries, Pool, Visibility};
// OAuth2 imports will be used when implementing actual OAuth2 flow
// use oauth2::{
//     AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
//     basic::BasicClient, reqwest::async_http_client,
//     AuthUrl, TokenUrl,
// };
use serde::Deserialize;
use web_pages::routes::integrations::{Connect, OAuth2Callback};

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

pub fn routes() -> Router {
    Router::new()
        .typed_get(connect_loader)
        .typed_get(oauth2_callback)
}

/// Initiate OAuth2 connection
pub async fn connect_loader(
    Connect {
        team_id: _team_id,
        integration_id: _integration_id,
    }: Connect,
    _current_user: Jwt,
    Extension(_pool): Extension<Pool>,
) -> Result<Redirect, CustomError> {
    // TODO: Get OAuth2 config from OpenAPI spec
    // TODO: Build authorization URL
    // TODO: Redirect to OAuth provider

    // Placeholder redirect for now
    Ok(Redirect::to("https://example.com/oauth/authorize"))
}

/// Handle OAuth2 callback
pub async fn oauth2_callback(
    OAuth2Callback {
        team_id,
        integration_id,
    }: OAuth2Callback,
    Query(_query): Query<CallbackQuery>,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Redirect, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let user_id = current_user.sub.parse::<i32>().unwrap();
    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // TODO: Exchange code for token
    // TODO: Store connection

    // Placeholder - store a dummy connection
    let refresh_token: Option<&str> = None;
    let expires_at: Option<time::OffsetDateTime> = None;

    let _connection_id = queries::connections::insert_connection()
        .bind(
            &transaction,
            &integration_id,
            &user_id,
            &team_id,
            &Visibility::Private,
            &"dummy_access_token".to_string(),
            &refresh_token,
            &expires_at,
            &serde_json::json!([]),
        )
        .one()
        .await?;

    transaction.commit().await?;

    Ok(Redirect::to(&format!(
        "/teams/{}/integrations/{}",
        team_id, integration_id
    )))
}
