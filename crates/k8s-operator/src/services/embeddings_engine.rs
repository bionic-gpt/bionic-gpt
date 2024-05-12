use super::deployment;
use crate::error::Error;
use crate::operator::crd::BionicSpec;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};

pub const NAME: &str = "embeddings-api";
pub const PORT: u16 = 80;

// An API for embeddings
pub async fn deploy(client: Client, spec: BionicSpec, namespace: &str) -> Result<(), Error> {
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: NAME.to_string(),
            image_name: super::EMBEDDINGS_ENGINE_IMAGE.to_string(),
            replicas: spec.replicas,
            port: PORT,
            env: vec![],
            init_container: None,
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
    if api.get(NAME).await.is_ok() {
        api.delete(NAME, &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get(NAME).await.is_ok() {
        api.delete(NAME, &DeleteParams::default()).await?;
    }

    Ok(())
}
