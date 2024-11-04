use std::collections::BTreeMap;

use super::install::rand_hex;
use crate::error::Error;
use crate::operators::{BootstrapSpec, Cluster, ClusterSpec, InitDBSpec, SecretSpec, StorageSpec};
use k8s_openapi::api::core::v1::Secret;
use kube::api::{DeleteParams, ObjectMeta};
use kube::{
    api::{Api, PostParams},
    Client,
};

pub async fn deploy_app_database(
    client: &Client,
    namespace: &str,
    app_name: &str,
    insecure_override_passwords: &Option<String>,
    db_user_prefix: &Option<String>,
) -> Result<Option<String>, Error> {
    // If the cluster config exists, then do nothing.
    let cluster_name = format!("{}-db-cluster", app_name);
    let cluster_api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    let cluster = cluster_api.get(&cluster_name).await;
    if cluster.is_ok() {
        return Ok(None);
    }
    let app_database_password: String = insecure_override_passwords.clone().unwrap_or(rand_hex());
    let readonly_database_password: String =
        insecure_override_passwords.clone().unwrap_or(rand_hex());
    let dbowner_password: String = insecure_override_passwords.clone().unwrap_or(rand_hex());

    let db_user_prefix: String = db_user_prefix.clone().unwrap_or("".to_string());

    let cluster = Cluster {
        metadata: ObjectMeta {
            name: Some(cluster_name.clone()),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        spec: ClusterSpec {
            instances: 1,
            bootstrap: BootstrapSpec {
                initdb: InitDBSpec {
                    database: app_name.to_string(),
                    owner: "db-owner".to_string(),
                    secret: SecretSpec {
                        name: "db-owner".to_string(),
                    },
                    post_init_sql: Some(vec![
                        format!(
                            "CREATE ROLE {}application LOGIN ENCRYPTED PASSWORD '{}'",
                            db_user_prefix, app_database_password
                        ),
                        format!(
                            "CREATE ROLE {}readonly LOGIN ENCRYPTED PASSWORD '{}'",
                            db_user_prefix, readonly_database_password
                        ),
                    ]),
                    post_init_application_sql: Some(vec![
                        "CREATE EXTENSION IF NOT EXISTS vector".to_string()
                    ]),
                },
            },
            storage: StorageSpec {
                size: "1Gi".to_string(),
            },
        },
    };

    cluster_api.create(&PostParams::default(), &cluster).await?;

    let mut secret_data = BTreeMap::new();
    secret_data.insert(
        "migrations-url".to_string(),
        format!(
            "postgres://db-owner:{}@{}-rw:5432/{}?sslmode=disable",
            app_database_password, &cluster_name, app_name
        ),
    );
    secret_data.insert(
        "application-url".to_string(),
        format!(
            "postgres://application:{}@{}-rw:5432/{}?sslmode=disable",
            app_database_password, &cluster_name, app_name
        ),
    );
    secret_data.insert(
        "readonly-url".to_string(),
        format!(
            "postgres://readonly:{}@{}-rw:5432/{}?sslmode=disable",
            app_database_password, &cluster_name, app_name
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

    let secret_api: Api<Secret> = Api::namespaced(client.clone(), namespace);
    secret_api
        .create(&PostParams::default(), &dbowner_secret)
        .await?;
    secret_api
        .create(&PostParams::default(), &db_urls_secret)
        .await?;

    Ok(Some(readonly_database_password))
}

pub async fn _delete(client: Client, namespace: &str, app_name: &str) -> Result<(), Error> {
    // Remove deployments
    let cluster_name = format!("{}-db-cluster", app_name);

    let api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    if api.get(&cluster_name).await.is_ok() {
        api.delete(&cluster_name, &DeleteParams::default()).await?;
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
