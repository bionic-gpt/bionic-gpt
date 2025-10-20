mod ci;
mod cli;

use std::env;

use crate::args::{Args, Command};
use dagger_sdk::{HostDirectoryOpts, Query, Secret, connect, errors::ConnectError};
use eyre::{Result, eyre};
use tokio::time::{Duration, sleep};

pub(crate) const BASE_IMAGE: &str = "purtontech/rust-on-nails-devcontainer:1.3.18";
pub(crate) const POSTGRES_IMAGE: &str = "ankane/pgvector";
pub(crate) const DB_PASSWORD: &str = "testpassword";
pub(crate) const DATABASE_URL: &str =
    "postgresql://postgres:testpassword@postgres:5432/postgres?sslmode=disable";
pub(crate) const DB_FOLDER: &str = "crates/db";
pub(crate) const PIPELINE_FOLDER: &str = "crates/web-assets";
pub(crate) const APP_EXE_NAME: &str = "web-server";
pub(crate) const OPERATOR_EXE_NAME: &str = "k8s-operator";
pub(crate) const RAG_ENGINE_EXE_NAME: &str = "rag-engine";
pub(crate) const AIRBYTE_EXE_NAME: &str = "airbyte-connector";
pub(crate) const TARGET_TRIPLE: &str = "x86_64-unknown-linux-musl";

pub(crate) const APP_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt:latest";
pub(crate) const MIGRATIONS_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt-db-migrations:latest";
pub(crate) const RAG_ENGINE_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt-rag-engine:latest";
pub(crate) const OPERATOR_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt-k8s-operator:latest";
pub(crate) const AIRBYTE_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt-airbyte-connector:latest";

pub(crate) const SUMMARY_PATH: &str = "/build/SUMMARY.md";
const DAGGER_CONNECT_ATTEMPTS: usize = 3;
const DAGGER_RETRY_BASE_DELAY_SECS: u64 = 2;

pub async fn run(args: Args) -> Result<()> {
    let Args { command } = args;

    run_with_retries(command).await
}

async fn run_with_retries(command: Command) -> Result<()> {
    let mut last_connect_err = None;

    for attempt in 1..=DAGGER_CONNECT_ATTEMPTS {
        let attempt_command = command.clone();
        match connect(|client| async move { dispatch(client, attempt_command).await }).await {
            Ok(_) => return Ok(()),
            Err(ConnectError::FailedToConnect(err)) => {
                last_connect_err = Some(err);

                if attempt < DAGGER_CONNECT_ATTEMPTS {
                    let delay_secs = DAGGER_RETRY_BASE_DELAY_SECS * attempt as u64;
                    eprintln!(
                        "Dagger engine not ready yet (attempt {attempt}/{DAGGER_CONNECT_ATTEMPTS}). \
                         Retrying in {delay_secs}s..."
                    );
                    sleep(Duration::from_secs(delay_secs)).await;
                    continue;
                }
            }
            Err(ConnectError::DaggerContext(err)) => {
                return Err(err.wrap_err("dagger pipeline failed"));
            }
            Err(ConnectError::FailedToShutdown(err)) => {
                return Err(err.wrap_err("dagger engine shutdown failed"));
            }
        }
    }

    let err = last_connect_err.unwrap_or_else(|| eyre!("dagger engine did not report an error"));
    Err(err.wrap_err(format!(
        "failed to connect to dagger engine after {DAGGER_CONNECT_ATTEMPTS} attempts"
    )))
}

async fn dispatch(client: Query, command: Command) -> Result<()> {
    let repo_filters = HostDirectoryOpts {
        exclude: Some(vec![
            "target/",
            ".git/",
            "tmp/",
            "crates/web-assets/node_modules/",
        ]),
        gitignore: None,
        include: None,
        no_cache: None,
    };
    let repo = client.host().directory_opts(".", repo_filters);

    match command {
        Command::PullRequest => ci::run(&client, &repo, ci::PublishMode::PullRequest).await?,
        Command::All => ci::run(&client, &repo, ci::PublishMode::All).await?,
        Command::BuildCli { target } => cli::build_cli(&client, &repo, target).await?,
    }

    Ok(())
}

pub(crate) fn container_from(client: &Query, image: &str) -> dagger_sdk::Container {
    let container = client.container();
    let container = if let Some((username, secret)) = dockerhub_credentials(client) {
        container.with_registry_auth(image, username, secret)
    } else {
        container
    };
    container.from(image)
}

fn dockerhub_credentials(client: &Query) -> Option<(String, Secret)> {
    let username = env::var("DOCKERHUB_USERNAME").ok()?;
    let token = env::var("DOCKERHUB_TOKEN")
        .or_else(|_| env::var("DOCKERHUB_PASSWORD"))
        .ok()?;
    let secret = client.set_secret("dockerhub_token", token);
    Some((username, secret))
}
