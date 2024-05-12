use super::deployment;
use crate::error::Error;
use crate::operator::crd::BionicSpec;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{ConfigMap, Service};
use kube::api::{DeleteParams, Patch, PatchParams};
use kube::{Api, Client};
use serde_json::json;

const ENVOY_YAML: &str = include_str!("../../envoy/envoy.yaml");

// We are using envoy to add security headers to all responses from the main application.
pub async fn deploy(client: Client, spec: BionicSpec, namespace: &str) -> Result<(), Error> {
    // Put the envoy.yaml into a ConfigMap
    let config_map = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": "envoy",
            "namespace": namespace
        },
        "data": {
            "envoy.yaml": ENVOY_YAML,
        }
    });

    let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    api.patch(
        "envoy",
        &PatchParams::apply(crate::MANAGER),
        &Patch::Apply(config_map),
    )
    .await?;

    // Envoy
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "envoy".to_string(),
            image_name: super::ENVOYPROXY_IMAGE.to_string(),
            replicas: spec.replicas,
            port: 7901,
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

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get("envoy").await.is_ok() {
        api.delete("envoy", &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get("envoy").await.is_ok() {
        api.delete("envoy", &DeleteParams::default()).await?;
    }

    // Remove configmaps
    let api: Api<ConfigMap> = Api::namespaced(client, namespace);
    if api.get("envoy").await.is_ok() {
        api.delete("envoy", &DeleteParams::default()).await?;
    }

    Ok(())
}
