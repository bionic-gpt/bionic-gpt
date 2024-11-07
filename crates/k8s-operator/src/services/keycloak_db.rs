use std::collections::BTreeMap;

use super::database::{
    rand_hex, BootstrapSpec, Cluster, ClusterSpec, InitDBSpec, SecretSpec, StorageSpec,
};
use crate::error::Error;
use k8s_openapi::api::core::v1::Secret;
use kube::api::{DeleteParams, ObjectMeta};
use kube::{
    api::{Api, PostParams},
    Client,
};

pub async fn deploy(
    client: Client,
    namespace: &str,
    disk_size: i32,
) -> Result<Option<String>, Error> {
    // If the cluster is already created then leave it alone.
    let cluster_api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    let cluster = cluster_api.get("keycloak-db-cluster").await;
    if cluster.is_ok() {
        return Ok(None);
    }

    let database_password: String = rand_hex();

    let cluster = Cluster {
        metadata: ObjectMeta {
            name: Some("keycloak-db-cluster".to_string()),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        spec: ClusterSpec {
            instances: 1,
            bootstrap: BootstrapSpec {
                initdb: InitDBSpec {
                    database: "keycloak".to_string(),
                    owner: "keycloak-db-owner".to_string(),
                    secret: SecretSpec {
                        name: "keycloak-db-owner".to_string(),
                    },
                    post_init_sql: None,
                    post_init_application_sql: None,
                },
            },
            storage: StorageSpec {
                size: format!("{}Gi", disk_size),
            },
        },
    };
    cluster_api.create(&PostParams::default(), &cluster).await?;

    let mut secret_data = BTreeMap::new();
    secret_data.insert("username".to_string(), "keycloak-db-owner".to_string());
    secret_data.insert("password".to_string(), database_password.clone());

    let dbowner_secret = Secret {
        metadata: ObjectMeta {
            name: Some("keycloak-db-owner".to_string()),
            namespace: Some(namespace.to_string()),
            ..ObjectMeta::default()
        },
        string_data: Some(secret_data),
        ..Default::default()
    };

    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    secret_api
        .create(&PostParams::default(), &dbowner_secret)
        .await?;

    Ok(Some(database_password))
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    if api.get("keycloak-db-cluster").await.is_ok() {
        api.delete("keycloak-db-cluster", &DeleteParams::default())
            .await?;
    }

    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    if api.get("keycloak-db-owner").await.is_ok() {
        secret_api
            .delete("keycloak-db-owner", &DeleteParams::default())
            .await?;
    }

    Ok(())
}
