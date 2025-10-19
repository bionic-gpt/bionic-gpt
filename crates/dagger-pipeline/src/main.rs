use std::env;

use clap::{Parser, Subcommand, ValueEnum};
use dagger_sdk::{Container, Directory, File, HostDirectoryOpts, Query, Service, connect};
use eyre::{Result, WrapErr, eyre};

const BASE_IMAGE: &str = "purtontech/rust-on-nails-devcontainer:1.3.18";
const POSTGRES_IMAGE: &str = "ankane/pgvector";
const DB_PASSWORD: &str = "testpassword";
const DATABASE_URL: &str =
    "postgresql://postgres:testpassword@postgres:5432/postgres?sslmode=disable";
const DB_FOLDER: &str = "crates/db";
const PIPELINE_FOLDER: &str = "crates/web-assets";
const APP_EXE_NAME: &str = "web-server";
const OPERATOR_EXE_NAME: &str = "k8s-operator";
const RAG_ENGINE_EXE_NAME: &str = "rag-engine";
const AIRBYTE_EXE_NAME: &str = "airbyte-connector";
const TARGET_TRIPLE: &str = "x86_64-unknown-linux-musl";

const APP_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt:latest";
const MIGRATIONS_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt-db-migrations:latest";
const RAG_ENGINE_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt-rag-engine:latest";
const OPERATOR_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt-k8s-operator:latest";
const AIRBYTE_IMAGE_NAME: &str = "ghcr.io/bionic-gpt/bionicgpt-airbyte-connector:latest";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which pipeline target to execute.
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
enum Command {
    /// Build and test for pull request validation (no publish).
    PullRequest,
    /// Build for main branch and publish all artifacts.
    All,
    /// Build the CLI for the given operating system.
    BuildCli {
        #[arg(value_enum)]
        target: CliTarget,
    },
}

#[derive(ValueEnum, Clone, Copy)]
enum CliTarget {
    Linux,
    Macos,
    Windows,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PublishMode {
    PullRequest,
    All,
}

struct BuildOutputs {
    container: Container,
    summary: File,
    app_binary: File,
    operator_binary: File,
    rag_engine_binary: File,
    airbyte_binary: File,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let command = args.command;

    connect(|client| async move { run_command(client, command).await })
        .await
        .map_err(|err| eyre!("failed to run dagger pipeline: {err}"))?;

