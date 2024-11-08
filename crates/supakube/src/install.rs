use crate::database::deploy_app_database;
use crate::keycloak::deploy_keycloak;
use crate::keycloak_db::deploy_keycloak_database;
use crate::nginx::deploy_nginx;
use crate::oauth2_proxy::deploy_oauth2_proxy;
use crate::operators::install_postgres_operator;
use crate::{application::deploy_application, selenium::deploy_selenium};
use anyhow::Result;
use k8s_openapi::api::core::v1::Namespace;
use kube::{
    api::{ObjectMeta, PostParams},
    Api, Client, Error,
};
use rand::Rng;

const POSTGRES_SERVICE: &str = include_str!("../config/postgres-service.yaml");
const APPLICATION_SERVICE: &str = include_str!("../config/nginx-service.yaml");
const SELENIUM_SERVICE: &str = include_str!("../config/selenium-service.yaml");

pub async fn install(installer: &crate::Installer) -> Result<()> {
    println!("🔗 Connecting to the cluster...");
    let client = Client::try_default().await?;
    println!("🔗 Connected");

    println!("🔧 Creating Namespace : {}", &installer.namespace);
    create_namespace(&client, &installer.namespace).await?;

    install_postgres_operator(&client).await?;

    println!("⛃ Deploying application database");
    deploy_app_database(
        &client,
        &installer.namespace,
        &installer.app_name,
        &installer.insecure_override_passwords,
        &installer.db_user_prefix,
    )
    .await?;

    println!("⛃ Deploying keycloak database");
    deploy_keycloak_database(&client, &installer.namespace).await?;
    println!("🔧 Deploying keycloak");
    deploy_keycloak(&client, installer, &installer.namespace).await?;
    println!("🔧 Deploying Oauth2 Proxy");
    deploy_oauth2_proxy(&client, installer, &installer.namespace).await?;
    println!("🔧 Deploying the application");
    deploy_application(&client, installer, &installer.namespace).await?;
    println!("🔧 Deploying Nginx");
    deploy_nginx(&client, &installer.namespace).await?;
    println!("🔧 Deploying Selenium");
    deploy_selenium(&client, &installer.namespace).await?;

    if installer.development {
        println!("🚀 Mapping Postgres to port 30001");
        super::apply::apply(&client, POSTGRES_SERVICE, Some(&installer.namespace)).await?;
        println!("🚀 Mapping Nginx to port 30000");
        super::apply::apply(&client, APPLICATION_SERVICE, Some(&installer.namespace)).await?;
        println!("🚀 Mapping selenium to port 30002");
        super::apply::apply(&client, SELENIUM_SERVICE, Some(&installer.namespace)).await?;
    }

    Ok(())
}

pub async fn create_namespace(client: &Client, namespace: &str) -> Result<Namespace> {
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

pub fn rand_hex() -> String {
    let mut rng = rand::thread_rng();
    (0..5).map(|_| rng.gen::<u8>().to_string()).collect()
}
