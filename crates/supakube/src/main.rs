mod application;
mod apply;
mod database;
mod deployment;
mod error;
mod install;
mod keycloak;
mod keycloak_db;
mod nginx;
mod oauth2_proxy;
mod operators;
mod selenium;

use anyhow::Result;
use clap::{Parser, Subcommand};

// Images we are using
const KEYCLOAK_IMAGE: &str = "quay.io/keycloak/keycloak:23.0";
const OAUTH2_PROXY_IMAGE: &str = "quay.io/oauth2-proxy/oauth2-proxy:v7.5.1";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub struct Installer {
    /// In which namespace shall we install the application
    #[arg(long, default_value = "rust-on-nails")]
    namespace: String,
    /// In which namespace shall we install the operators?
    #[arg(long, default_value = "nails-system")]
    operator_namespace: String,
    /// What is the application called
    #[arg(long, default_value = "nails")]
    app_name: String,
    /// The number of replicas
    #[arg(long, default_value_t = 1)]
    replicas: i32,
    /// The hostname we are deploying on. By default use the local ip address
    #[arg(long, default_value = "http://localhost:30000")]
    hostname_url: String,
    /// Don't create random db passwords but use this one. NOT FOR PRODUCTION
    #[arg(long)]
    insecure_override_passwords: Option<String>,
    /// Is this development? In which case we may do some things differently
    #[arg(long, default_value_t = false)]
    development: bool,
    /// Historically we prefixed the db users with the app ame
    #[arg(long)]
    db_user_prefix: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install the supakube services into Kubernetes
    Install(Installer),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install(installer) => {
            install::install(installer).await?;
        }
    }

    Ok(())
}
