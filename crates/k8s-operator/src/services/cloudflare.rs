use crate::cli::apply;
use anyhow::Result;
use kube::Client;

const CLOUDFLARE_YAML: &str = include_str!("../../config/cloudflare.yaml");

pub async fn install(installer: &crate::cli::CloudflareInstaller) -> Result<()> {
    println!("Connecting to the cluster...");
    let client = Client::try_default().await?;
    println!("Connected");
    let yaml = CLOUDFLARE_YAML.replace("$TUNNEL_TOKEN", &installer.token);
    let yaml = yaml.replace("$TUNNEL_NAME", &installer.name);
    apply::apply(&client, &yaml, Some(&installer.namespace)).await?;
    println!("Cloudflare tunnel installed");
    Ok(())
}
