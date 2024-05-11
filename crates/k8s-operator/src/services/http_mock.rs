use super::deployment;
use crate::error::Error;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::api::{Patch, PatchParams};
use kube::{Api, Client};
use serde_json::json;

const EMBEDDINGS_YAML: &str = include_str!("../../config/mocks/embeddings.mock.yaml");
const OPENAI_YAML: &str = include_str!("../../config/mocks/openai.mock.yaml");
const UNSTRUCTURED_YAML: &str = include_str!("../../config/mocks/unstructured.mock.yaml");
const OPENAI_MODEL_YAML: &str = include_str!("../../config/mocks/models.mock.yaml");

// We are using envoy to add security headers to all responses from the main application.
pub async fn deploy(client: Client, name: &str, port: u16, namespace: &str) -> Result<(), Error> {
    // Put the envoy.yaml into a ConfigMap
    let config_map = serde_json::json!({
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
            "modes.mock.yaml": OPENAI_MODEL_YAML,
        }
    });

    let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    api.patch(
        name,
        &PatchParams::apply(crate::MANAGER),
        &Patch::Apply(config_map),
    )
    .await?;

    // Envoy
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: name.to_string(),
            replicas: 1,
            image_name: super::HTTP_MOCK.to_string(),
            port,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec!["httpmock".into()],
                args: vec![
                    "--expose".to_string(),
                    "-m".to_string(),
                    "/mocks".to_string(),
                    "-p".to_string(),
                    format!("{}", port),
                ],
            }),
            volume_mounts: vec![json!({"name": format!("{}-config", name), "mountPath": "/mocks"})],
            volumes: vec![json!({"name": format!("{}-config", name),
                "configMap": {
                    "name": name
                }
            })],
        },
        namespace,
    )
    .await?;

    Ok(())
}
