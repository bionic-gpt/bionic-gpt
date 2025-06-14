use crate::{CustomError, Jwt};
use axum::extract::{Extension, Query};
use axum::response::Redirect;
use axum::Router;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use axum_extra::routing::RouterExt;
use db::{authz, queries, Pool, Visibility};
use integrations::{BionicOpenAPI, OAuth2Config};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
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
    jar: CookieJar,
    Extension(pool): Extension<Pool>,
) -> Result<(CookieJar, Redirect), CustomError> {
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

    // Load OAuth client credentials from the database
    let oauth_client = queries::oauth_clients::oauth_client_by_provider_url()
        .bind(&transaction, &oauth2_config.authorization_url)
        .one()
        .await?;

    let client_id = ClientId::new(oauth_client.client_id);
    let client_secret = ClientSecret::new(oauth_client.client_secret);

    let auth_url = AuthUrl::new(oauth2_config.authorization_url)
        .map_err(|_| CustomError::FaultySetup("Invalid authorization endpoint URL".to_string()))?;
    let token_url = TokenUrl::new(oauth2_config.token_url)
        .map_err(|_| CustomError::FaultySetup("Invalid token endpoint URL".to_string()))?;

    // Set up the config for the OAuth2 process using dynamic configuration
    let redirect_uri = format!(
        "http://localhost:7703/app/team/{}/integrations/{}/oauth2/callback",
        team_id, integration_id
    );
    let client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URL"));

    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let mut auth_request = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_code_challenge);

    // Add scopes from the OAuth2 configuration
    for scope in oauth2_config.scopes {
        auth_request = auth_request.add_scope(Scope::new(scope));
    }

    let (authorize_url, csrf_state) = auth_request.url();

    // Store verifier and state in cookies
    let mut jar = jar;
    let mut verifier_cookie =
        Cookie::new("oauth_pkce_verifier", pkce_code_verifier.secret().clone());
    verifier_cookie.set_path("/");
    jar = jar.add(verifier_cookie);
    let mut state_cookie = Cookie::new("oauth_csrf_state", csrf_state.secret().clone());
    state_cookie.set_path("/");
    jar = jar.add(state_cookie);

    Ok((jar, Redirect::to(authorize_url.as_str())))
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
    Query(query): Query<CallbackQuery>,
    current_user: Jwt,
    jar: CookieJar,
    Extension(pool): Extension<Pool>,
) -> Result<(CookieJar, Redirect), CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // Load OAuth client credentials
    let integration = queries::integrations::integration()
        .bind(&transaction, &integration_id)
        .one()
        .await?;
    let oauth2_config = get_oauth2_config_from_integration(&integration)?;
    let oauth_client = queries::oauth_clients::oauth_client_by_provider_url()
        .bind(&transaction, &oauth2_config.authorization_url)
        .one()
        .await?;

    let client = BasicClient::new(ClientId::new(oauth_client.client_id))
        .set_client_secret(ClientSecret::new(oauth_client.client_secret))
        .set_auth_uri(AuthUrl::new(oauth2_config.authorization_url).unwrap())
        .set_token_uri(TokenUrl::new(oauth2_config.token_url).unwrap())
        .set_redirect_uri(
            RedirectUrl::new(format!(
                "http://localhost:7703/app/team/{}/integrations/{}/oauth2/callback",
                team_id, integration_id
            ))
            .unwrap(),
        );

    // Validate CSRF state
    let state_cookie = jar.get("oauth_csrf_state");
    let verifier_cookie = jar.get("oauth_pkce_verifier");

    let state_cookie = match state_cookie {
        Some(c) => c.value().to_string(),
        None => return Err(CustomError::FaultySetup("Missing CSRF state".into())),
    };
    let verifier_cookie = match verifier_cookie {
        Some(c) => c.value().to_string(),
        None => return Err(CustomError::FaultySetup("Missing PKCE verifier".into())),
    };

    if state_cookie != query.state {
        return Err(CustomError::FaultySetup("Invalid CSRF state".into()));
    }

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Exchange code for token
    let token = client
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(PkceCodeVerifier::new(verifier_cookie))
        .request_async(&http_client)
        .await
        .map_err(|e| CustomError::FaultySetup(format!("Token exchange failed: {e}")))?;

    let refresh_token = token.refresh_token().map(|t| t.secret().to_string());
    let expires_at = token
        .expires_in()
        .map(|dur| time::OffsetDateTime::now_utc() + time::Duration::seconds(dur.as_secs() as i64));

    queries::connections::insert_oauth2_connection()
        .bind(
            &transaction,
            &integration_id,
            &team_id,
            &Visibility::Private,
            &token.access_token().secret().to_string(),
            &refresh_token.as_deref(),
            &expires_at,
            &serde_json::to_value(oauth2_config.scopes).unwrap_or_else(|_| serde_json::json!([])),
        )
        .one()
        .await?;

    transaction.commit().await?;

    let jar = jar.clone().remove(Cookie::build("oauth_csrf_state"));
    let jar = jar.clone().remove(Cookie::build("oauth_pkce_verifier"));

    Ok((
        jar,
        Redirect::to(&format!(
            "/teams/{}/integrations/{}",
            team_id, integration_id
        )),
    ))
}
