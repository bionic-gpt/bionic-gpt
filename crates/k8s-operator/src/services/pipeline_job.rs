use super::deployment;
use crate::error::Error;
use crate::operator::crd::BionicSpec;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

// Large Language Model
pub async fn deploy(client: Client, spec: BionicSpec, namespace: &str) -> Result<(), Error> {
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "pipeline-job".to_string(),
            image_name: format!("{}:{}", super::BIONICGPT_PIPELINE_JOB_IMAGE, spec.version),
            replicas: spec.replicas,
            port: 3000,
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
                    "CHUNKING_ENGINE",
                    "value":
                    "http://chunking-engine:8000"
                }),
            ],
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

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete("pipeline-job", &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete("pipeline-job", &DeleteParams::default()).await?;

    Ok(())
}
