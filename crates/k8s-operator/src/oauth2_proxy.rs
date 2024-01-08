use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{ConfigMap, Service};
use kube::api::{DeleteParams, PostParams};
use kube::{Api, Client};
use serde_json::json;


// Oauth2 Proxy handles are authentication as our Open ID Connect provider
pub async fn deploy(
    client: Client,
    _name: &str,
    spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {

    // Oauth2 Proxy
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "oauth2-proxy".to_string(),
            image_name: spec.oauth2_proxy_image,
            replicas: spec.replicas,
            port: 7900,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![
                    "OAUTH2_PROXY_HTTP_ADDRESS": "0.0.0.0:7900",
                    "OAUTH2_PROXY_COOKIE_SECRET": "OQINaROshtE9TcZkNAm-5Zs2Pv3xaWytBmc5W7sPX7w=",
                    "OAUTH2_PROXY_EMAIL_DOMAINS": "*",
                    "OAUTH2_PROXY_COOKIE_SECURE": "false",
                    "OAUTH2_PROXY_UPSTREAMS": "http://envoy:7901",
                    "OAUTH2_PROXY_UPSTREAMS_TIMEOUT": "600s",
                    "OAUTH2_PROXY_CLIENT_SECRET": "69b26b08-12fe-48a2-85f0-6ab223f45777",
                    "OAUTH2_PROXY_CLIENT_ID": "bionic-gpt",
                    "OAUTH2_PROXY_REDIRECT_URL": "http://localhost:7900/oauth2/callback",
                    "OAUTH2_PROXY_OIDC_ISSUER_URL": "http://keycloak:7910/realms/bionic-gpt",
                    "OAUTH2_PROXY_INSECURE_OIDC_SKIP_ISSUER_VERIFICATION": "true",
                    "OAUTH2_PROXY_INSECURE_OIDC_ALLOW_UNVERIFIED_EMAIL": "true",
                    "OAUTH2_PROXY_PROVIDER": "oidc",
                    "OAUTH2_PROXY_PROVIDER_DISPLAY_NAME": "Keycloak",
                    "OAUTH2_PROXY_AUTH_LOGGING": "true",
                    "OAUTH2_PROXY_SKIP_PROVIDER_BUTTON": "true",
                    "OAUTH2_PROXY_WHITELIST_DOMAINS": "localhost:7910",
                    "OAUTH2_PROXY_SKIP_AUTH_ROUTES": "^/v1*"
                ],
                args: vec![],
            }),
            expose_service: true,
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn delete(client: Client, _name: &str, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete("oauth2-proxy", &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete("oauth2-proxy", &DeleteParams::default()).await?;

    Ok(())
}
