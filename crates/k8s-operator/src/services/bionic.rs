use std::collections::BTreeMap;

use crate::error::Error;
use crate::operator::crd::BionicSpec;
use crate::services::deployment;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{ConfigMap, Secret, Service};
use kube::api::{DeleteParams, ObjectMeta, Patch, PatchParams, PostParams};
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

static LICENCE: &str = "LICENCE";
static LICENCE_SECRET: &str = "bionic-gpt-licence";

pub const BIONIC_NAME: &str = "bionic-gpt";
static CONFIG_MAP_NAME: &str = "bionic-config";
static MAX_UPLOAD_SIZE_MB_KEY: &str = "MAX_UPLOAD_SIZE_MB";
static MAX_ATTACHMENTS_KEY: &str = "MAX_ATTACHMENTS";

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
            "name": "APP_BASE_URL",
            "value": spec.hostname_url.clone()
        }),
        json!({
            "name": MAX_UPLOAD_SIZE_MB_KEY,
            "valueFrom": {
                "configMapKeyRef": {
                    "name": CONFIG_MAP_NAME,
                    "key": MAX_UPLOAD_SIZE_MB_KEY
                }
            }
        }),
        json!({
            "name": MAX_ATTACHMENTS_KEY,
            "valueFrom": {
                "configMapKeyRef": {
                    "name": CONFIG_MAP_NAME,
                    "key": MAX_ATTACHMENTS_KEY
                }
            }
        }),
        json!({
            "name":
            LICENCE,
            "valueFrom": {
                "secretKeyRef": {
                    "name": LICENCE_SECRET,
                    "key": LICENCE
                }
            }
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

    let config_map = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": CONFIG_MAP_NAME,
            "namespace": namespace
        },
        "data": {
            MAX_UPLOAD_SIZE_MB_KEY: "1000",
            MAX_ATTACHMENTS_KEY: "5"
        }
    });

    let config_api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    config_api
        .patch(
            CONFIG_MAP_NAME,
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(config_map),
        )
        .await?;

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
            name: BIONIC_NAME.to_string(),
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

    email_secret(namespace, spec.clone(), client.clone()).await?;

    licence_secret(namespace, client).await?;

    Ok(())
}

// Create a dummy licence if it doesn't exist
async fn licence_secret(namespace: &str, client: Client) -> Result<(), Error> {
    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    let secret = secret_api.get(LICENCE_SECRET).await;
    if secret.is_err() {
        let mut secret_data = BTreeMap::new();
        secret_data.insert(
            LICENCE.to_string(),
            "Contact https://bionic-gpt.com for a licence.".to_string(),
        );

        let secret = Secret {
            metadata: ObjectMeta {
                name: Some(LICENCE_SECRET.to_string()),
                namespace: Some(namespace.to_string()),
                ..ObjectMeta::default()
            },
            string_data: Some(secret_data),
            ..Default::default()
        };
        secret_api.create(&PostParams::default(), &secret).await?;
    }
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
    if api.get(BIONIC_NAME).await.is_ok() {
        api.delete(BIONIC_NAME, &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get(BIONIC_NAME).await.is_ok() {
        api.delete(BIONIC_NAME, &DeleteParams::default()).await?;
    }

    // Remove configmap
    let api: Api<ConfigMap> = Api::namespaced(client, namespace);
    if api.get(CONFIG_MAP_NAME).await.is_ok() {
        api.delete(CONFIG_MAP_NAME, &DeleteParams::default())
            .await?;
    }
    Ok(())
}
