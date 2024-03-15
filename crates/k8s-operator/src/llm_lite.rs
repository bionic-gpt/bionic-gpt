use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use crate::tgi::TGI_NAME;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};

const LITE_LLM: &str = "llm-lite";

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
            name: LITE_LLM.to_string(),
            image_name: crate::LITE_LLM_IMAGE.to_string(),
            replicas: 1,
            port: 8000,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![
                    "--model".into(),
                    "huggingface/TheBloke/zephyr-7B-beta-AWQ".into(),
                    "--api_base".into(),
                    format!("http://{}/generate_stream", TGI_NAME),
                    "--host".into(),
                    "0.0.0.0".into(),
                    "--port".into(),
                    "3000".into(),
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
    api.delete(LITE_LLM, &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete(LITE_LLM, &DeleteParams::default()).await?;

    Ok(())
}
