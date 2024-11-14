use std::collections::BTreeMap;

use crate::error::Error;
use k8s_openapi::api::core::v1::Secret;
use kube::api::{DeleteParams, ObjectMeta};
use kube::CustomResource;
use kube::{
    api::{Api, PostParams},
    Client,
};
use rand::Rng;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct BootstrapSpec {
    pub initdb: InitDBSpec,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct InitDBSpec {
    pub database: String,
    pub owner: String,
    pub secret: SecretSpec,
    #[serde(rename = "postInitSQL")]
    pub post_init_sql: Option<Vec<String>>,
    #[serde(rename = "postInitApplicationSQL")]
    pub post_init_application_sql: Option<Vec<String>>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SecretSpec {
    pub name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct StorageSpec {
    pub size: String,
}

/// Corresponds to the Cluster resource
#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "postgresql.cnpg.io",
    version = "v1",
    kind = "Cluster",
    plural = "clusters",
    derive = "PartialEq",
    namespaced
)]
pub struct ClusterSpec {
    pub instances: i32,
    pub bootstrap: BootstrapSpec,
    pub storage: StorageSpec,
}

pub async fn deploy(
    client: Client,
    namespace: &str,
    disk_size: i32,
    insecure_override_passwords: &Option<String>,
) -> Result<Option<String>, Error> {
    // If the cluster config exists, then do nothing.
    let cluster_api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    let cluster = cluster_api.get("bionic-db-cluster").await;
    if cluster.is_ok() {
        return Ok(None);
    }
    let app_database_password: String = insecure_override_passwords.clone().unwrap_or(rand_hex());
    let readonly_database_password: String =
        insecure_override_passwords.clone().unwrap_or(rand_hex());
    let dbowner_password: String = insecure_override_passwords.clone().unwrap_or(rand_hex());

    let cluster = Cluster {
        metadata: ObjectMeta {
            name: Some("bionic-db-cluster".to_string()),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        spec: ClusterSpec {
            instances: 1,
            bootstrap: BootstrapSpec {
                initdb: InitDBSpec {
                    database: "bionic-gpt".to_string(),
                    owner: "db-owner".to_string(),
                    secret: SecretSpec {
                        name: "db-owner".to_string(),
                    },
                    post_init_sql: Some(vec![
                        format!(
                            "CREATE ROLE bionic_application LOGIN ENCRYPTED PASSWORD '{}'",
                            app_database_password
                        ),
                        format!(
                            "CREATE ROLE bionic_readonly LOGIN ENCRYPTED PASSWORD '{}'",
                            readonly_database_password
                        ),
                    ]),
                    post_init_application_sql: Some(vec![
                        "CREATE EXTENSION IF NOT EXISTS vector".to_string()
                    ]),
                },
            },
            storage: StorageSpec {
                size: format!("{}Gi", disk_size),
            },
        },
    };

    cluster_api.create(&PostParams::default(), &cluster).await?;

    let mut secret_data = BTreeMap::new();
    secret_data.insert(
        "migrations-url".to_string(),
        format!(
            "postgres://db-owner:{}@bionic-db-cluster-rw:5432/bionic-gpt?sslmode=disable",
            dbowner_password
        ),
    );
    secret_data.insert(
        "application-url".to_string(),
        format!(
            "postgres://bionic_application:{}@bionic-db-cluster-rw:5432/bionic-gpt?sslmode=disable",
            app_database_password
        ),
    );
    secret_data.insert(
        "readonly-url".to_string(),
        format!(
            "postgres://bionic_readonly:{}@bionic-db-cluster-rw:5432/bionic-gpt?sslmode=disable",
            readonly_database_password
        ),
    );

    let db_urls_secret = Secret {
        metadata: ObjectMeta {
            name: Some("database-urls".to_string()),
            namespace: Some(namespace.to_string()),
            ..ObjectMeta::default()
        },
        string_data: Some(secret_data),
        ..Default::default()
    };

    let mut secret_data = BTreeMap::new();
    secret_data.insert("username".to_string(), "db-owner".to_string());
    secret_data.insert("password".to_string(), dbowner_password);

    let dbowner_secret = Secret {
        metadata: ObjectMeta {
            name: Some("db-owner".to_string()),
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
    secret_api
        .create(&PostParams::default(), &db_urls_secret)
        .await?;

    Ok(Some(readonly_database_password))
}

pub fn rand_hex() -> String {
    let mut rng = rand::thread_rng();
    (0..5).map(|_| rng.gen::<u8>().to_string()).collect()
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    if api.get("bionic-db-cluster").await.is_ok() {
        api.delete("bionic-db-cluster", &DeleteParams::default())
            .await?;
    }

    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    if api.get("database-urls").await.is_ok() {
        secret_api
            .delete("database-urls", &DeleteParams::default())
            .await?;
    }
    if api.get("db-owner").await.is_ok() {
        secret_api
            .delete("db-owner", &DeleteParams::default())
            .await?;
    }

    Ok(())
}
