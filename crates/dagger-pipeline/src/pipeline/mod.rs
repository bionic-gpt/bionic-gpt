mod ci;
use crate::args::{Args, Command};
use dagger_sdk::{HostDirectoryOpts, Query, connect};
use eyre::{Result, WrapErr};

pub(crate) const BASE_IMAGE: &str = "purtontech/rust-on-nails-devcontainer:1.3.46";
pub(crate) const POSTGRES_IMAGE: &str = "ankane/pgvector";
pub(crate) const DB_PASSWORD: &str = "testpassword";
pub(crate) const DATABASE_URL: &str =
    "postgresql://postgres:testpassword@postgres:5432/postgres?sslmode=disable";
pub(crate) const DB_FOLDER: &str = "crates/db";
pub(crate) const PIPELINE_FOLDER: &str = "crates/web-assets";
pub(crate) const APP_EXE_NAME: &str = "web-server";
pub(crate) const RAG_ENGINE_EXE_NAME: &str = "rag-engine";
pub(crate) const AIRBYTE_EXE_NAME: &str = "airbyte-connector";
pub(crate) const POSTGRES_MCP_EXE_NAME: &str = "postgres-mcp";
pub(crate) const CLI_GATEWAY_EXE_NAME: &str = "cli-gateway";
pub(crate) const TARGET_TRIPLE: &str = "x86_64-unknown-linux-musl";

pub(crate) const APP_IMAGE_REPO: &str = "ghcr.io/bionic-gpt/bionicgpt";
pub(crate) const MIGRATIONS_IMAGE_REPO: &str = "ghcr.io/bionic-gpt/bionicgpt-db-migrations";
pub(crate) const RAG_ENGINE_IMAGE_REPO: &str = "ghcr.io/bionic-gpt/bionicgpt-rag-engine";
pub(crate) const AIRBYTE_IMAGE_REPO: &str = "ghcr.io/bionic-gpt/bionicgpt-airbyte-connector";
pub(crate) const POSTGRES_MCP_IMAGE_REPO: &str = "ghcr.io/bionic-gpt/bionicgpt-postgres-mcp";
pub(crate) const CLI_GATEWAY_IMAGE_REPO: &str = "ghcr.io/bionic-gpt/bionicgpt-cli-gateway";
pub(crate) const EVAL_MOCKS_IMAGE_REPO: &str = "ghcr.io/bionic-gpt/bionicgpt-eval-mocks";

pub(crate) const SUMMARY_PATH: &str = "/build/SUMMARY.md";

pub async fn run(args: Args) -> Result<()> {
    let Args { command } = args;

    if matches!(command, Command::GenerateEvalMocksSpec) {
        ci::write_combined_eval_mocks_openapi()?;
        return Ok(());
    }

    connect(|client| async move { dispatch(client, command).await })
        .await
        .wrap_err("failed to run dagger pipeline")
}

async fn dispatch(client: Query, command: Command) -> Result<()> {
    let repo_filters = HostDirectoryOpts {
        exclude: Some(vec!["target/", ".git/", "tmp/"]),
        gitignore: None,
        include: None,
        no_cache: None,
    };
    let repo = client.host().directory_opts(".", repo_filters);

    match command {
        Command::PullRequest => ci::run(&client, &repo, ci::PublishMode::PullRequest).await?,
        Command::All => ci::run(&client, &repo, ci::PublishMode::All).await?,
        Command::GenerateEvalMocksSpec => unreachable!("handled before Dagger connection"),
    }

    Ok(())
}
