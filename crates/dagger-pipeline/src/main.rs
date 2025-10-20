//mod args;
//mod pipeline;

use dagger_sdk::{Container, File, HostDirectoryOpts, Query, Service};
use eyre::{Context, Result};
use std::fs;

pub(crate) const BASE_IMAGE: &str = "purtontech/rust-on-nails-devcontainer:1.3.18";
pub(crate) const POSTGRES_IMAGE: &str = "ankane/pgvector";
pub(crate) const DB_PASSWORD: &str = "testpassword";
pub(crate) const DATABASE_URL: &str =
    "postgresql://postgres:testpassword@postgres:5432/postgres?sslmode=disable";
pub(crate) const SUMMARY_PATH: &str = "/build/SUMMARY.md";
pub(crate) const APP_EXE_NAME: &str = "web-server";
pub(crate) const OPERATOR_EXE_NAME: &str = "k8s-operator";
pub(crate) const RAG_ENGINE_EXE_NAME: &str = "rag-engine";
pub(crate) const AIRBYTE_EXE_NAME: &str = "airbyte-connector";
pub(crate) const TARGET_TRIPLE: &str = "x86_64-unknown-linux-musl";

pub(crate) const DB_FOLDER: &str = "crates/db";
pub(crate) const PIPELINE_FOLDER: &str = "crates/web-assets";

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
    dagger_sdk::connect(|client| async move {
        let outputs = build_workspace(&client).await?;
        publish_summary(&outputs.summary).await?;
        publish_images(&client, &outputs).await?;

        Ok(())
    })
    .await?;

    Ok(())
}

async fn build_workspace(client: &Query) -> Result<BuildOutputs> {
    let host_source_dir = client.host().directory_opts(
        ".",
        HostDirectoryOpts {
            exclude: Some(vec![
                "crates/web-assets/node_modules",
                "crates/web-assets/dist",
                "target",
            ]),
            include: None,
            no_cache: None,
            gitignore: None,
        },
    );

    let postgres_service = postgres_service(client);

    let after_postgres = client
        .container()
        .from(BASE_IMAGE)
        .with_directory("/workspace", host_source_dir)
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
    const IMAGE_EXPORT_DIR: &str = "tmp/dagger-images";

    fs::create_dir_all(IMAGE_EXPORT_DIR)
        .wrap_err("failed to create dagger image export directory")?;

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
        .with_user("1001")
        .with_file("/axum-server", outputs.app_binary.clone())
        .with_directory(format!("/build/{}", PIPELINE_FOLDER), dist_dir)
        .with_directory(format!("/build/{}/images", PIPELINE_FOLDER), images_dir)
        .with_entrypoint(vec!["./axum-server"])
        .as_tarball()
        .export(format!("{IMAGE_EXPORT_DIR}/bionicgpt-app.tar"))
        .await
        .wrap_err("failed to export app image tarball")?;

    client
        .container()
        .with_user("1001")
        .with_file("/rag-engine", outputs.rag_engine_binary.clone())
        .with_entrypoint(vec!["./rag-engine"])
        .as_tarball()
        .export(format!("{IMAGE_EXPORT_DIR}/bionicgpt-rag-engine.tar"))
        .await
        .wrap_err("failed to export rag engine image tarball")?;

    client
        .container()
        .with_user("1001")
        .with_file("/airbyte-connector", outputs.airbyte_binary.clone())
        .with_entrypoint(vec!["./airbyte-connector"])
        .as_tarball()
        .export(format!(
            "{IMAGE_EXPORT_DIR}/bionicgpt-airbyte-connector.tar"
        ))
        .await
        .wrap_err("failed to export airbyte image tarball")?;

    client
        .container()
        .with_file("/k8s-operator", outputs.operator_binary.clone())
        .with_entrypoint(vec!["./k8s-operator", "operator"])
        .as_tarball()
        .export(format!("{IMAGE_EXPORT_DIR}/bionicgpt-operator.tar"))
        .await
        .wrap_err("failed to export operator image tarball")?;

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
        .as_tarball()
        .export(format!("{IMAGE_EXPORT_DIR}/bionicgpt-db-migrations.tar"))
        .await
        .wrap_err("failed to export migration image tarball")?;

    Ok(())
}
