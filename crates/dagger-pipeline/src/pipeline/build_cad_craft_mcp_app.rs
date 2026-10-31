use super::ci::{ensure_built, ghcr_credentials, maybe_publish};
use dagger_sdk::{Directory, Query};
use eyre::{Result, WrapErr};

pub(super) async fn run(
    client: &Query,
    repo: &Directory,
    upstream: &str,
    tag: Option<&str>,
    publish: bool,
) -> Result<()> {
    let source = client
        .git("https://github.com/storytold/cadcraft")
        .r#ref(upstream);
    let sha = source.commit().await.wrap_err("resolve CADCraft ref")?;
    let wasm = client
        .container()
        .from("rust:1.95-bookworm")
        .with_exec(vec!["rustup", "target", "add", "wasm32-unknown-unknown"])
        .with_exec(vec![
            "cargo",
            "install",
            "trunk",
            "--version",
            "0.21.14",
            "--locked",
        ])
        .with_directory(
            "/src",
            client
                .git("https://github.com/storytold/cadcraft")
                .r#ref(&sha)
                .tree(),
        )
        .with_workdir("/src/apps/cadcraft-web")
        .with_exec(vec![
            "trunk",
            "build",
            "--release",
            "--locked",
            "--dist",
            "/dist",
            "--public-url",
            "./",
        ]);
    let builder = client
        .container()
        .from("rust:1.95-bookworm")
        .with_directory(
            "/launcher",
            repo.directory("crates/dagger-pipeline/cad-craft-mcp-app"),
        )
        .with_workdir("/launcher")
        .with_exec(vec!["cargo", "build", "--release", "--locked"]);
    let wasm = wasm.with_new_file("/dist/mcp-bridge.js", r#"
if (window.parent !== window) {
let ready = false;
window.addEventListener('message', event => {
 if (event.source !== window.parent) return;
 const m = event.data;
 if (m?.id === 'cadcraft-init' && m.result) {
  ready = true;
  window.parent.postMessage({jsonrpc:'2.0',method:'ui/notifications/initialized'}, '*');
  window.parent.postMessage({jsonrpc:'2.0',method:'ui/notifications/size-changed',params:{height:700}}, '*');
 }
 if (m?.method === 'ui/resource-teardown') window.parent.postMessage({jsonrpc:'2.0',id:m.id,result:{}}, '*');
});
window.parent.postMessage({jsonrpc:'2.0',id:'cadcraft-init',method:'ui/initialize',params:{protocolVersion:'2026-01-26',appInfo:{name:'CADCraft',version:'0.1.0'},appCapabilities:{}}}, '*');
}
"#).with_exec(vec!["sh", "-c", r#"sed -i 's@</body>@<script src="./mcp-bridge.js"></script></body>@' /dist/index.html"#]);
    let image = client
        .container()
        .from("debian:bookworm-slim")
        .with_workdir("/app")
        .with_file(
            "/app/server",
            builder.file("/launcher/target/release/cad-craft-mcp-app"),
        )
        .with_directory("/app/assets", wasm.directory("/dist"))
        .with_user("1001")
        .with_exposed_port(8080)
        .with_entrypoint(vec!["/app/server"]);
    ensure_built(&image, "CADCraft MCP App").await?;
    if publish {
        let credentials = ghcr_credentials(true)?;
        maybe_publish(
            client,
            &image,
            "ghcr.io/bionic-gpt/bionicgpt-cad-craft-mcp-app",
            credentials.as_ref(),
            "ghcr.io",
            "CADCraft MCP App",
            &[sha, "latest".into()],
        )
        .await?;
    } else {
        let archive = std::env::temp_dir().join(format!("cadcraft-{}.tar", std::process::id()));
        image.export(archive.to_string_lossy()).await?;
        let loaded = std::process::Command::new("docker")
            .args(["load", "--input"])
            .arg(&archive)
            .output()?;
        eyre::ensure!(
            loaded.status.success(),
            "docker load failed: {}",
            String::from_utf8_lossy(&loaded.stderr)
        );
        let output = String::from_utf8(loaded.stdout)?;
        let id = output
            .lines()
            .find_map(|line| {
                line.strip_prefix("Loaded image ID: ")
                    .or_else(|| line.strip_prefix("Loaded image: "))
            })
            .ok_or_else(|| eyre::eyre!("unexpected docker load response: {output}"))?;
        let status = std::process::Command::new("docker")
            .args([
                "tag",
                id,
                tag.unwrap_or("bionic-gpt-cad-craft-mcp-app:local"),
            ])
            .status()?;
        eyre::ensure!(status.success(), "docker tag failed");
        std::fs::remove_file(archive)?;
    }
    Ok(())
}
