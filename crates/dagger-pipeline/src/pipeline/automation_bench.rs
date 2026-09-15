use std::env;

use dagger_sdk::{Container, Directory, Query};
use eyre::{Result, WrapErr, eyre};

use super::ci::{PublishCredentials, ensure_built, maybe_publish};

const ADAPTER_ROOT: &str = "crates/dagger-pipeline/automation-bench";
const IMAGE_REPO: &str = "ghcr.io/bionic-gpt/automationbench-api";

async fn build_adapter(
    client: &Query,
    repo: &Directory,
    automationbench_ref: &str,
) -> Result<(Container, String)> {
    let upstream_ref = client
        .git("https://github.com/zapier/AutomationBench")
        .r#ref(automationbench_ref);
    let upstream_commit = upstream_ref
        .commit()
        .await
        .wrap_err("failed to resolve AutomationBench commit")?;

    let dist = format!("/repo/{ADAPTER_ROOT}/dist");
    let builder = client
        .container()
        .from("python:3.13-slim")
        .with_directory("/repo", repo.clone())
        .with_directory("/upstream", upstream_ref.tree())
        .with_workdir("/repo")
        .with_env_variable("AUTOMATIONBENCH_PATH", "/upstream")
        .with_env_variable("AUTOMATIONBENCH_ROUTES", format!("{dist}/routes.json"))
        .with_env_variable(
            "PYTHONPATH",
            format!("/repo/{ADAPTER_ROOT}:/repo/{ADAPTER_ROOT}/tools"),
        )
        .with_exec(vec![
            "python",
            "-m",
            "pip",
            "install",
            "-r",
            &format!("{ADAPTER_ROOT}/requirements.txt"),
        ])
        .with_exec(vec!["rm", "-rf", &dist])
        .with_exec(vec![
            "python",
            &format!("{ADAPTER_ROOT}/tools/generate.py"),
            "--automationbench",
            "/upstream",
            "--output",
            &dist,
        ])
        .with_exec(vec![
            "python",
            &format!("{ADAPTER_ROOT}/tools/validate.py"),
            "--automationbench",
            "/upstream",
            "--output",
            &dist,
        ])
        .with_exec(vec!["pytest", "-q", &format!("{ADAPTER_ROOT}/tests")])
        .with_exec(vec![
            "cp",
            "-R",
            "/upstream/automationbench",
            &format!("{dist}/automationbench"),
        ])
        .with_new_file(
            format!("{dist}/automationbench-commit.txt"),
            upstream_commit.clone(),
        );

    Ok((builder, upstream_commit))
}

pub(super) async fn run(
    client: &Query,
    repo: &Directory,
    automationbench_ref: &str,
    local_tag: Option<&str>,
    publish: bool,
    output: &str,
) -> Result<()> {
    let (builder, upstream_commit) = build_adapter(client, repo, automationbench_ref).await?;
    let dist = builder.directory(format!("/repo/{ADAPTER_ROOT}/dist"));
    let image = dist.docker_build();
    ensure_built(&image, "AutomationBench API image").await?;

    if publish {
        let credentials = publish_credentials()?;
        maybe_publish(
            client,
            &image,
            IMAGE_REPO,
            Some(&credentials),
            "ghcr.io",
            "AutomationBench API image",
            &[upstream_commit.clone(), "latest".to_string()],
        )
        .await?;
        println!("Published AutomationBench commit {upstream_commit}");
    } else {
        let tag = local_tag.unwrap_or("bionic-gpt-automationbench:local");
        let image_id = image
            .id()
            .await
            .wrap_err("failed to materialize AutomationBench API image")?;
        let materialized_image = client.load_container_from_id(image_id);
        materialized_image
            .export_image(tag)
            .await
            .wrap_err_with(|| format!("failed to export AutomationBench API image as {tag}"))?;
        println!("Exported AutomationBench API image as {tag}");
    }

    dist.directory("openapi")
        .export(output)
        .await
        .wrap_err("failed to export AutomationBench OpenAPI documents")?;
    println!("Exported AutomationBench OpenAPI documents to {output}/");
    Ok(())
}

fn publish_credentials() -> Result<PublishCredentials> {
    let username = env::var("GHCR_USERNAME").or_else(|_| env::var("GITHUB_ACTOR"));
    let token = env::var("GHCR_TOKEN").or_else(|_| env::var("GITHUB_TOKEN"));
    match (username, token) {
        (Ok(username), Ok(token)) => Ok(PublishCredentials { username, token }),
        (Err(username), Err(token)) => Err(eyre!(
            "publishing AutomationBench requires GHCR credentials: username: {username}; token: {token}"
        )),
        (Err(username), Ok(_)) => Err(eyre!(
            "publishing AutomationBench requires GHCR username: {username}"
        )),
        (Ok(_), Err(token)) => Err(eyre!(
            "publishing AutomationBench requires GHCR token: {token}"
        )),
    }
}