    Ok(())
}

async fn run_command(client: Query, command: Command) -> Result<()> {
    let repo_filters = HostDirectoryOpts {
        exclude: Some(vec!["target/", ".git/", "tmp/"]),
        gitignore: None,
        include: None,
        no_cache: None,
    };
    let repo = client.host().directory_opts(".", repo_filters);

    match command {
        Command::PullRequest => run_ci_pipeline(&client, &repo, PublishMode::PullRequest).await?,
        Command::All => run_ci_pipeline(&client, &repo, PublishMode::All).await?,
        Command::BuildCli { target } => build_cli(&client, &repo, target).await?,
    }

    Ok(())
}

async fn run_ci_pipeline(client: &Query, repo: &Directory, mode: PublishMode) -> Result<()> {
    let outputs = build_workspace(client, repo).await?;
    publish_summary(&outputs.summary).await?;
    export_ci_artifacts(&outputs).await?;

    if matches!(mode, PublishMode::All) {
        publish_images(client, &outputs).await?;
    }

    Ok(())
}

async fn build_workspace(client: &Query, repo: &Directory) -> Result<BuildOutputs> {
    let base = base_builder_container(client, repo);

    let npm_prepared = base.with_exec(vec![
        "bash".to_string(),
        "-lc".to_string(),
        format!(
            "cd {pipeline} && npm ci && npm run release",
            pipeline = PIPELINE_FOLDER
        ),
    ]);

    let postgres_service = postgres_service(client);

    let build_commands = format!(
        r#"set -euo pipefail
dbmate --wait --migrations-dir {db}/migrations up
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --exclude integration-testing --exclude rag-engine
cargo build --release --target {target}
cat <<'EOF' > SUMMARY.md
## Quality Checks

- ✅ Started temporary Postgres (ankane/pgvector) for database-backed checks
- ✅ Applied migrations with `dbmate --wait --migrations-dir {db}/migrations up`
- ✅ `cargo fmt --all -- --check`
- ✅ `cargo clippy --workspace --all-targets -- -D warnings`
- ✅ `cargo test --workspace --exclude integration-testing --exclude rag-engine`
- ✅ `cargo build --release --target {target}`

Tests ran via `cargo test --workspace --exclude integration-testing --exclude rag-engine`.
EOF
"#,
        db = DB_FOLDER,
        target = TARGET_TRIPLE
    );

    let built = npm_prepared
        .with_service_binding("postgres", postgres_service)
        .with_env_variable("DATABASE_URL", DATABASE_URL)
        .with_env_variable("APP_DATABASE_URL", DATABASE_URL)
        .with_exec(vec!["bash", "-lc", &build_commands]);

    let summary = built.file("SUMMARY.md");

    let app_binary = built.file(format!(
        "target/{target}/release/{exe}",
        target = TARGET_TRIPLE,
        exe = APP_EXE_NAME
    ));
    let operator_binary = built.file(format!(
        "target/{target}/release/{exe}",
        target = TARGET_TRIPLE,
        exe = OPERATOR_EXE_NAME
    ));
    let rag_engine_binary = built.file(format!(
        "target/{target}/release/{exe}",
        target = TARGET_TRIPLE,
        exe = RAG_ENGINE_EXE_NAME
    ));
    let airbyte_binary = built.file(format!(
        "target/{target}/release/{exe}",
        target = TARGET_TRIPLE,
        exe = AIRBYTE_EXE_NAME
    ));

    Ok(BuildOutputs {
        container: built,
        summary,
        app_binary,
        operator_binary,
        rag_engine_binary,
        airbyte_binary,
    })
}

fn base_builder_container(client: &Query, repo: &Directory) -> Container {
    let cache_cargo = client.cache_volume("cargo-target");
    let cache_registry = client.cache_volume("cargo-registry");
    let cache_npm = client.cache_volume("npm-cache");

    client
        .container()
        .from(BASE_IMAGE)
        .with_user("vscode")
        .with_workdir("/build")
        .with_directory(".", repo.clone())
        .with_mounted_cache("/build/target", cache_cargo)
        .with_mounted_cache("/usr/local/cargo/registry", cache_registry)
        .with_mounted_cache(
            format!("/build/{}/node_modules", PIPELINE_FOLDER),
            cache_npm,
        )
}

fn postgres_service(client: &Query) -> Service {
    client
        .container()
        .from(POSTGRES_IMAGE)
        .with_env_variable("POSTGRES_PASSWORD", DB_PASSWORD)
        .with_exposed_port(5432)
        .as_service()
}

async fn publish_summary(summary: &File) -> Result<()> {
    summary
        .export("SUMMARY.md")
        .await
        .wrap_err("unable to export SUMMARY.md to host")?;
    Ok(())
}

async fn export_ci_artifacts(outputs: &BuildOutputs) -> Result<()> {
    outputs
        .app_binary
        .export(format!("target/{}/release/{}", TARGET_TRIPLE, APP_EXE_NAME))
        .await
        .wrap_err("failed to export app binary")?;
    outputs
        .rag_engine_binary
        .export(format!(
            "target/{}/release/{}",
            TARGET_TRIPLE, RAG_ENGINE_EXE_NAME
        ))
        .await
        .wrap_err("failed to export rag engine binary")?;
    outputs
        .airbyte_binary
        .export(format!(
            "target/{}/release/{}",
            TARGET_TRIPLE, AIRBYTE_EXE_NAME
        ))
        .await
        .wrap_err("failed to export airbyte binary")?;
    outputs
        .operator_binary
        .export(format!(
            "target/{}/release/{}",
            TARGET_TRIPLE, OPERATOR_EXE_NAME
        ))
        .await
        .wrap_err("failed to export operator binary")?;
    outputs
        .container
        .directory(format!("{}/dist", PIPELINE_FOLDER))
        .export(format!("{}/dist", PIPELINE_FOLDER))
        .await
        .wrap_err("failed to export web assets dist directory")?;

    Ok(())
}

async fn publish_images(client: &Query, outputs: &BuildOutputs) -> Result<()> {
    let username = env::var("GHCR_USERNAME")
        .or_else(|_| env::var("GITHUB_ACTOR"))
        .wrap_err("set GHCR_USERNAME or rely on default GITHUB_ACTOR in the workflow")?;
    let token = env::var("GHCR_TOKEN")
        .or_else(|_| env::var("GITHUB_TOKEN"))
        .wrap_err("set GHCR_TOKEN or ensure GITHUB_TOKEN is available")?;
    let ghcr_secret = client.set_secret("ghcr_token", token);
    let registry = "ghcr.io";

    let dist_dir = outputs
        .container
        .directory(format!("{}/dist", PIPELINE_FOLDER));
    dist_dir
        .id()
        .await
        .wrap_err("failed to load web assets dist directory")?;
    let images_dir = outputs
        .container
        .directory(format!("{}/images", PIPELINE_FOLDER));
    images_dir
        .id()
        .await
        .wrap_err("failed to load images directory")?;

    client
        .container()
        .from("scratch")
        .with_user("1001")
        .with_file("/axum-server", outputs.app_binary.clone())
        .with_directory(format!("/build/{}", PIPELINE_FOLDER), dist_dir)
        .with_directory(format!("/build/{}/images", PIPELINE_FOLDER), images_dir)
        .with_entrypoint(vec!["./axum-server"])
        .with_registry_auth(registry, &username, ghcr_secret.clone())
        .publish(APP_IMAGE_NAME)
        .await
        .wrap_err("failed to publish app image")?;

    client
        .container()
        .from("scratch")
        .with_user("1001")
        .with_file("/rag-engine", outputs.rag_engine_binary.clone())
        .with_entrypoint(vec!["./rag-engine"])
        .with_registry_auth(registry, &username, ghcr_secret.clone())
        .publish(RAG_ENGINE_IMAGE_NAME)
        .await
        .wrap_err("failed to publish rag engine image")?;

    client
        .container()
        .from("scratch")
        .with_user("1001")
        .with_file("/airbyte-connector", outputs.airbyte_binary.clone())
        .with_entrypoint(vec!["./airbyte-connector"])
        .with_registry_auth(registry, &username, ghcr_secret.clone())
        .publish(AIRBYTE_IMAGE_NAME)
        .await
        .wrap_err("failed to publish airbyte image")?;

    client
        .container()
        .from("scratch")
        .with_file("/k8s-operator", outputs.operator_binary.clone())
        .with_entrypoint(vec!["./k8s-operator", "operator"])
        .with_registry_auth(registry, &username, ghcr_secret.clone())
        .publish(OPERATOR_IMAGE_NAME)
        .await
        .wrap_err("failed to publish operator image")?;

    let db_dir = outputs.container.directory(DB_FOLDER);
    db_dir
        .id()
        .await
        .wrap_err("failed to prepare db directory")?;

    client
        .container()
        .from("alpine")
        .with_exec(vec!["apk", "add", "--no-cache", "curl", "postgresql-client", "tzdata"])
        .with_exec(vec![
            "sh",
            "-lc",
            "curl -OL https://github.com/amacneil/dbmate/releases/download/v2.2.0/dbmate-linux-amd64",
        ])
        .with_exec(vec![
            "sh",
            "-lc",
            "mv ./dbmate-linux-amd64 /usr/bin/dbmate && chmod +x /usr/bin/dbmate",
        ])
        .with_directory("/db", db_dir)
        .with_workdir("/db")
        .with_entrypoint(vec!["dbmate", "up"])
        .with_registry_auth(registry, &username, ghcr_secret)
        .publish(MIGRATIONS_IMAGE_NAME)
        .await
        .wrap_err("failed to publish migration image")?;

    Ok(())
}

async fn build_cli(client: &Query, repo: &Directory, target: CliTarget) -> Result<()> {
    match target {
        CliTarget::Linux => build_cli_linux(client, repo).await,
        CliTarget::Macos => build_cli_macos(client, repo).await,
        CliTarget::Windows => build_cli_windows(client, repo).await,
    }
}

async fn build_cli_linux(client: &Query, repo: &Directory) -> Result<()> {
    let container = client
        .container()
        .from(BASE_IMAGE)
        .with_user("vscode")
        .with_workdir("/build")
        .with_directory(".", repo.clone())
        .with_exec(vec![
            "bash",
            "-lc",
            "cd crates/k8s-operator && cargo build --release",
        ]);

    container
        .file("crates/k8s-operator/target/release/k8s-operator")
        .export("bionic-cli-linux")
        .await
        .wrap_err("failed to export linux cli")?;

    Ok(())
}

async fn build_cli_macos(client: &Query, repo: &Directory) -> Result<()> {
    let container = client
        .container()
        .from("joseluisq/rust-linux-darwin-builder:1.85")
        .with_workdir("/build")
        .with_directory(".", repo.clone())
        .with_exec(vec![
            "bash",
            "-lc",
            "cd crates/k8s-operator && CC=o64-clang CXX=o64-clang++ cargo build --release --target x86_64-apple-darwin",
        ]);

    container
        .file("crates/k8s-operator/target/x86_64-apple-darwin/release/k8s-operator")
        .export("bionic-cli-darwin")
        .await
        .wrap_err("failed to export macos cli")?;

    Ok(())
}

async fn build_cli_windows(client: &Query, repo: &Directory) -> Result<()> {
    let container = client
        .container()
        .from(BASE_IMAGE)
        .with_workdir("/build")
        .with_directory(".", repo.clone())
        .with_exec(vec![
            "bash",
            "-lc",
            r#"
set -euo pipefail
sudo apt update && sudo apt upgrade -y
sudo apt install -y g++-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
cd crates/k8s-operator
cargo build --release --target x86_64-pc-windows-gnu
"#,
        ]);

    container
        .file("crates/k8s-operator/target/x86_64-pc-windows-gnu/release/k8s-operator.exe")
        .export("bionic-cli-windows.exe")
        .await
        .wrap_err("failed to export windows cli")?;

    Ok(())
}
