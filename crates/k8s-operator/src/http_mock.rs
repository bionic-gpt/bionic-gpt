use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::api::PostParams;
use kube::{Api, Client};
use serde_json::json;

const EMBEDDINGS_YAML: &str = include_str!("../config/mocks/embeddings.mock.yaml");
const OPENAI_YAML: &str = include_str!("../config/mocks/openai.mock.yaml");
const UNSTRUCTURED_YAML: &str = include_str!("../config/mocks/unstructured.mock.yaml");

// We are using envoy to add security headers to all responses from the main application.
pub async fn deploy(client: Client, name: &str, port: u16, namespace: &str) -> Result<(), Error> {
    // Put the envoy.yaml into a ConfigMap
    let config_map = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": name,
            "namespace": namespace
        },
        "data": {
            "embeddings.mock.yaml": EMBEDDINGS_YAML,
            "openai.mock.yaml": OPENAI_YAML,
            "unstructured.mock.yaml": UNSTRUCTURED_YAML,
        }
    }))?;

    let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    api.create(&PostParams::default(), &config_map).await?;

    // Envoy
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: name.to_string(),
            replicas: 1,
            image_name: crate::HTTP_MOCK.to_string(),
            port,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![
                    "/usr/local/bin/envoy".to_string(),
                    "-c".to_string(),
                    "/etc/envoy/envoy.yaml".to_string(),
                    "--service-cluster".to_string(),
                    "envoy".to_string(),
                    "--service-node".to_string(),
                    "envoy".to_string(),
                    "--log-level".to_string(),
                    "info".to_string(),
                ],
            }),
            volume_mounts: vec![json!({"name": "envoy-config", "mountPath": "/etc/envoy/"})],
            volumes: vec![json!({"name": "envoy-config",
                "configMap": {
                    "name": "envoy"
                }
            })],
        },
        namespace,
    )
    .await?;

    Ok(())
}
