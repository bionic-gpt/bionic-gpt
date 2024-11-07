use super::deployment;
use crate::error::Error;
use crate::operator::crd::BionicSpec;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{Secret, Service};
use kube::api::{DeleteParams, PostParams};
use kube::{Api, Client};
use rand::rngs::OsRng;
use rand::RngCore;
use serde_json::json;
use url::Url;

// Oauth2 Proxy handles are authentication as our Open ID Connect provider
pub async fn deploy(client: Client, spec: BionicSpec, namespace: &str) -> Result<(), Error> {
    let whitelist_domain = Url::parse(&spec.hostname_url);
    let whitelist_domain = if let Ok(host) = &whitelist_domain {
        host.host_str().unwrap_or_default()
    } else {
        ""
    };

    // Oauth2 Proxy
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "oauth2-proxy".to_string(),
            image_name: super::OAUTH2_PROXY_IMAGE.to_string(),
            replicas: spec.replicas,
            port: 7900,
            env: vec![
                json!({"name": "OAUTH2_PROXY_HTTP_ADDRESS", "value": "0.0.0.0:7900"}),
                json!({
                    "name":
                    "OAUTH2_PROXY_COOKIE_SECRET",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "cookie-secret"
                        }
                    }
                }),
                json!({"name": "OAUTH2_PROXY_EMAIL_DOMAINS", "value": "*"}),
                json!({"name": "OAUTH2_PROXY_COOKIE_SECURE", "value": "false"}),
                json!({"name": "OAUTH2_PROXY_UPSTREAMS", "value": "http://envoy:7901"}),
                json!({"name": "OAUTH2_PROXY_UPSTREAM_TIMEOUT", "value": "600s"}),
                json!({
                    "name":
                    "OAUTH2_PROXY_CLIENT_SECRET",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "client-secret"
                        }
                    }
                }),
                json!({
                    "name":
                    "OAUTH2_PROXY_CLIENT_ID",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "client-id"
                        }
                    }
                }),
                json!({
                    "name":
                    "OAUTH2_PROXY_REDIRECT_URL",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "redirect-uri"
                        }
                    }
                }),
                json!({
                    "name":
                    "OAUTH2_PROXY_OIDC_ISSUER_URL",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "issuer-url"
                        }
                    }
                }),

                // This line sends us the user info in a JWT (which is base64 encoded)
                json!({"name": "OAUTH2_PROXY_PASS_ACCESS_TOKEN", "value": "true"}),

                json!({"name": "OAUTH2_PROXY_INSECURE_OIDC_SKIP_ISSUER_VERIFICATION", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_INSECURE_OIDC_ALLOW_UNVERIFIED_EMAIL", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_PROVIDER", "value": "oidc"}),
                json!({"name": "OAUTH2_PROXY_PROVIDER_DISPLAY_NAME", "value": "Keycloak"}),
                json!({"name": "OAUTH2_PROXY_AUTH_LOGGING", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_SKIP_PROVIDER_BUTTON", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_WHITELIST_DOMAINS", "value": whitelist_domain}),
                json!({"name": "OAUTH2_PROXY_SKIP_AUTH_ROUTES", "value": "^/v1*"}),
                json!({"name": "OAUTH2_PROXY_SCOPE", "value": "openid email profile"})
            ],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![],
            }),
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await?;

    oauthproxy_secret(namespace, spec, client).await?;

    Ok(())
}

async fn oauthproxy_secret(namespace: &str, spec: BionicSpec, client: Client) -> Result<(), Error> {
    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    let secret = secret_api.get("oidc-secret").await;
    if secret.is_err() {
        let secret = serde_json::from_value(serde_json::json!({
            "apiVersion": "v1",
            "kind": "Secret",
            "metadata": {
                "name": "oidc-secret",
                "namespace": namespace
            },
            "stringData": {
                "client-id": "bionic-gpt",
                "client-secret": "69b26b08-12fe-48a2-85f0-6ab223f45777",
                "redirect-uri": format!("{}/oauth2/callback", spec.hostname_url),
                "issuer-url": "http://keycloak:7910/oidc/realms/bionic-gpt",
                "cookie-secret": rand_base64()
            }
        }))?;
        secret_api.create(&PostParams::default(), &secret).await?;
    }
    Ok(())
}

pub fn rand_base64() -> String {
    // Generate random bytes
    let mut rng = OsRng;
    let mut random_bytes = [0u8; 32];
    rng.fill_bytes(&mut random_bytes);

    // Encode random bytes to Base64
    base64::encode_config(random_bytes, base64::URL_SAFE_NO_PAD)
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get("oauth2-proxy").await.is_ok() {
        api.delete("oauth2-proxy", &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get("oauth2-proxy").await.is_ok() {
        api.delete("oauth2-proxy", &DeleteParams::default()).await?;
    }

    let api: Api<Secret> = Api::namespaced(client.clone(), namespace);
    if api.get("oidc-secret").await.is_ok() {
        api.delete("oidc-secret", &DeleteParams::default()).await?;
    }

    Ok(())
}
