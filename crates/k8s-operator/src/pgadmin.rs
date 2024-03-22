use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{ConfigMap, Secret, Service};
use kube::api::{DeleteParams, PostParams};
use kube::{Api, Client};
use serde_json::json;

const PGADMIN: &str = "pgadmin";
const CONFIG_JSON: &str = include_str!("../config/servers.json");

// Large Language Model
pub async fn deploy(
    client: Client,
    password: &str,
    keycloak_password: &str,
    namespace: &str,
) -> Result<(), Error> {
    let passfile = format!("bionic-db-cluster-rw:5432:*:bionic_readonly:{}", password);
    let keycloak_passfile = format!(
        "keycloak-db-cluster-rw:5432:*:keycloak-db-owner:{}",
        keycloak_password
    );
    // Put the envoy.yaml into a ConfigMap
    let config_map = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": PGADMIN,
            "namespace": namespace
        },
        "data": {
            "servers.json": CONFIG_JSON,
            "passfile": &passfile,
            "passfile_keycloak": &keycloak_passfile,
        }
    }))?;

    let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    api.create(&PostParams::default(), &config_map).await?;

    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: PGADMIN.to_string(),
            image_name: crate::PGADMIN_IMAGE.to_string(),
            replicas: 1,
            port: 80,
            env: vec![
                json!({
                    "name":
                    "PGADMIN_DEFAULT_EMAIL",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "pgadmin",
                            "key": "email"
                        }
                    }
                }),
                json!({
                    "name":
                    "PGADMIN_DEFAULT_PASSWORD",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "pgadmin",
                            "key": "password"
                        }
                    }
                }),
                json!({
                    "name":
                    "SCRIPT_NAME",
                    "value":
                    "/pgadmin"
                }),
            ],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![],
            }),
            volume_mounts: vec![
                json!(
                {
                    "name": PGADMIN,
                    "mountPath": "/pgadmin4/servers.json",
                    "subPath": "servers.json"
                }),
                json!({
                    "name": PGADMIN,
                    "mountPath": "/pgadmin4/passfile",
                    "subPath": "passfile"
                }),
            ],
            volumes: vec![json!({"name": PGADMIN,
                "configMap": {
                    "name": PGADMIN
                }
            })],
        },
        namespace,
    )
    .await?;

    let secret = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "Secret",
        "metadata": {
            "name": PGADMIN,
            "namespace": namespace
        },
        "stringData": {
            "email": "pgadmin@pgadmin.com",
            "password": crate::database::rand_hex()
        }
    }))?;

    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    secret_api.create(&PostParams::default(), &secret).await?;

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete(PGADMIN, &DeleteParams::default()).await?;

    let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    api.delete(PGADMIN, &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete(PGADMIN, &DeleteParams::default()).await?;

    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    secret_api.delete(PGADMIN, &DeleteParams::default()).await?;

    Ok(())
}
