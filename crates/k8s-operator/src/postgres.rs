use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

// Postgres and pgVector
pub async fn deploy(
    client: Client,
    _name: &str,
    spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    // First Postgres with pgVector
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "postgres".to_string(),
            image_name: crate::POSTGRES_PGVECTOR_IMAGE.to_string(),
            replicas: spec.replicas,
            port: 5432,
            env: vec![
                json!({"name": "POSTGRES_PASSWORD", "value": "testpassword"}),
                json!({"name": "POSTGRES_USER", "value": "postgres"}),
                json!({"name": "POSTGRES_DB", "value": "keycloak"}),
            ],
            init_container: None,
            command: None,
            expose_service: false,
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn delete(client: Client, _name: &str, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete("postgres", &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client, namespace);
    api.delete("postgres", &DeleteParams::default()).await?;
    Ok(())
}
