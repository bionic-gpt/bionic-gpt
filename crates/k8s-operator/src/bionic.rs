use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

// The web user interface
pub async fn deploy(
    client: Client,
    _name: &str,
    spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    // Bionic with the migrations as a sidecar
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "bionic-gpt".to_string(),
            image_name: format!("{:?}:{:?}", crate::BIONICGPT_IMAGE, spec.version),
            replicas: spec.replicas,
            port: 7903,
            env: vec![json!({
                "name": 
                "APP_DATABASE_URL", 
                "value": 
                "postgresql://bionic_application:testpassword@postgres:5432/bionic-gpt?sslmode=disable"
            }),
            json!({
                "name": 
                "PORT", 
                "value": 
                "7903"
            })],
            init_container: Some(deployment::InitContainer {
                image_name: format!("{:?}:{:?}", crate::BIONICGPT_DB_MIGRATIONS_IMAGE, spec.version),
                env: vec![json!({
                    "name": 
                    "DATABASE_URL", 
                    "value": 
                    "postgresql://postgres:testpassword@postgres:5432/bionic-gpt?sslmode=disable"
                })]
            }),
            command: None,
            expose_service: false,
            volume_mounts: vec![],
            volumes: vec![]
        },
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn delete(client: Client, _name: &str, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete("bionic-gpt", &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client, namespace);
    api.delete("bionic-gpt", &DeleteParams::default()).await?;
    Ok(())
}
