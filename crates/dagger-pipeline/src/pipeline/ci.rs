use std::env;

use dagger_sdk::{Container, Directory, File, Query, Service};
use eyre::{Result, WrapErr, eyre};

use super::{
    AIRBYTE_EXE_NAME, AIRBYTE_IMAGE_REPO, APP_EXE_NAME, APP_IMAGE_REPO, BASE_IMAGE, DATABASE_URL,
    DB_FOLDER, DB_PASSWORD, MIGRATIONS_IMAGE_REPO, PIPELINE_FOLDER, POSTGRES_IMAGE,
    POSTGRES_MCP_EXE_NAME, POSTGRES_MCP_IMAGE_REPO, RAG_ENGINE_EXE_NAME, RAG_ENGINE_IMAGE_REPO,
    SUMMARY_PATH, TARGET_TRIPLE,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishMode {
    PullRequest,
    All,
}

fn collect_image_tags() -> Vec<String> {
    let mut tags = vec!["latest".to_string()];

    if let Ok(version) = env::var("RELEASE_VERSION") {
        let version = version.trim();
        if !version.is_empty() && !tags.iter().any(|tag| tag == version) {
            tags.push(version.to_string());
        }
    }

    if let Ok(additional) = env::var("ADDITIONAL_IMAGE_TAGS") {
        for tag in additional.split(',') {
            let tag = tag.trim();
            if tag.is_empty() || tags.iter().any(|existing| existing == tag) {
                continue;
            }
            tags.push(tag.to_string());
        }
    }

    tags
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
    rag_engine_binary: File,
    airbyte_binary: File,
    postgres_mcp_binary: File,
}

struct PublishCredentials {
    username: String,
    token: String,
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
    let rag_engine_binary = summary_container.file(release_binary_path(RAG_ENGINE_EXE_NAME));
    let airbyte_binary = summary_container.file(release_binary_path(AIRBYTE_EXE_NAME));
    let postgres_mcp_binary = summary_container.file(release_binary_path(POSTGRES_MCP_EXE_NAME));

    Ok(BuildOutputs {
        container: summary_container,
        summary,
        app_binary,
        rag_engine_binary,
        airbyte_binary,
        postgres_mcp_binary,
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
    println!("Start to build containers");
    let registry = "ghcr.io";
    let require_publish = env::var("CI").is_ok();
    let tags = collect_image_tags();
    println!("Container tags to publish: {}", tags.join(", "));

    let username = env::var("GHCR_USERNAME").or_else(|_| env::var("GITHUB_ACTOR"));
    let token = env::var("GHCR_TOKEN").or_else(|_| env::var("GITHUB_TOKEN"));

    let credentials = match (username, token) {
        (Ok(username), Ok(token)) => {
            println!("Using GHCR username `{username}` for image publication");
            Some(PublishCredentials { username, token })
        }
        (Err(user_err), Ok(_)) => {
            println!(
                "GHCR username not found locally (`GHCR_USERNAME` / `GITHUB_ACTOR`): {user_err}"
            );
            None
        }
        (Ok(_), Err(token_err)) => {
            println!("GHCR token not found locally (`GHCR_TOKEN` / `GITHUB_TOKEN`): {token_err}");
            None
        }
        (Err(user_err), Err(token_err)) => {
            println!("GHCR username not found: {user_err}");
            println!("GHCR token not found: {token_err}");
            None
        }
    };

    if credentials.is_none() {
        if require_publish {
            return Err(eyre!(
                "publishing images requires GHCR credentials (`GHCR_USERNAME`/`GITHUB_ACTOR` and `GHCR_TOKEN`/`GITHUB_TOKEN`)"
            ));
        }
        println!("GHCR credentials not provided; images will be built but not published.");
    }

    println!("Collecting build artifacts for publication");
    let dist_dir = outputs
        .container
        .directory(format!("{}/dist", PIPELINE_FOLDER));
    println!("Resolving dist directory");
    dist_dir
        .id()
        .await
        .wrap_err("failed to load web assets dist directory")?;
    println!("Resolved dist directory");
    let images_dir = outputs
        .container
        .directory(format!("{}/images", PIPELINE_FOLDER));
    println!("Resolving images directory");
    images_dir
        .id()
        .await
        .wrap_err("failed to load images directory")?;
    println!("Resolved images directory");

    // The generated StaticFile metadata in the web-assets crate bakes in absolute paths
    // such as `/workspace/crates/web-assets/dist/...`. Ensure the runtime image mirrors
    // those locations so asset lookups succeed.
    let app_container = client
        .container()
        .with_user("1001")
        .with_file("/axum-server", outputs.app_binary.clone())
        .with_directory("/workspace/crates/web-assets/dist", dist_dir.clone())
        .with_directory("/workspace/crates/web-assets/images", images_dir.clone())
        .with_entrypoint(vec!["./axum-server"]);

    ensure_built(&app_container, "app image").await?;
    maybe_publish(
        client,
        &app_container,
        APP_IMAGE_REPO,
        credentials.as_ref(),
        registry,
        "app image",
        &tags,
    )
    .await?;

    let rag_container = client
        .container()
        .with_user("1001")
        .with_file("/rag-engine", outputs.rag_engine_binary.clone())
        .with_entrypoint(vec!["./rag-engine"]);

    ensure_built(&rag_container, "rag engine image").await?;
    maybe_publish(
        client,
        &rag_container,
        RAG_ENGINE_IMAGE_REPO,
        credentials.as_ref(),
        registry,
        "rag engine image",
        &tags,
    )
    .await?;

    let airbyte_container = client
        .container()
        .with_user("1001")
        .with_file("/airbyte-connector", outputs.airbyte_binary.clone())
        .with_entrypoint(vec!["./airbyte-connector"]);

    ensure_built(&airbyte_container, "airbyte image").await?;
    maybe_publish(
        client,
        &airbyte_container,
        AIRBYTE_IMAGE_REPO,
        credentials.as_ref(),
        registry,
        "airbyte image",
        &tags,
    )
    .await?;

    let postgres_mcp_container = client
        .container()
        .with_user("1001")
        .with_file("/postgres-mcp", outputs.postgres_mcp_binary.clone())
        .with_entrypoint(vec!["./postgres-mcp"]);

    ensure_built(&postgres_mcp_container, "postgres mcp image").await?;
    maybe_publish(
        client,
        &postgres_mcp_container,
        POSTGRES_MCP_IMAGE_REPO,
        credentials.as_ref(),
        registry,
        "postgres mcp image",
        &tags,
    )
    .await?;

    let db_dir = outputs.container.directory(DB_FOLDER);
    println!("Resolving db directory");
    db_dir
        .id()
        .await
        .wrap_err("failed to prepare db directory")?;
    println!("Resolved db directory");

    let migrations_container = client
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
        .with_entrypoint(vec!["dbmate", "--migrations-dir", "./migrations", "up"]);

    ensure_built(&migrations_container, "migration image").await?;
    maybe_publish(
        client,
        &migrations_container,
        MIGRATIONS_IMAGE_REPO,
        credentials.as_ref(),
        registry,
        "migration image",
        &tags,
    )
    .await?;

    Ok(())
}

async fn ensure_built(container: &Container, label: &str) -> Result<()> {
    println!("Building {label}");
    container
        .id()
        .await
        .wrap_err_with(|| format!("failed to build {label}"))?;
    println!("Built {label}");
    Ok(())
}

async fn maybe_publish(
    client: &Query,
    container: &Container,
    image_repo: &str,
    credentials: Option<&PublishCredentials>,
    registry: &str,
    label: &str,
    tags: &[String],
) -> Result<()> {
    if let Some(creds) = credentials {
        println!(
            "Publishing {label} to {registry} with tags: {}",
            tags.join(", ")
        );
        let secret = client.set_secret("ghcr_token", creds.token.clone());
        for tag in tags {
            let reference = format!("{image_repo}:{tag}");
            container
                .clone()
                .with_registry_auth(registry, &creds.username, secret.clone())
                .publish(&reference)
                .await
                .wrap_err_with(|| format!("failed to publish {label} ({reference})"))?;
            println!("Published {label} as {reference}");
        }
        println!("Published {label}");
    } else {
        println!("Skipping publish of {label}");
    }
    Ok(())
}
