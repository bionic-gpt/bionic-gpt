use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::{Patch, PatchParams};
use kube::{api::Api, Client};
use serde_json::{json, Value};

pub struct Command {
    pub command: Vec<String>,
    pub args: Vec<String>,
}

pub struct InitContainer {
    pub image_name: String,
    pub env: Vec<Value>,
}

pub struct ServiceDeployment {
    pub name: String,
    pub replicas: i32,
    pub image_name: String,
    pub port: u16,
    pub env: Vec<Value>,
    pub init_container: Option<InitContainer>,
    pub command: Option<Command>,
    pub volume_mounts: Vec<Value>,
    pub volumes: Vec<Value>,
}

/// Create a deployment and a service.
/// Include sidecars if needed.
pub async fn deployment(
    client: Client,
    service_deployment: ServiceDeployment,
    namespace: &str,
) -> Result<(), Error> {
    let app_labels = serde_json::json!({
        "app": service_deployment.name,
        "component": service_deployment.name
    });

    let init_containers: Vec<Value> =
        if let Some(init_container) = service_deployment.init_container {
            vec![json!({
                "name": "init",
                "image": init_container.image_name,
                "imagePullPolicy": "IfNotPresent",
                "env": init_container.env
            })]
        } else {
            vec![]
        };

    let containers = if let Some(command) = service_deployment.command {
        json!([{
            "name": service_deployment.name,
            "image": service_deployment.image_name,
            "imagePullPolicy": "IfNotPresent",
            "ports": [{
                "containerPort": service_deployment.port
            }],
            "env": service_deployment.env,
            "volumeMounts": service_deployment.volume_mounts,
            "command": command.command,
            "args": command.args
        }])
    } else {
        json!([{
            "name": service_deployment.name,
            "image": service_deployment.image_name,
            "ports": [{
                "containerPort": service_deployment.port
            }],
            "env": service_deployment.env,
            "volumeMounts": service_deployment.volume_mounts,
        }])
    };

    // Create the Deployment object
    let deployment = serde_json::json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": service_deployment.name,
            "labels": app_labels,
            "namespace": namespace
        },
        "spec": {
            "replicas": service_deployment.replicas,
            "selector": {
                "matchLabels": app_labels
            },
            "template": {
                "metadata": {
                    "labels": app_labels
                },
                "spec": {
                    "initContainers": init_containers,
                    "containers": containers,
                    "volumes": service_deployment.volumes,
                }
            }
        }
    });

    // Create the deployment defined above
    let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    let _deployment = deployment_api
        .patch(
            &service_deployment.name,
            &PatchParams::apply(crate::MANAGER),
            &Patch::Apply(deployment),
        )
        .await?;

    // If this is an update to an existing deployment then we want it to rollout
    // see https://github.com/kube-rs/kube/pull/635
    deployment_api.restart(&service_deployment.name).await?;

    service(
        client,
        &service_deployment.name,
        service_deployment.port,
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn service(
    client: Client,
    name: &str,
    port_number: u16,
    namespace: &str,
) -> Result<Service, Error> {
    // Create the Deployment object

    let service = serde_json::json!({
        "apiVersion": "v1",
        "kind": "Service",
        "metadata": {
            "name": name,
            "namespace": namespace
        },
        "spec": {
            "type": "ClusterIP",
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
    });

    let service_api: Api<Service> = Api::namespaced(client, namespace);
    let service = service_api
        .patch(
            name,
            &PatchParams::apply(crate::MANAGER),
            &Patch::Apply(service),
        )
        .await?;
    Ok(service)
}
