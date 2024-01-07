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
    namespace: &str,
) -> Result<Deployment, Error> {
    // Create the Deployment object
    let deployment = serde_json::from_value(serde_json::json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": name,
            "labels": {
                "app": name
            }
        },
        "spec": {
            "replicas": replicas,
            "selector": {
                "matchLabels": {
                    "app": name
                }
            },
            "template": {
                "metadata": {
                    "labels": {
                        "app": name
                    }
                },
                "spec": {
                    "containers": [
                        {
                            "name": "example-container",
                            "image": "nginx:latest",
                            "ports": [
                                {
                                    "containerPort": 80
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
    replicas: i32,
    namespace: &str,
) -> Result<Service, Error> {
    // Create the Deployment object
    let service = serde_json::from_value(serde_json::json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": name,
            "labels": {
                "app": name
            }
        },
        "spec": {
            "replicas": replicas,
            "selector": {
                "matchLabels": {
                    "app": name
                }
            },
            "template": {
                "metadata": {
                    "labels": {
                        "app": name
                    }
                },
                "spec": {
                    "containers": [
                        {
                            "name": "example-container",
                            "image": "nginx:latest",
                            "ports": [
                                {
                                    "containerPort": 80
                                }
                            ]
                        }
                    ]
                }
            }
        }
    }))?;

    let service_api: Api<Service> = Api::namespaced(client, namespace);
    Ok(service_api.create(&PostParams::default(), &service).await?)
}
