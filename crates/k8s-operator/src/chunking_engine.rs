use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};

// Chunking engine - Turn documents into text chunks
pub async fn deploy(
    client: Client,
    _name: &str,
    spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    // Chunking engine - Turn documents into text chunks
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "chunking-engine".to_string(),
            image_name: spec.chunking_engine_image,
            replicas: spec.replicas,
            port: 8000,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![],
            }),
            expose_service: true,
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
    api.delete("chunking-engine", &DeleteParams::default())
        .await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete("chunking-engine", &DeleteParams::default())
        .await?;

    Ok(())
}
