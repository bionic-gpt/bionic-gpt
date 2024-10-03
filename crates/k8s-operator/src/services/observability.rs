use crate::cli::apply;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{api::DeleteParams, Api, Client};

const GRAFANA_YAML: &str = include_str!("../../config/grafana.yaml");

// Large Language Model
pub async fn deploy(
    client: Client,
    password: Option<String>,
    _namespace: &str,
) -> Result<(), Error> {
    // If we have the passwords then extract them.
    let password = if let Some(password) = password {
        password
    } else {
        "".to_string()
    };

    let yaml = GRAFANA_YAML.replace("$BIONIC_PASSWORD", &password);

    apply::apply(&client, &yaml, None).await.unwrap();

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get(GRAFANA_YAML).await.is_ok() {
        api.delete(GRAFANA_YAML, &DeleteParams::default()).await?;
    }

    Ok(())
}
