use crate::error::Error;
use crate::operator::crd::BionicSpec;
use crate::services::deployment;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

// The web user interface
pub async fn deploy(client: Client, spec: BionicSpec, namespace: &str) -> Result<(), Error> {
    // Bionic with the migrations as a sidecar
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "bionic-gpt".to_string(),
            image_name: format!("{}:{}", super::BIONICGPT_IMAGE, spec.version),
            replicas: spec.replicas,
            port: 7903,
            env: vec![
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
                    "LOGOUT_URL",
                    "value":
                    "/oidc/realms/bionic-gpt/protocol/openid-connect/logout"
                }),
            ],
            init_container: Some(deployment::InitContainer {
                image_name: format!("{}:{}", super::BIONICGPT_DB_MIGRATIONS_IMAGE, spec.version),
                env: vec![json!({
                "name":
                "DATABASE_URL",
                "valueFrom": {
                    "secretKeyRef": {
                        "name": "database-urls",
                        "key": "migrations-url"
                    }
                }})],
            }),
            command: None,
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete("bionic-gpt", &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client, namespace);
    api.delete("bionic-gpt", &DeleteParams::default()).await?;
    Ok(())
}
