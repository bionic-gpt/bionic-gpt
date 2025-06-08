use crate::{CustomError, Jwt};
use axum::extract::{Extension, Query};
use axum::response::Redirect;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, queries, Pool, Visibility};
use integrations::{BionicOpenAPI, OAuth2Config};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
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
    let integration = queries::integrations::integration()
        .bind(&transaction, &integration_id)
        .one()
        .await?;

    // Extract OAuth2 configuration from the integration's OpenAPI definition
    let oauth2_config = get_oauth2_config_from_integration(&integration)?;

    let client_id =
        ClientId::new(std::env::var("OAUTH2_CLIENT_ID").unwrap_or("DUMMY_VALUE".into()));
    let client_secret =
        ClientSecret::new(std::env::var("OAUTH2_CLIENT_SECRET").unwrap_or("DUMMY_VALUE".into()));

    let auth_url = AuthUrl::new(oauth2_config.authorization_url)
        .map_err(|_| CustomError::FaultySetup("Invalid authorization endpoint URL".to_string()))?;
    let token_url = TokenUrl::new(oauth2_config.token_url)
        .map_err(|_| CustomError::FaultySetup("Invalid token endpoint URL".to_string()))?;

    // Set up the config for the OAuth2 process using dynamic configuration
    let client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
        );

    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let mut auth_request = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_code_challenge);

    // Add scopes from the OAuth2 configuration
    for scope in oauth2_config.scopes {
        auth_request = auth_request.add_scope(Scope::new(scope));
    }

    let (authorize_url, _csrf_state) = auth_request.url();

    Ok(Redirect::to(authorize_url.as_str()))
}

/// Extract OAuth2 configuration from an integration's OpenAPI definition
fn get_oauth2_config_from_integration(
    integration: &db::queries::integrations::Integration,
) -> Result<OAuth2Config, CustomError> {
    let definition = integration.definition.as_ref().ok_or_else(|| {
        CustomError::FaultySetup("Integration has no OpenAPI definition".to_string())
    })?;

    let spec = oas3::from_json(definition.to_string())
        .map_err(|e| CustomError::FaultySetup(format!("Invalid OpenAPI spec: {}", e)))?;

    let bionic_api = BionicOpenAPI::from_spec(spec);

    bionic_api
        .get_oauth2_config()
        .ok_or_else(|| CustomError::FaultySetup("Integration does not support OAuth2".to_string()))
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

    let _connection_id = queries::connections::insert_oauth2_connection()
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
