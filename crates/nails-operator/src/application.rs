use crate::deployment;
use crate::error::Error;
use crate::Installer;
use anyhow::Result;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

// The web user interface
pub async fn deploy_application(
    client: &Client,
    installer: &Installer,
    namespace: &str,
) -> Result<()> {
    let env = vec![
        json!({
            "name":
            "APP_DATABASE_URL",
            "valueFrom": {
                "secretKeyRef": {
                    "name": "database-urls",
                    "key": "application-url"
                }
            }
        }),
        json!({
            "name":
            "PORT",
            "value":
            "7903"
        }),
        json!({
            "name":
            "ECHO_ADDR",
            "value":
            ":7903"
        }),
        json!({
            "name":
            "LOGOUT_URL",
            "value":
            "/oidc/realms/bionic-gpt/protocol/openid-connect/logout"
        }),
    ];

    let image_name = "bionic-gpt/bionicgpt:latest".to_string();

    // Bionic with the migrations as a sidecar
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: installer.app_name.clone(),
            image_name,
            replicas: installer.replicas,
            port: 7903,
            env,
            command: None,
            volume_mounts: vec![],
            volumes: vec![],
            init_container: None,
        },
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn _delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get("bionic-gpt").await.is_ok() {
        api.delete("bionic-gpt", &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client, namespace);
    if api.get("bionic-gpt").await.is_ok() {
        api.delete("bionic-gpt", &DeleteParams::default()).await?;
    }
    Ok(())
}
