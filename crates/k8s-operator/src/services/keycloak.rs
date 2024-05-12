use std::collections::BTreeMap;

use super::deployment;
use crate::error::Error;
use crate::operator::crd::BionicSpec;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{ConfigMap, Secret, Service};
use kube::api::{DeleteParams, ObjectMeta, Patch, PatchParams, PostParams};
use kube::{Api, Client};
use serde_json::json;

const CONFIG_JSON: &str = include_str!("../../keycloak/realm.json");
pub const KEYCLOAK_NAME: &str = "keycloak";

// We are using envoy to add security headers to all responses from the main application.
pub async fn deploy(client: Client, spec: BionicSpec, namespace: &str) -> Result<(), Error> {
    // Put the envoy.yaml into a ConfigMap
    let config_map = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": KEYCLOAK_NAME,
            "namespace": namespace
        },
        "data": {
            "realm.json": CONFIG_JSON,
        }
    });

    let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    api.patch(
        KEYCLOAK_NAME,
        &PatchParams::apply(crate::MANAGER),
        &Patch::Apply(config_map),
    )
    .await?;

    // Keycloak
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: KEYCLOAK_NAME.to_string(),
            image_name: super::KEYCLOAK_IMAGE.to_string(),
            replicas: spec.replicas,
            port: 7910,
            env: vec![
                json!({
                    "name":
                    "KC_DB",
                    "value":
                    "postgres"
                }),
                json!({
                    "name":
                    "KC_DB_PASSWORD",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "keycloak-db-owner",
                            "key": "password"
                        }
                    }
                }),
                json!({
                    "name":
                    "KC_DB_USERNAME",
                    "value":
                    "keycloak-db-owner"
                }),
                json!({
                    "name":
                    "KC_DB_URL",
                    "value":
                    "jdbc:postgresql://keycloak-db-cluster-rw:5432/keycloak"
                }),
                json!({
                    "name":
                    "KEYCLOAK_ADMIN",
                    "value":
                    "admin"
                }),
                json!({
                    "name":
                    "KEYCLOAK_ADMIN_PASSWORD",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "keycloak-secrets",
                            "key": "admin-password"
                        }
                    }
                }),
                json!({
                    "name":
                    "KC_HEALTH_ENABLED",
                    "value":
                    "true"
                }),
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
                    format!("--hostname-url={}/oidc", spec.hostname_url),
                    "--http-relative-path=/oidc".to_string(),
                ],
            }),
            volume_mounts: vec![
                json!({"name": KEYCLOAK_NAME, "mountPath": "/opt/keycloak/data/import"}),
            ],
            volumes: vec![json!({"name": KEYCLOAK_NAME,
                "configMap": {
                    "name": KEYCLOAK_NAME
                }
            })],
        },
        namespace,
    )
    .await?;

    keycloak_secrets(namespace, client).await?;

    Ok(())
}

async fn keycloak_secrets(namespace: &str, client: Client) -> Result<(), Error> {
    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    let secret = secret_api.get("keycloak-secrets").await;
    if secret.is_err() {
        let mut secret_data = BTreeMap::new();
        secret_data.insert("admin-password".to_string(), super::database::rand_hex());
        let keycloak_secret = Secret {
            metadata: ObjectMeta {
                name: Some("keycloak-secrets".to_string()),
                namespace: Some(namespace.to_string()),
                ..ObjectMeta::default()
            },
            string_data: Some(secret_data),
            ..Default::default()
        };
        secret_api
            .create(&PostParams::default(), &keycloak_secret)
            .await?;
    }
    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get(KEYCLOAK_NAME).await.is_ok() {
        api.delete(KEYCLOAK_NAME, &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get(KEYCLOAK_NAME).await.is_ok() {
        api.delete(KEYCLOAK_NAME, &DeleteParams::default()).await?;
    }

    // Remove configmaps
    let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    if api.get(KEYCLOAK_NAME).await.is_ok() {
        api.delete(KEYCLOAK_NAME, &DeleteParams::default()).await?;
    }

    // Remove secrets
    let api: Api<Secret> = Api::namespaced(client, namespace);
    if api.get("keycloak-secrets").await.is_ok() {
        api.delete("keycloak-secrets", &DeleteParams::default())
            .await?;
    }

    Ok(())
}
