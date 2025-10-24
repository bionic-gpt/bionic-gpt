use crate::args::CliTarget;
use dagger_sdk::{Directory, Query};
use eyre::{Result, WrapErr};

use super::BASE_IMAGE;

pub(super) async fn build_cli(client: &Query, repo: &Directory, target: CliTarget) -> Result<()> {
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
        .with_directory("/workspace", repo.clone())
        .with_workdir("/workspace/crates/k8s-operator")
        .with_user("root")
        .with_exec(vec!["cargo", "build", "--release"]);

    container
        .file("/workspace/target/release/k8s-operator")
        .export("bionic-cli-linux")
        .await
        .wrap_err("failed to export linux cli")?;

    Ok(())
}

async fn build_cli_macos(client: &Query, repo: &Directory) -> Result<()> {
    let container = client
        .container()
        .from("joseluisq/rust-linux-darwin-builder:1.85")
        .with_directory("/build", repo.clone())
        .with_workdir("/build/crates/k8s-operator")
        .with_env_variable("CC", "o64-clang")
        .with_env_variable("CXX", "o64-clang++")
        .with_exec(vec![
            "cargo",
            "build",
            "--release",
            "--target",
            "x86_64-apple-darwin",
        ]);

    container
        .file("/build/target/x86_64-apple-darwin/release/k8s-operator")
        .export("bionic-cli-darwin")
        .await
        .wrap_err("failed to export macos cli")?;

    Ok(())
}

async fn build_cli_windows(client: &Query, repo: &Directory) -> Result<()> {
    let container = client
        .container()
        .from(BASE_IMAGE)
        .with_directory("/build", repo.clone())
        .with_workdir("/build")
        .with_exec(vec!["sudo", "apt", "update"])
        .with_exec(vec!["sudo", "apt", "upgrade", "-y"])
        .with_exec(vec!["sudo", "apt", "install", "-y", "g++-mingw-w64-x86-64"])
        .with_exec(vec!["rustup", "target", "add", "x86_64-pc-windows-gnu"])
        .with_workdir("/build/crates/k8s-operator")
        .with_user("root")
        .with_exec(vec![
            "cargo",
            "build",
            "--release",
            "--target",
            "x86_64-pc-windows-gnu",
        ]);

    container
        .file("/build/target/x86_64-pc-windows-gnu/release/k8s-operator.exe")
        .export("bionic-cli-windows.exe")
        .await
        .wrap_err("failed to export windows cli")?;

    Ok(())
}
