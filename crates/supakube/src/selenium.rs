use crate::deployment;
use anyhow::Result;
use kube::Client;
use serde_json::json;

pub const SELENIUM_NAME: &str = "selenium-standalone-chrome";

// The web user interface
pub async fn deploy_selenium(client: &Client, namespace: &str) -> Result<()> {
    let env = vec![json!({
        "name":
        "VNC_NO_PASSWORD",
        "value":
        "1"
    })];

    let image_name = "selenium/standalone-chrome:4".to_string();

    // Bionic with the migrations as a sidecar
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: SELENIUM_NAME.to_string(),
            image_name,
            replicas: 1,
            port: 7903,
            env,
            command: None,
            init_container: None,
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await?;

    Ok(())
}
