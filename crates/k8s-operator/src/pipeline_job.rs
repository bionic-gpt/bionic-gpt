use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

// Large Language Model
pub async fn deploy(
    client: Client,
    _name: &str,
    spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "pipeline-job".to_string(),
            image_name: format!("{}:{}", crate::BIONICGPT_PIPELINE_JOB_IMAGE, spec.version),
            replicas: spec.replicas,
            port: 3000,
            env: vec![json!({
                "name": 
                "APP_DATABASE_URL", 
                "value": 
                "postgresql://bionic_application:testpassword@postgres:5432/bionic-gpt?sslmode=disable"
            })],
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
    api.delete("pipeline-job", &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete("pipeline-job", &DeleteParams::default()).await?;

    Ok(())
}
