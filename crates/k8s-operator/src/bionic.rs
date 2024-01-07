use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

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
    _name: &str,
    spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    // First Postgres with pgVector
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "postgres".to_string(),
            image_name: "ankane/pgvector".to_string(),
            replicas: spec.replicas,
            port: 5432,
            env: vec![
                json!({"name": "POSTGRES_PASSWORD", "value": "testpassword"}),
                json!({"name": "POSTGRES_USER", "value": "postgres"}),
                json!({"name": "POSTGRES_DB", "value": "keycloak"}),
            ],
            init_container: None,
            command: None,
        },
        namespace,
    )
    .await?;

    // Bionic with the migrations as a sidecar
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "bionic-gpt".to_string(),
            image_name: "ghcr.io/bionic-gpt/bionicgpt:1.5.12".to_string(),
            replicas: spec.replicas,
            port: 7703,
            env: vec![json!({
                "name": 
                "APP_DATABASE_URL", 
                "value": 
                "postgresql://bionic_application:testpassword@postgres:5432/bionic-gpt?sslmode=disable"
            })],
            init_container: Some(deployment::InitContainer {
                image_name: spec.bionicgpt_db_migrations_image,
                env: vec![json!({
                    "name": 
                    "DATABASE_URL", 
                    "value": 
                    "postgresql://postgres:testpassword@postgres:5432/bionic-gpt?sslmode=disable"
                })]
            }),
            command: None
        },
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
pub async fn delete(client: Client, _name: &str, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete("postgres", &DeleteParams::default()).await?;
    api.delete("bionic-gpt", &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client, namespace);
    api.delete("postgres", &DeleteParams::default()).await?;
    api.delete("bionic-gpt", &DeleteParams::default()).await?;
    Ok(())
}
