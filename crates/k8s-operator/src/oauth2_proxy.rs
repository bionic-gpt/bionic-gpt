use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
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
            env: vec![
                json!({"name": "OAUTH2_PROXY_HTTP_ADDRESS", "value": "0.0.0.0:7900"}),
                json!({"name": "OAUTH2_PROXY_COOKIE_SECRET", "value": "OQINaROshtE9TcZkNAm-5Zs2Pv3xaWytBmc5W7sPX7w="}),
                json!({"name": "OAUTH2_PROXY_EMAIL_DOMAINS", "value": "*"}),
                json!({"name": "OAUTH2_PROXY_COOKIE_SECURE", "value": "false"}),
                json!({"name": "OAUTH2_PROXY_UPSTREAMS", "value": "http://envoy:7901"}),
                json!({"name": "OAUTH2_PROXY_UPSTREAMS_TIMEOUT", "value": "600s"}),
                json!({"name": "OAUTH2_PROXY_CLIENT_SECRET", "value": "69b26b08-12fe-48a2-85f0-6ab223f45777"}),
                json!({"name": "OAUTH2_PROXY_CLIENT_ID", "value": "bionic-gpt"}),
                json!({"name": "OAUTH2_PROXY_REDIRECT_URL", "value": "http://localhost:7900/oauth2/callback"}),
                json!({"name": "OAUTH2_PROXY_OIDC_ISSUER_URL", "value": "http://keycloak:7910/realms/bionic-gpt"}),
                json!({"name": "OAUTH2_PROXY_INSECURE_OIDC_SKIP_ISSUER_VERIFICATION", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_INSECURE_OIDC_ALLOW_UNVERIFIED_EMAIL", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_PROVIDER", "value": "oidc"}),
                json!({"name": "OAUTH2_PROXY_PROVIDER_DISPLAY_NAME", "value": "Keycloak"}),
                json!({"name": "OAUTH2_PROXY_AUTH_LOGGING", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_SKIP_PROVIDER_BUTTON", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_WHITELIST_DOMAINS", "value": "localhost:7910"}),
                json!({"name": "OAUTH2_PROXY_SKIP_AUTH_ROUTES", "value": "^/v1*"})
            ],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
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
