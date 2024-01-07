use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::{
    api::{Api, PostParams},
    Client,
};

/// Create a deployment and a service.
/// Include sidecars if needed.
pub async fn _deployment(
    client: Client,
    name: &str,
    replicas: i32,
    image_name: String,
    port: u16,
    namespace: &str,
) -> Result<Deployment, Error> {
    /***let init_container = serde_json::json!({
        "name": "init-container",
        "image": "busybox:latest",
        "command": ["sh", "-c", "echo Initializing... && sleep 10"]
    });**/

    let app_labels = serde_json::json!({
        "app": name,
        "component": name
    });

    // Create the Deployment object
    let deployment = serde_json::from_value(serde_json::json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": name,
            "labels": app_labels,
            "namespace": namespace
        },
        "spec": {
            "replicas": replicas,
            "selector": {
                "matchLabels": app_labels
            },
            "template": {
                "metadata": {
                    "labels": app_labels
                },
                "spec": {
                    //"initContainers": [init_container],
                    "containers": [
                        {
                            "name": name,
                            "image": image_name,
                            "ports": [
                                {
                                    "containerPort": port
                                }
                            ]
                        }
                    ]
                }
            }
        }
    }))?;

    // Create the deployment defined above
    let deployment_api: Api<Deployment> = Api::namespaced(client, namespace);
    Ok(deployment_api
        .create(&PostParams::default(), &deployment)
        .await?)
}

pub async fn _service(
    client: Client,
    name: &str,
    port_number: u16,
    namespace: &str,
) -> Result<Service, Error> {
    // Create the Deployment object
    let service = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "Service",
        "metadata": {
            "name": name,
            "namespace": namespace
        },
        "spec": {
            "selector": {
                "app": name
            },
            "ports": [
                {
                    "protocol": "TCP",
                    "port": port_number,
                    "targetPort": port_number
                }
            ]
        }
    }))?;

    let service_api: Api<Service> = Api::namespaced(client, namespace);
    Ok(service_api.create(&PostParams::default(), &service).await?)
}
