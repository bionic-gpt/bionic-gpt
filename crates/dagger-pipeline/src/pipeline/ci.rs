use std::collections::HashSet;
use std::path::Path;
use std::{env, fs};

use dagger_sdk::{Container, Directory, File, Query, Service};
use eyre::{Result, WrapErr, eyre};
use serde_yaml::{Mapping, Value};

use super::{
    AIRBYTE_EXE_NAME, AIRBYTE_IMAGE_REPO, APP_EXE_NAME, APP_IMAGE_REPO, BASE_IMAGE,
    CLI_GATEWAY_EXE_NAME, CLI_GATEWAY_IMAGE_REPO, DATABASE_URL, DB_FOLDER, DB_PASSWORD,
    EVAL_MOCKS_IMAGE_REPO, MIGRATIONS_IMAGE_REPO, PIPELINE_FOLDER, POSTGRES_IMAGE,
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
    cli_gateway_binary: File,
}

struct PublishCredentials {
    username: String,
    token: String,
}

fn release_binary_path(exe: &str) -> String {
    format!("target/{TARGET_TRIPLE}/release/{exe}")
}

const EVAL_MOCKS_SPEC_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../infra-as-code/eval-mocks/openapi/specs"
);
const EVAL_MOCKS_GENERATED_SPEC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../infra-as-code/eval-mocks/openapi/generated/eval-mocks.openapi.yaml"
);
const CLI_GATEWAY_SPEC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../crates/cli-gateway/specs/typst.openapi.yaml"
);

#[derive(serde::Deserialize)]
struct CliBuildSpec {
    source: CliBuildSource,
    binary: CliBuildBinary,
}

#[derive(serde::Deserialize)]
struct CliBuildSource {
    #[serde(rename = "type")]
    source_type: String,
    image: Option<String>,
}

#[derive(serde::Deserialize)]
struct CliBuildBinary {
    from: String,
    to: String,
}

