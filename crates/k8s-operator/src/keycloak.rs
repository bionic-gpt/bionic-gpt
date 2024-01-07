use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

pub async fn _deploy(
    client: Client,
    _name: &str,
    spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    // Keycloak
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "keycloak".to_string(),
            image_name: spec.keycloak_image,
            replicas: spec.replicas,
            port: 7910,
            env: vec![
                json!({"name": "KC_DB", "value": "postgres"}),
                json!({"name": "KC_DB_PASSWORD", "value": "testpassword"}),
                json!({"name": "KC_DB_USERNAME", "value": "postgres"}),
                json!({"name": "KC_DB_URL", "value": "jdbc:postgresql://postgres/keycloak"}),
                json!({"name": "KEYCLOAK_ADMIN", "value": "admin"}),
                json!({"name": "KEYCLOAK_ADMIN_PASSWORD", "value": "Pa55w0rd"}),
                json!({"name": "KC_HEALTH_ENABLED", "value": "true"}),
            ],
            init_container: None,
            command: Some(vec![
                "start-dev".to_string(),
                "--import-realm".to_string(),
                "--http-port=7910".to_string(),
                "--proxy=edge".to_string(),
                "--hostname=localhost:7910".to_string(),
            ]),
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
pub async fn _delete(client: Client, _name: &str, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete("keycloak", &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client, namespace);
    api.delete("keycloak", &DeleteParams::default()).await?;
    Ok(())
}
