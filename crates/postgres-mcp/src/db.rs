use crate::error::{ApiError, ApiResult};
use once_cell::sync::OnceCell;
use rustls::{ClientConfig, RootCertStore};
use rustls_native_certs::load_native_certs;
use std::str::FromStr;
use tokio_postgres::{config::SslMode, Client, Config, NoTls};
use tokio_postgres_rustls::MakeRustlsConnect;
use tracing::debug;

static TLS_CONNECTOR: OnceCell<MakeRustlsConnect> = OnceCell::new();

pub async fn connect(connection_str: &str) -> ApiResult<Client> {
    let mut config = Config::from_str(connection_str)
        .map_err(|err| ApiError::unauthorized(format!("invalid connection string: {err}")))?;

    config.application_name("postgres-mcp");

    let ssl_mode = config.get_ssl_mode();
    match ssl_mode {
        SslMode::Disable => {
            let (client, connection) = config.connect(NoTls).await?;
            spawn_connection(connection);
            Ok(client)
        }
        _ => {
            let mut last_err = None;
            if let Some(tls) = maybe_tls_connector()? {
                match config.clone().connect(tls.clone()).await {
                    Ok((client, connection)) => {
                        spawn_connection(connection);
                        return Ok(client);
                    }
                    Err(err) => {
                        last_err = Some(err);
                    }
                }
            }

            if ssl_mode == SslMode::Prefer {
                let (client, connection) = config.connect(NoTls).await?;
                spawn_connection(connection);
                return Ok(client);
            }

            if let Some(err) = last_err {
                Err(err.into())
            } else {
                Err(ApiError::internal(
                    "TLS configuration unavailable on this host",
                ))
            }
        }
    }
}

fn maybe_tls_connector() -> ApiResult<Option<MakeRustlsConnect>> {
    TLS_CONNECTOR
        .get_or_try_init(build_tls_connector)
        .map(|connector| Some(connector.clone()))
        .or_else(|err| {
            debug!(error = %err, "failed to initialize TLS connector");
            Ok(None)
        })
}

fn build_tls_connector() -> ApiResult<MakeRustlsConnect> {
    let cert_result = load_native_certs();
    if !cert_result.errors.is_empty() {
        debug!(
            errors = cert_result.errors.len(),
            "some native certificates failed to load"
        );
    }

    let mut roots = RootCertStore::empty();
    let (added, ignored) = roots.add_parsable_certificates(cert_result.certs);
    debug!(added, ignored, "loaded native certificate roots");

    if added == 0 {
        return Err(ApiError::internal(
            "no usable root certificates found in system store",
        ));
    }

    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    Ok(MakeRustlsConnect::new(config))
}

fn spawn_connection<T>(connection: T)
where
    T: std::future::Future<Output = Result<(), tokio_postgres::Error>> + Send + 'static,
{
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            debug!(error = %err, "postgres connection closed");
        }
    });
}