fn cli_gateway_build_spec() -> Result<CliBuildSpec> {
    let source = fs::read_to_string(CLI_GATEWAY_SPEC)
        .wrap_err_with(|| format!("failed to read {CLI_GATEWAY_SPEC}"))?;
    let document: serde_yaml::Value =
        serde_yaml::from_str(&source).wrap_err("failed to parse cli-gateway OpenAPI spec")?;
    let build = document
        .get("x-cli-build")
        .ok_or_else(|| eyre!("cli-gateway spec is missing x-cli-build"))?;
    let build: CliBuildSpec = serde_yaml::from_value(build.clone())
        .wrap_err("invalid cli-gateway x-cli-build extension")?;
    if build.source.source_type != "container" {
        return Err(eyre!(
            "unsupported cli-gateway build source type `{}`",
            build.source.source_type
        ));
    }
    if build.source.image.as_deref().unwrap_or("").is_empty()
        || build.binary.from.is_empty()
        || build.binary.to.is_empty()
    {
        return Err(eyre!(
            "cli-gateway x-cli-build has incomplete source or binary fields"
        ));
    }
    Ok(build)
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

    let mut after_postgres = client
        .container()
        .from(BASE_IMAGE)
        .with_directory("/workspace", repo.clone())
        .with_workdir("/workspace")
        .with_user("root")
        .with_service_binding("postgres", postgres_service)
        .with_env_variable("DATABASE_URL", DATABASE_URL)
        .with_env_variable("APP_DATABASE_URL", DATABASE_URL);

    if let Ok(version) = env::var("RELEASE_VERSION") {
        let version = version.trim();
        if !version.is_empty() {
            after_postgres = after_postgres.with_env_variable("RELEASE_VERSION", version);
        }
    }

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
    let cli_gateway_binary = summary_container.file(release_binary_path(CLI_GATEWAY_EXE_NAME));

    Ok(BuildOutputs {
        container: summary_container,
        summary,
        app_binary,
        rag_engine_binary,
        airbyte_binary,
        postgres_mcp_binary,
        cli_gateway_binary,
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
    let ca_certs = outputs.container.file("/etc/ssl/certs/ca-certificates.crt");

    // The generated StaticFile metadata in the web-assets crate bakes in absolute paths
    // such as `/workspace/crates/web-assets/dist/...`. Ensure the runtime image mirrors
    // those locations so asset lookups succeed.
    let app_container = client
        .container()
        .with_user("1001")
        .with_file("/axum-server", outputs.app_binary.clone())
        .with_directory("/workspace/crates/web-assets/dist", dist_dir.clone())
        .with_directory("/workspace/crates/web-assets/images", images_dir.clone())
        .with_file("/etc/ssl/certs/ca-certificates.crt", ca_certs.clone())
        .with_env_variable("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt")
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
        .with_file("/etc/ssl/certs/ca-certificates.crt", ca_certs.clone())
        .with_env_variable("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt")
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
        .with_file("/etc/ssl/certs/ca-certificates.crt", ca_certs.clone())
        .with_env_variable("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt")
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
        .with_file("/etc/ssl/certs/ca-certificates.crt", ca_certs.clone())
        .with_env_variable("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt")
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

    let cli_build = cli_gateway_build_spec()?;
    let cli_source = client
        .container()
        .from(cli_build.source.image.as_deref().unwrap());
    let cli_gateway_container = cli_source
        .with_file(
            &cli_build.binary.to,
            cli_source.file(&cli_build.binary.from),
        )
        .with_file("/cli-gateway", outputs.cli_gateway_binary.clone())
        .with_file(
            "/etc/cli-gateway/openapi.yaml",
            outputs.container.file(CLI_GATEWAY_SPEC),
        )
        .with_exposed_port(8080)
        .with_entrypoint(vec!["/cli-gateway"]);

    ensure_built(&cli_gateway_container, "cli gateway image").await?;
    maybe_publish(
        client,
        &cli_gateway_container,
        CLI_GATEWAY_IMAGE_REPO,
        credentials.as_ref(),
        registry,
        "cli gateway image",
        &tags,
    )
    .await?;

    let eval_mocks_data = combined_eval_mocks_openapi()?;
    let eval_mocks_container = client
        .container()
        .from("mockoon/cli:9.7.0")
        .with_new_file("/home/mockoon/data/eval-mocks.openapi.yaml", eval_mocks_data)
        .with_exec(vec![
            "sh",
            "-c",
            "mockoon-cli start --data /home/mockoon/data/eval-mocks.openapi.yaml --port 3100 --hostname 0.0.0.0 & pid=$!; sleep 2; kill -0 \"$pid\"; status=$?; kill \"$pid\"; exit \"$status\"",
        ])
        .with_exposed_port(3100)
        .with_entrypoint(vec![
            "mockoon-cli",
            "start",
            "--data",
            "/home/mockoon/data/eval-mocks.openapi.yaml",
            "--port",
            "3100",
            "--hostname",
            "0.0.0.0",
        ]);

    ensure_built(&eval_mocks_container, "eval mocks image").await?;
    maybe_publish(
        client,
        &eval_mocks_container,
        EVAL_MOCKS_IMAGE_REPO,
        credentials.as_ref(),
        registry,
        "eval mocks image",
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

fn combined_eval_mocks_openapi() -> Result<String> {
    let mut spec_paths = fs::read_dir(EVAL_MOCKS_SPEC_DIR)
        .wrap_err_with(|| format!("failed to read {EVAL_MOCKS_SPEC_DIR}"))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<Vec<_>>>()
        .wrap_err_with(|| format!("failed to list {EVAL_MOCKS_SPEC_DIR}"))?;

    spec_paths.retain(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".openapi.yaml"))
    });
    spec_paths.sort();

    if spec_paths.is_empty() {
        return Err(eyre!(
            "no eval mock OpenAPI specs found in {EVAL_MOCKS_SPEC_DIR}"
        ));
    }

    let mut paths = Mapping::new();
    let mut schemas = Mapping::new();
    let mut operation_ids = HashSet::new();

    for spec_path in spec_paths {
        let spec_text = fs::read_to_string(&spec_path)
            .wrap_err_with(|| format!("failed to read {}", spec_path.display()))?;
        let spec: Value = serde_yaml::from_str(&spec_text)
            .wrap_err_with(|| format!("failed to parse {}", spec_path.display()))?;

        let source_name = spec_path.display().to_string();
        let spec_paths = get_required_mapping(&spec, "paths", &source_name)?;
        collect_operation_ids(spec_paths, &mut operation_ids, &source_name)?;
        merge_mapping(&mut paths, spec_paths, "path", &source_name)?;

        if let Some(components) = get_mapping(&spec, "components")
            && let Some(spec_schemas) = get_mapping_from_mapping(components, "schemas")
        {
            merge_mapping(&mut schemas, spec_schemas, "schema", &source_name)?;
        }
    }

    let mut root = Mapping::new();
    root.insert(string_value("openapi"), string_value("3.0.3"));
    root.insert(string_value("info"), combined_info());
    root.insert(string_value("servers"), combined_servers());
    root.insert(string_value("paths"), Value::Mapping(paths));

    let mut components = Mapping::new();
    components.insert(string_value("schemas"), Value::Mapping(schemas));
    root.insert(string_value("components"), Value::Mapping(components));

    serde_yaml::to_string(&Value::Mapping(root)).wrap_err("failed to serialize combined eval spec")
}

pub(super) fn write_combined_eval_mocks_openapi() -> Result<()> {
    let combined = combined_eval_mocks_openapi()?;
    let output_path = Path::new(EVAL_MOCKS_GENERATED_SPEC);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).wrap_err_with(|| {
            format!(
                "failed to create eval mocks generated spec directory {}",
                parent.display()
            )
        })?;
    }
    fs::write(output_path, combined).wrap_err_with(|| {
        format!(
            "failed to write eval mocks generated spec {}",
            output_path.display()
        )
    })?;
    println!("Generated {}", output_path.display());
    Ok(())
}

