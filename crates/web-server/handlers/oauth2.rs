use crate::{CustomError, Jwt};
use axum::extract::{Extension, Query};
use axum::response::Redirect;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, queries, Pool, Visibility};
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl};
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

    // Get OAuth2 config from OpenAPI spec
    if let Some(definition) = &integration.definition {
        if let Ok(spec) = oas3::from_json(definition.to_string()) {
            if let Some(components) = &spec.components {
                for security_scheme in components.security_schemes.values() {
                    if let Ok(scheme_value) = serde_json::to_value(security_scheme) {
                        if let Some(scheme_type) = scheme_value.get("type").and_then(|t| t.as_str())
                        {
                            if scheme_type == "oauth2" {
                                // Extract OAuth2 configuration
                                if let Some(flows) = scheme_value.get("flows") {
                                    if let Some(auth_code_flow) = flows.get("authorizationCode") {
                                        let auth_url = auth_code_flow
                                            .get("authorizationUrl")
                                            .and_then(|u| u.as_str())
                                            .ok_or_else(|| {
                                                CustomError::FaultySetup(
                                                    "Missing authorization URL".to_string(),
                                                )
                                            })?;

                                        // Build authorization URL with placeholder client credentials
                                        // TODO: Get actual client credentials from configuration
                                        let _client_id =
                                            ClientId::new("placeholder-client-id".to_string());
                                        let _client_secret = ClientSecret::new(
                                            "placeholder-client-secret".to_string(),
                                        );
                                        let _redirect_url = RedirectUrl::new(format!(
                                            "http://localhost:3000/teams/{}/integrations/{}/oauth2/callback",
                                            team_id, integration_id
                                        ))
                                        .map_err(|e| CustomError::FaultySetup(e.to_string()))?;

                                        let _auth_url_parsed = AuthUrl::new(auth_url.to_string())
                                            .map_err(|e| {
                                            CustomError::FaultySetup(e.to_string())
                                        })?;

                                        // For now, just redirect to the authorization URL with basic parameters
                                        let mut auth_url_with_params = url::Url::parse(auth_url)
                                            .map_err(|e| CustomError::FaultySetup(e.to_string()))?;

                                        auth_url_with_params.query_pairs_mut()
                                            .append_pair("client_id", "placeholder-client-id")
                                            .append_pair("response_type", "code")
                                            .append_pair("redirect_uri", &format!(
                                                "http://localhost:3000/teams/{}/integrations/{}/oauth2/callback",
                                                team_id, integration_id
                                            ))
                                            .append_pair("scope", "read")
                                            .append_pair("state", "csrf-token-placeholder");

                                        transaction.commit().await?;

                                        // Redirect to OAuth provider
                                        return Ok(Redirect::to(auth_url_with_params.as_str()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err(CustomError::FaultySetup(
        "OAuth2 configuration not found".to_string(),
    ))
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
