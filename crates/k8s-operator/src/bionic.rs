use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use kube::api::DeleteParams;
use kube::{Api, Client};

/// Creates a new deployment of `n` pods with the `inanimate/echo-server:latest` docker image inside,
/// where `n` is the number of `replicas` given.
///
/// # Arguments
/// - `client` - A Kubernetes client to create the deployment with.
/// - `name` - Name of the deployment to be created
/// - `replicas` - Number of pod replicas for the Deployment to contain
/// - `namespace` - Namespace to create the Kubernetes Deployment in.
///
/// Note: It is assumed the resource does not already exists for simplicity. Returns an `Error` if it does.
pub async fn deploy(
    client: Client,
    name: &str,
    replicas: i32,
    namespace: &str,
) -> Result<(), Error> {
    deployment::deployment(
        client,
        name,
        "nginx",
        replicas,
        "nginx:latest",
        80,
        namespace,
    )
    .await?;

    Ok(())
}

/// Deletes an existing deployment.
///
/// # Arguments:
/// - `client` - A Kubernetes client to delete the Deployment with
/// - `name` - Name of the deployment to delete
/// - `namespace` - Namespace the existing deployment resides in
///
/// Note: It is assumed the deployment exists for simplicity. Otherwise returns an Error.
pub async fn delete(client: Client, name: &str, namespace: &str) -> Result<(), Error> {
    let api: Api<Deployment> = Api::namespaced(client, namespace);
    let name = format!("{name}-nginx");
    api.delete(&name, &DeleteParams::default()).await?;
    Ok(())
}
