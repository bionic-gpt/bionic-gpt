use anyhow::Result;

const CLOUDFLARE_YAML: &str = include_str!("../../config/cloudflare.yaml");

pub async fn install(installer: &crate::cli::CloudflareInstaller) -> Result<()> {
    let yaml = CLOUDFLARE_YAML.replace("$TUNNEL_TOKEN", &installer.token);
    let yaml = yaml.replace("$TUNNEL_NAME", &installer.name);
    println!("{}", yaml);
    Ok(())
}
