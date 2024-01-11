use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{ConfigMap, Service};
use kube::api::{DeleteParams, PostParams};
use kube::{Api, Client};
use serde_json::json;

const REALM_JSON: &str = include_str!("../keycloak/realm.json");

pub async fn deploy(
    client: Client,
    _name: &str,
    spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    // Put the real.json into a ConfigMap
    let config_map = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": "keycloak",
            "namespace": namespace
        },
        "data": {
            "realm.json": REALM_JSON,
        }
    }))?;

    let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    api.create(&PostParams::default(), &config_map).await?;

    // Keycloak
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "keycloak".to_string(),
            image_name: crate::KEYCLOAK_IMAGE.to_string(),
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
            command: Some(deployment::Command {
                command: vec![],
                args: vec![
                    "start-dev".to_string(),
                    "--import-realm".to_string(),
                    "--http-port=7910".to_string(),
                    "--proxy=edge".to_string(),
                    "--hostname-strict=false".to_string(),
                    "--hostname-strict-https=false".to_string(),
                    "--http-relative-path=/oidc".to_string(),
                    format!("--hostname-url={}/oidc", spec.hostname_url),
                ],
            }),
            expose_service: true,
            volume_mounts: vec![
                json!({"name": "keycloak-config", "mountPath": "/opt/keycloak/data/import"}),
            ],
            volumes: vec![json!({"name": "keycloak-config",
                "configMap": {
                    "name": "keycloak"
                }
            })],
        },
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn delete(client: Client, _name: &str, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete("keycloak", &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete("keycloak", &DeleteParams::default()).await?;

    // Remove configmaps
    let api: Api<ConfigMap> = Api::namespaced(client, namespace);
    api.delete("keycloak", &DeleteParams::default()).await?;

    Ok(())
}
