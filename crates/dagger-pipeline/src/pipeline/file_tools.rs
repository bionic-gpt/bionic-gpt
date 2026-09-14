use std::env;

use dagger_sdk::{Container, Directory, Query};
use eyre::{Result, WrapErr, eyre};

use super::ci::{PublishCredentials, ensure_built, maybe_publish};

const OFFICE_TOOLS_ROOT: &str = "crates/dagger-pipeline/office-tools";
const OFFICE_TOOLS_IMAGE_REPO: &str = "ghcr.io/bionic-gpt/office-tools";

async fn office_spec_builder(
    client: &Query,
    repo: &Directory,
    archipelago_ref: &str,
) -> Result<(Container, String)> {
    let upstream_ref = client
        .git("https://github.com/Mercor-Intelligence/archipelago")
        .r#ref(archipelago_ref);
    let upstream_commit = upstream_ref
        .commit()
        .await
        .wrap_err("failed to resolve Archipelago commit")?;
    let upstream_tree = upstream_ref.tree();

    let builder = client
        .container()
        .from("python:3.13-slim")
        .with_directory("/repo", repo.clone())
        .with_directory("/upstream", upstream_tree)
        .with_workdir("/repo")
        .with_exec(vec![
            "python",
            "-m",
            "pip",
            "install",
            "-r",
            &format!("{OFFICE_TOOLS_ROOT}/requirements.txt"),
        ])
        .with_exec(vec!["rm", "-rf", &format!("{OFFICE_TOOLS_ROOT}/dist")])
        .with_exec(vec![
            "python",
            &format!("{OFFICE_TOOLS_ROOT}/tools/stage_archipelago.py"),
            "--archipelago",
            "/upstream",
            "--output",
            &format!("{OFFICE_TOOLS_ROOT}/dist"),
        ])
        .with_env_variable(
            "ARCHIPELAGO_ROOT",
            format!("/repo/{OFFICE_TOOLS_ROOT}/dist/archipelago"),
        )
        .with_env_variable(
            "PYTHONPATH",
            format!("/repo/{OFFICE_TOOLS_ROOT}:/repo/{OFFICE_TOOLS_ROOT}/dist"),
        )
        .with_exec(vec![
            "python",
            &format!("{OFFICE_TOOLS_ROOT}/tools/generate_openapi.py"),
            "--staged",
            &format!("{OFFICE_TOOLS_ROOT}/dist"),
            "--output",
            &format!("{OFFICE_TOOLS_ROOT}/dist/openapi"),
        ])
        .with_exec(vec![
            "python",
            &format!("{OFFICE_TOOLS_ROOT}/tools/validate.py"),
            "--openapi",
            &format!("{OFFICE_TOOLS_ROOT}/dist/openapi"),
        ]);

    Ok((builder, upstream_commit))
}

pub(super) async fn generate_specs(
    client: &Query,
    repo: &Directory,
    archipelago_ref: &str,
    output: &str,
) -> Result<()> {
    let (builder, upstream_commit) = office_spec_builder(client, repo, archipelago_ref).await?;
    builder
        .directory(format!("/repo/{OFFICE_TOOLS_ROOT}/dist/openapi"))
        .export(output)
        .await
        .wrap_err("failed to export built-in Office OpenAPI specs")?;
    println!("Generated Office OpenAPI specs from Archipelago {upstream_commit} in {output}");
    Ok(())
}

pub(super) async fn run(
    client: &Query,
    repo: &Directory,
    archipelago_ref: &str,
    local_tag: Option<&str>,
    publish: bool,
) -> Result<()> {
    let (builder, upstream_commit) = office_spec_builder(client, repo, archipelago_ref).await?;
    let builder = builder
        .with_exec(vec!["apt-get", "update"])
        .with_exec(vec![
            "apt-get",
            "install",
            "-y",
            "--no-install-recommends",
            "libreoffice-calc",
            "libreoffice-impress",
            "libreoffice-writer",
        ])
        .with_exec(vec!["rm", "-rf", "/var/lib/apt/lists"])
        .with_exec(vec!["pytest", "-q", &format!("{OFFICE_TOOLS_ROOT}/tests")])
        .with_exec(vec![
            "cp",
            "-R",
            &format!("{OFFICE_TOOLS_ROOT}/server"),
            &format!("{OFFICE_TOOLS_ROOT}/dist/server"),
        ])
        .with_exec(vec![
            "cp",
            &format!("{OFFICE_TOOLS_ROOT}/requirements.txt"),
            &format!("{OFFICE_TOOLS_ROOT}/dist/requirements.txt"),
        ])
        .with_exec(vec![
            "cp",
            &format!("{OFFICE_TOOLS_ROOT}/Dockerfile"),
            &format!("{OFFICE_TOOLS_ROOT}/dist/Dockerfile"),
        ])
        .with_exec(vec![
            "cp",
            &format!("{OFFICE_TOOLS_ROOT}/README.md"),
            &format!("{OFFICE_TOOLS_ROOT}/dist/README.md"),
        ]);

    let dist = builder.directory(format!("/repo/{OFFICE_TOOLS_ROOT}/dist"));
    let office_image = dist.docker_build();
    ensure_built(&office_image, "Office tools image").await?;

    if publish {
        let credentials = office_publish_credentials()?;
        maybe_publish(
            client,
            &office_image,
            OFFICE_TOOLS_IMAGE_REPO,
            Some(&credentials),
            "ghcr.io",
            "Office tools image",
            &[upstream_commit.clone(), "latest".to_string()],
        )
        .await?;
        println!("Published Archipelago commit {upstream_commit}");
    } else {
        let tag = local_tag.unwrap_or("bionic-gpt-office-tools:local");
        let image_id = office_image
            .id()
            .await
            .wrap_err("failed to materialize Office tools image")?;
        let materialized_image = client.load_container_from_id(image_id);
        materialized_image
            .export_image(tag)
            .await
            .wrap_err_with(|| format!("failed to export Office tools image as {tag}"))?;
        println!("Exported Office tools image as {tag}");
    }

    dist.directory("openapi")
        .export("office-tools-openapi")
        .await
        .wrap_err("failed to export Office OpenAPI documents")?;
    println!("Exported Office OpenAPI documents to office-tools-openapi/");
    Ok(())
}

fn office_publish_credentials() -> Result<PublishCredentials> {
    let username = env::var("GHCR_USERNAME").or_else(|_| env::var("GITHUB_ACTOR"));
    let token = env::var("GHCR_TOKEN").or_else(|_| env::var("GITHUB_TOKEN"));
    match (username, token) {
        (Ok(username), Ok(token)) => Ok(PublishCredentials { username, token }),
        (Err(username), Err(token)) => Err(eyre!(
            "publishing Office tools requires GHCR credentials: username: {username}; token: {token}"
        )),
        (Err(username), Ok(_)) => Err(eyre!(
            "publishing Office tools requires GHCR username: {username}"
        )),
        (Ok(_), Err(token)) => Err(eyre!(
            "publishing Office tools requires GHCR token: {token}"
        )),
    }
}
