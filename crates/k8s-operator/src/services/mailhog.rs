use crate::cli::apply;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{api::DeleteParams, Api, Client};

const MAILHOG_YAML: &str = include_str!("../../config/mailhog.yaml");

// Large Language Model
pub async fn deploy(client: Client, namespace: &str) -> Result<(), Error> {
    apply::apply(&client, MAILHOG_YAML, Some(namespace))
        .await
        .unwrap();

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get(MAILHOG_YAML).await.is_ok() {
        api.delete(MAILHOG_YAML, &DeleteParams::default()).await?;
    }

    Ok(())
}
