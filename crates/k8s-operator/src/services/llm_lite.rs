use super::deployment;
use super::tgi::{MODEL_NAME, MODEL_REPOSITORY};
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};

const LITE_LLM: &str = "llm-lite";

// Large Language Model
pub async fn deploy(client: Client, namespace: &str) -> Result<(), Error> {
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: LITE_LLM.to_string(),
            image_name: super::LITE_LLM_IMAGE.to_string(),
            replicas: 1,
            port: 11434,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![
                    "--model".into(),
                    format!("huggingface/{}", MODEL_REPOSITORY),
                    "--api_base".into(),
                    format!("http://{}/generate_stream", MODEL_NAME),
                    "--host".into(),
                    "0.0.0.0".into(),
                    "--port".into(),
                    "11434".into(),
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

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get(LITE_LLM).await.is_ok() {
        api.delete(LITE_LLM, &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get(LITE_LLM).await.is_ok() {
        api.delete(LITE_LLM, &DeleteParams::default()).await?;
    }

    Ok(())
}