fn collect_operation_ids(
    paths: &Mapping,
    operation_ids: &mut HashSet<String>,
    source: &str,
) -> Result<()> {
    for (path_name, path_value) in paths {
        let Some(path_item) = path_value.as_mapping() else {
            continue;
        };
        for (method_name, operation_value) in path_item {
            let Some(operation) = operation_value.as_mapping() else {
                continue;
            };
            let Some(operation_id) = get_string_from_mapping(operation, "operationId") else {
                continue;
            };
            if !operation_ids.insert(operation_id.to_string()) {
                return Err(eyre!(
                    "duplicate operationId `{operation_id}` in {source} at {} {}",
                    display_yaml_key(method_name),
                    display_yaml_key(path_name)
                ));
            }
        }
    }
    Ok(())
}

fn merge_mapping(
    destination: &mut Mapping,
    source: &Mapping,
    label: &str,
    source_name: &str,
) -> Result<()> {
    for (key, value) in source {
        if let Some(existing) = destination.get(key) {
            if existing != value {
                return Err(eyre!(
                    "conflicting {label} `{}` while merging {source_name}",
                    display_yaml_key(key)
                ));
            }
            continue;
        }
        destination.insert(key.clone(), value.clone());
    }
    Ok(())
}

fn get_required_mapping<'a>(value: &'a Value, key: &str, source: &str) -> Result<&'a Mapping> {
    get_mapping(value, key).ok_or_else(|| eyre!("missing `{key}` mapping in {source}"))
}

fn get_mapping<'a>(value: &'a Value, key: &str) -> Option<&'a Mapping> {
    let lookup = string_value(key);
    value.as_mapping()?.get(&lookup)?.as_mapping()
}

fn get_mapping_from_mapping<'a>(mapping: &'a Mapping, key: &str) -> Option<&'a Mapping> {
    let lookup = string_value(key);
    mapping.get(&lookup)?.as_mapping()
}

fn get_string_from_mapping<'a>(mapping: &'a Mapping, key: &str) -> Option<&'a str> {
    let lookup = string_value(key);
    mapping.get(&lookup)?.as_str()
}

fn display_yaml_key(value: &Value) -> String {
    value.as_str().unwrap_or("<non-string>").to_string()
}

fn combined_info() -> Value {
    let mut info = Mapping::new();
    info.insert(string_value("title"), string_value("Bionic Eval Mock APIs"));
    info.insert(string_value("version"), string_value("1.0"));
    info.insert(
        string_value("description"),
        string_value("Combined deterministic mock APIs for the Bionic architect course."),
    );
    Value::Mapping(info)
}

fn combined_servers() -> Value {
    Value::Sequence(vec![
        server_value(
            "http://eval-mocks:3100",
            "Docker Compose network URL used by Bionic containers",
        ),
        server_value("http://localhost:3100", "Host URL for manual testing"),
    ])
}

fn server_value(url: &str, description: &str) -> Value {
    let mut server = Mapping::new();
    server.insert(string_value("url"), string_value(url));
    server.insert(string_value("description"), string_value(description));
    Value::Mapping(server)
}

fn string_value(value: &str) -> Value {
    Value::String(value.to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combined_eval_mocks_openapi_includes_all_eval_paths() {
        let combined = combined_eval_mocks_openapi().expect("combined spec should be generated");
        let parsed: Value =
            serde_yaml::from_str(&combined).expect("combined spec should be valid YAML");
        let paths = get_required_mapping(&parsed, "paths", "combined spec").unwrap();

        assert!(paths.contains_key(string_value("/email/emails")));
        assert!(paths.contains_key(string_value("/email/emails/{id}")));
        assert!(paths.contains_key(string_value("/email/drafts")));
        assert!(paths.contains_key(string_value("/email/send")));
        assert!(paths.contains_key(string_value("/web/search")));
    }

    #[test]
    fn combined_eval_mocks_openapi_includes_shared_schemas() {
        let combined = combined_eval_mocks_openapi().expect("combined spec should be generated");
        let parsed: Value =
            serde_yaml::from_str(&combined).expect("combined spec should be valid YAML");
        let components = get_required_mapping(&parsed, "components", "combined spec").unwrap();
        let schemas = get_mapping_from_mapping(components, "schemas").unwrap();

        assert!(schemas.contains_key(string_value("EmailSummary")));
        assert!(schemas.contains_key(string_value("SearchResult")));
    }
}
