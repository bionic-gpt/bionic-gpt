use crate::args::CliTarget;
use dagger_sdk::{Directory, Query};
use eyre::{Result, WrapErr};

use super::{BASE_IMAGE, container_from};

pub(super) async fn build_cli(client: &Query, repo: &Directory, target: CliTarget) -> Result<()> {
    match target {
        CliTarget::Linux => build_cli_linux(client, repo).await,
        CliTarget::Macos => build_cli_macos(client, repo).await,
        CliTarget::Windows => build_cli_windows(client, repo).await,
    }
}

async fn build_cli_linux(client: &Query, repo: &Directory) -> Result<()> {
    let container = container_from(client, BASE_IMAGE)
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
    let container = container_from(client, "joseluisq/rust-linux-darwin-builder:1.85")
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
    let container = container_from(client, BASE_IMAGE)
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
