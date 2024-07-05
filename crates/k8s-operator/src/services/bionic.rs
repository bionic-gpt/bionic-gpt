use std::collections::BTreeMap;

use crate::error::Error;
use crate::operator::crd::BionicSpec;
use crate::services::deployment;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{Secret, Service};
use kube::api::{DeleteParams, ObjectMeta, PostParams};
use kube::{Api, Client};
use serde_json::json;

// Some constants so we don't get typos.
static INVITE_DOMAIN: &str = "INVITE_DOMAIN";
static INVITE_FROM_EMAIL_ADDRESS: &str = "INVITE_FROM_EMAIL_ADDRESS";
static SMTP_HOST: &str = "SMTP_HOST";
static SMTP_PORT: &str = "SMTP_PORT";
static SMTP_USERNAME: &str = "SMTP_USERNAME";
static SMTP_PASSWORD: &str = "SMTP_PASSWORD";
static SMTP_TLS_OFF: &str = "SMTP_TLS_OFF";
static SMTP_SECRETS: &str = "smtp-secrets";

// The web user interface
pub async fn deploy(client: Client, spec: BionicSpec, namespace: &str) -> Result<(), Error> {
    let mut env = vec![
        json!({
            "name":
            "VERSION",
            "value":
            format!("{}", spec.version)
        }),
        json!({
            "name":
            "APP_DATABASE_URL",
            "valueFrom": {
                "secretKeyRef": {
                    "name": "database-urls",
                    "key": "application-url"
                }
            }
        }),
        json!({
            "name":
            "PORT",
            "value":
            "7903"
        }),
        json!({
            "name":
            "LOGOUT_URL",
            "value":
            "/oidc/realms/bionic-gpt/protocol/openid-connect/logout"
        }),
        json!({
            "name":
            INVITE_DOMAIN,
            "valueFrom": {
                "secretKeyRef": {
                    "name": SMTP_SECRETS,
                    "key": lower_dash(INVITE_DOMAIN)
                }
            }
        }),
        json!({
            "name":
            INVITE_FROM_EMAIL_ADDRESS,
            "valueFrom": {
                "secretKeyRef": {
                    "name": SMTP_SECRETS,
                    "key": lower_dash(INVITE_FROM_EMAIL_ADDRESS)
                }
            }
        }),
        json!({
            "name":
            SMTP_HOST,
            "valueFrom": {
                "secretKeyRef": {
                    "name": SMTP_SECRETS,
                    "key": lower_dash(SMTP_HOST)
                }
            }
        }),
        json!({
            "name":
            SMTP_PORT,
            "valueFrom": {
                "secretKeyRef": {
                    "name": SMTP_SECRETS,
                    "key": lower_dash(SMTP_PORT)
                }
            }
        }),
        json!({
            "name":
            SMTP_USERNAME,
            "valueFrom": {
                "secretKeyRef": {
                    "name": SMTP_SECRETS,
                    "key": lower_dash(SMTP_USERNAME)
                }
            }
        }),
        json!({
            "name":
            SMTP_PASSWORD,
            "valueFrom": {
                "secretKeyRef": {
                    "name": SMTP_SECRETS,
                    "key": lower_dash(SMTP_PASSWORD)
                }
            }
        }),
        json!({
            "name":
            SMTP_TLS_OFF,
            "valueFrom": {
                "secretKeyRef": {
                    "name": SMTP_SECRETS,
                    "key": lower_dash(SMTP_TLS_OFF)
                }
            }
        }),
    ];

    if let Some(saas) = spec.saas {
        if saas {
            env.push(json!({
                "name":
                "ENABLE_SAAS",
                "value":
                "1"
            }));
        }
    }

    let image_name = if spec.hash_bionicgpt.is_empty() {
        format!("{}:{}", super::BIONICGPT_IMAGE, spec.version)
    } else {
        format!("{}@{}", super::BIONICGPT_IMAGE, spec.hash_bionicgpt)
    };

    let migrations_image_name = if spec.hash_bionicgpt_db_migrations.is_empty() {
        format!("{}:{}", super::BIONICGPT_DB_MIGRATIONS_IMAGE, spec.version)
    } else {
        format!(
            "{}@{}",
            super::BIONICGPT_DB_MIGRATIONS_IMAGE,
            spec.hash_bionicgpt_db_migrations
        )
    };

    // Bionic with the migrations as a sidecar
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "bionic-gpt".to_string(),
            image_name,
            replicas: spec.replicas,
            port: 7903,
            env,
            init_container: Some(deployment::InitContainer {
                image_name: migrations_image_name,
                env: vec![json!({
                "name":
                "DATABASE_URL",
                "valueFrom": {
                    "secretKeyRef": {
                        "name": "database-urls",
                        "key": "migrations-url"
                    }
                }})],
            }),
            command: None,
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await?;

    email_secret(namespace, spec, client).await?;

    Ok(())
}

// Create the email secret if it doesn't exist.
async fn email_secret(namespace: &str, spec: BionicSpec, client: Client) -> Result<(), Error> {
    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    let secret = secret_api.get(SMTP_SECRETS).await;
    if secret.is_err() {
        let mut secret_data = BTreeMap::new();
        secret_data.insert(lower_dash(INVITE_DOMAIN), spec.hostname_url);
        secret_data.insert(
            lower_dash(INVITE_FROM_EMAIL_ADDRESS),
            "support@application.com".to_string(),
        );
        secret_data.insert(lower_dash(SMTP_HOST), "mailhog".to_string());
        secret_data.insert(lower_dash(SMTP_PORT), "1025".to_string());
        secret_data.insert(lower_dash(SMTP_USERNAME), "thisisnotused".to_string());
        secret_data.insert(lower_dash(SMTP_PASSWORD), "thisisnotused".to_string());
        secret_data.insert(lower_dash(SMTP_TLS_OFF), "true".to_string());
        let keycloak_secret = Secret {
            metadata: ObjectMeta {
                name: Some(SMTP_SECRETS.to_string()),
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

fn lower_dash(s: &str) -> String {
    s.to_lowercase().replace('_', "-")
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get("bionic-gpt").await.is_ok() {
        api.delete("bionic-gpt", &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client, namespace);
    if api.get("bionic-gpt").await.is_ok() {
        api.delete("bionic-gpt", &DeleteParams::default()).await?;
    }
    Ok(())
}
