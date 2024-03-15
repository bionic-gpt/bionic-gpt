use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};

pub const TGI_NAME: &str = "tgi-zephyr-7b";

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
            name: TGI_NAME.to_string(),
            image_name: crate::TGI_IMAGE.to_string(),
            replicas: 1,
            port: 8000,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![
                    "--model-id".into(),
                    "TheBloke/zephyr-7B-beta-AWQ".into(),
                    "--max-batch-prefill-tokens".into(),
                    "2048".into(),
                    "--quantize".into(),
                    "gptq".into(),
                ],
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
    api.delete(TGI_NAME, &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete(TGI_NAME, &DeleteParams::default()).await?;

    Ok(())
}
