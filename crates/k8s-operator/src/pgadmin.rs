use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};

const PGADMIN: &str = "pgadmin";

// Large Language Model
pub async fn deploy(
    client: Client,
    _name: &str,
    _spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: PGADMIN.to_string(),
            image_name: crate::PGADMIN_IMAGE.to_string(),
            replicas: 1,
            port: 80,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![],
            }),
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
    api.delete(PGADMIN, &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete(PGADMIN, &DeleteParams::default()).await?;

    Ok(())
}
