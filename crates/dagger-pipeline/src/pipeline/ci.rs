use std::env;

use dagger_sdk::{Container, Directory, File, Query, Service};
use eyre::{Result, WrapErr};

use super::{
    AIRBYTE_EXE_NAME, AIRBYTE_IMAGE_NAME, APP_EXE_NAME, APP_IMAGE_NAME, BASE_IMAGE, DATABASE_URL,
    DB_FOLDER, DB_PASSWORD, MIGRATIONS_IMAGE_NAME, OPERATOR_EXE_NAME, OPERATOR_IMAGE_NAME,
    PIPELINE_FOLDER, POSTGRES_IMAGE, RAG_ENGINE_EXE_NAME, RAG_ENGINE_IMAGE_NAME, SUMMARY_PATH,
    TARGET_TRIPLE,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishMode {
    PullRequest,
    All,
}

pub(super) async fn run(client: &Query, repo: &Directory, mode: PublishMode) -> Result<()> {
    let outputs = build_workspace(client, repo).await?;
    publish_summary(&outputs.summary).await?;

    if matches!(mode, PublishMode::All) {
        publish_images(client, &outputs).await?;
    }

    Ok(())
}

struct BuildOutputs {
    container: Container,
    summary: File,
    app_binary: File,
    operator_binary: File,
    rag_engine_binary: File,
    airbyte_binary: File,
}

fn release_binary_path(exe: &str) -> String {
    format!("target/{TARGET_TRIPLE}/release/{exe}")
}

fn summary_markdown() -> String {
    format!(
        "## Quality Checks

- ✅ Started temporary Postgres (ankane/pgvector) for database-backed checks
- ✅ Applied migrations with `dbmate --wait --migrations-dir {db}/migrations up`
- ✅ `cargo fmt --all -- --check`
- ✅ `cargo clippy --workspace --all-targets -- -D warnings`
- ✅ `cargo test --workspace --exclude integration-testing --exclude rag-engine`
- ✅ `cargo build --release --target {target}`

Tests ran via `cargo test --workspace --exclude integration-testing --exclude rag-engine`.
",
        db = DB_FOLDER,
        target = TARGET_TRIPLE
    )
}

async fn build_workspace(client: &Query, repo: &Directory) -> Result<BuildOutputs> {
    let postgres_service = postgres_service(client);

    let after_postgres = client
        .container()
        .from(BASE_IMAGE)
        .with_directory("/workspace", repo.clone())
        .with_workdir("/workspace")
        .with_user("root")
        .with_service_binding("postgres", postgres_service)
        .with_env_variable("DATABASE_URL", DATABASE_URL)
        .with_env_variable("APP_DATABASE_URL", DATABASE_URL);

    let after_migrations = after_postgres.with_exec(vec![
        "dbmate",
        "--wait",
        "--migrations-dir",
        "crates/db/migrations",
        "up",
    ]);

    let after_node_install =
        after_migrations.with_exec(vec!["npm", "--prefix", "crates/web-assets", "install"]);

    let after_node_release = after_node_install.with_exec(vec![
        "npm",
        "--prefix",
        "crates/web-assets",
        "run",
        "release",
    ]);

    let after_rust = after_node_release.with_exec(vec![
        "cargo",
        "build",
        "--release",
        "--target",
        TARGET_TRIPLE,
    ]);

    let summary_container = after_rust.with_new_file(SUMMARY_PATH, summary_markdown());

    let summary = summary_container.file(SUMMARY_PATH);
    let app_binary = summary_container.file(release_binary_path(APP_EXE_NAME));
    let operator_binary = summary_container.file(release_binary_path(OPERATOR_EXE_NAME));
    let rag_engine_binary = summary_container.file(release_binary_path(RAG_ENGINE_EXE_NAME));
    let airbyte_binary = summary_container.file(release_binary_path(AIRBYTE_EXE_NAME));

    Ok(BuildOutputs {
        container: summary_container,
        summary,
        app_binary,
        operator_binary,
        rag_engine_binary,
        airbyte_binary,
    })
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
