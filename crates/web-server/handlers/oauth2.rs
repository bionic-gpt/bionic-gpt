use crate::{CustomError, Jwt};
use axum::extract::{Extension, Query};
use axum::response::Redirect;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, queries, Pool, Visibility};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, RevocationUrl,
    Scope, TokenUrl,
};
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

pub async fn connect_loader(
    Connect {
        team_id,
        integration_id,
    }: Connect,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Redirect, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Get the integration from the database
    let _integration = queries::integrations::integration()
        .bind(&transaction, &integration_id)
        .one()
        .await?;

    let google_client_id =
        ClientId::new(std::env::var("GOOGLE_CLIENT_ID").unwrap_or("DUMMY_VALUE".into()));
    let google_client_secret =
        ClientSecret::new(std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or("DUMMY_VALUE".into()));

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    // Set up the config for the Google OAuth2 process.
    let client = BasicClient::new(google_client_id)
        .set_client_secret(google_client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
        )
        // Google supports OAuth 2.0 Token Revocation (RFC-7009)
        .set_revocation_url(
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
                .expect("Invalid revocation endpoint URL"),
        );

    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    Ok(Redirect::to(authorize_url.as_str()))
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
    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // TODO: Exchange code for token
    // TODO: Store connection

    // Placeholder - store a dummy connection
    let refresh_token: Option<&str> = None;
    let expires_at: Option<time::OffsetDateTime> = None;

    let _connection_id = queries::oauth2_connections::insert_oauth2_connection()
        .bind(
            &transaction,
            &integration_id,
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
