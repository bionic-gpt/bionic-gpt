use crate::database::deploy_app_database;
use crate::operators::install_postgres_operator;
use anyhow::Result;
use k8s_openapi::api::core::v1::Namespace;
use kube::{
    api::{ObjectMeta, PostParams},
    Api, Client, Error,
};

const POSTGRES_SERVICE: &str = include_str!("../config/postgres-service.yaml");

pub async fn install(installer: &crate::Installer) -> Result<()> {
    tracing::info!("Connecting to the cluster...");
    let client = Client::try_default().await?;
    tracing::info!("Connected");

    create_namespace(&client, &installer.namespace).await?;

    install_postgres_operator(&client).await?;

    deploy_app_database(
        &client,
        &installer.namespace,
        &installer.app_name,
        &installer.insecure_override_passwords,
        &installer.db_user_prefix,
    )
    .await?;

    if installer.development {
        super::apply::apply(&client, POSTGRES_SERVICE, Some(&installer.namespace)).await?;
    }

    Ok(())
}

async fn create_namespace(client: &Client, namespace: &str) -> Result<Namespace> {
    tracing::info!("Ensuring existence of namespace {}", namespace);
    // Define the API object for Namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());

    // Check if the namespace already exists
    match namespaces.get(namespace).await {
        Ok(existing_ns) => {
            tracing::info!("Namespace {} already exists", namespace);
            Ok(existing_ns)
        }
        Err(Error::Api(err)) if err.code == 404 => {
            tracing::info!("Namespace {} not found, creating", namespace);

            let new_namespace = Namespace {
                metadata: ObjectMeta {
                    name: Some(namespace.to_string()),
                    ..Default::default()
                },
                ..Default::default()
            };

            let ns = namespaces
                .create(&PostParams::default(), &new_namespace)
                .await?;
            tracing::info!("Namespace {} created", namespace);
            Ok(ns)
        }
        Err(e) => {
            tracing::error!(
                "Failed to check existence of namespace {}: {:?}",
                namespace,
                e
            );
            Err(e.into())
        }
    }
}
