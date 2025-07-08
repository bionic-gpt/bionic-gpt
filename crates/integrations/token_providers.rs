use crate::bionic_openapi::OAuth2Config;
use async_trait::async_trait;
use db::{self, Pool};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RefreshToken, TokenResponse, TokenUrl,
};
use reqwest::Client;
use time::{Duration, OffsetDateTime};

#[async_trait]
pub trait TokenProvider: Send + Sync {
    async fn token(&self) -> Option<String>;
    async fn force_refresh(&self);
}

pub struct StaticTokenProvider {
    token: String,
}

impl StaticTokenProvider {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

#[async_trait]
impl TokenProvider for StaticTokenProvider {
    async fn token(&self) -> Option<String> {
        Some(self.token.clone())
    }

    async fn force_refresh(&self) {
        tracing::debug!("StaticTokenProvider::force_refresh called - no action taken");
    }
}

pub struct OAuth2TokenProvider {
    pool: Pool,
    sub: String,
    connection_id: i32,
    client: Client,
    token: tokio::sync::Mutex<Option<String>>,
    refresh_token: tokio::sync::Mutex<Option<String>>,
    expires_at: tokio::sync::Mutex<Option<OffsetDateTime>>,
    config: OAuth2Config,
}

impl OAuth2TokenProvider {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pool: Pool,
        sub: String,
        connection_id: i32,
        token: Option<String>,
        refresh_token: Option<String>,
        expires_at: Option<OffsetDateTime>,
        config: OAuth2Config,
    ) -> Self {
        Self {
            pool,
            sub,
            connection_id,
            client: Client::new(),
            token: tokio::sync::Mutex::new(token),
            refresh_token: tokio::sync::Mutex::new(refresh_token),
            expires_at: tokio::sync::Mutex::new(expires_at),
            config,
        }
    }

    async fn refresh(&self) {
        tracing::info!(
            "Refreshing OAuth token for connection {}",
            self.connection_id
        );
        let mut token_guard = self.token.lock().await;
        let mut refresh_guard = self.refresh_token.lock().await;
        let mut expiry_guard = self.expires_at.lock().await;

        let Some(refresh_token) = refresh_guard.as_ref() else {
            return;
        };

        let mut client = match self.pool.get().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to get db client: {}", e);
                return;
            }
        };
        let transaction = match client.transaction().await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                return;
            }
        };

        if let Err(e) =
            db::authz::set_row_level_security_user_id(&transaction, self.sub.clone()).await
        {
            tracing::error!("Failed to set RLS: {}", e);
            return;
        }

        let oauth_client = match db::queries::oauth_clients::oauth_client_by_provider_url()
            .bind(&transaction, &self.config.authorization_url)
            .one()
            .await
        {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to load oauth client: {}", e);
                return;
            }
        };

        let client = BasicClient::new(ClientId::new(oauth_client.client_id))
            .set_client_secret(ClientSecret::new(oauth_client.client_secret))
            .set_auth_uri(AuthUrl::new(self.config.authorization_url.clone()).unwrap())
            .set_token_uri(TokenUrl::new(self.config.token_url.clone()).unwrap());

        let token = match client
            .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
            .request_async(&self.client)
            .await
        {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Failed to refresh token: {}", e);
                return;
            }
        };

        let new_token = token.access_token().secret().to_string();
        let new_refresh = token.refresh_token().map(|t| t.secret().to_string());
        let new_expiry = token
            .expires_in()
            .map(|dur| OffsetDateTime::now_utc() + Duration::seconds(dur.as_secs() as i64));

        if let Err(e) = db::queries::connections::update_oauth2_connection()
            .bind(
                &transaction,
                &new_token,
                &new_refresh.as_deref(),
                &new_expiry,
                &self.connection_id,
            )
            .await
        {
            tracing::error!("Failed to update connection: {}", e);
            return;
        }
        if let Err(e) = transaction.commit().await {
            tracing::error!("Failed to commit token update: {}", e);
            return;
        }

        tracing::info!(
            "OAuth token refreshed for connection {}",
            self.connection_id
        );

        *token_guard = Some(new_token);
        *refresh_guard = new_refresh;
        *expiry_guard = new_expiry;
    }

    async fn refresh_if_needed(&self) {
        let expiry_guard = self.expires_at.lock().await;
        if expiry_guard
            .as_ref()
            .is_some_and(|e| *e > OffsetDateTime::now_utc())
        {
            return;
        }
        drop(expiry_guard);
        self.refresh().await;
    }
}

#[async_trait]
impl TokenProvider for OAuth2TokenProvider {
    async fn token(&self) -> Option<String> {
        self.refresh_if_needed().await;
        self.token.lock().await.clone()
    }

    async fn force_refresh(&self) {
        self.refresh().await;
    }
}
