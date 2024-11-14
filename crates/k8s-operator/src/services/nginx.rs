use crate::cli::apply;
use anyhow::Result;
use kube::Client;
use serde_json::json;

use super::deployment;

pub const NGINX_NAME: &str = "nginx";
const NGINX_CONF: &str = include_str!("../../config/nginx-proxy.conf");

// The web user interface
pub async fn deploy_nginx(client: &Client, namespace: &str) -> Result<()> {
    let env = vec![];

    let image_name = "nginx:1.27.2".to_string();

    // Put the envoy.yaml into a ConfigMap
    let config_map = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": NGINX_NAME,
            "namespace": namespace
        },
        "data": {
            "default.conf": NGINX_CONF,
        }
    });

    apply::apply(client, &config_map.to_string(), Some(namespace)).await?;

    // Bionic with the migrations as a sidecar
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: NGINX_NAME.to_string(),
            image_name,
            replicas: 1,
            port: 7903,
            env,
            command: None,
            init_container: None,
            volume_mounts: vec![json!({"name": NGINX_NAME, "mountPath": "/etc/nginx/conf.d"})],
            volumes: vec![json!({"name": NGINX_NAME,
                "configMap": {
                    "name": NGINX_NAME
                }
            })],
        },
        namespace,
    )
    .await?;

    Ok(())
}
