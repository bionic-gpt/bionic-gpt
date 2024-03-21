use std::collections::BTreeMap;

use crate::error::Error;
use k8s_openapi::api::networking::v1::{
    HTTPIngressPath, HTTPIngressRuleValue, Ingress, IngressBackend, IngressRule,
    IngressServiceBackend, IngressSpec, ServiceBackendPort,
};
use kube::api::{DeleteParams, ObjectMeta};
use kube::{
    api::{Api, PostParams},
    Client,
};

const INGRESS: &str = "bionic-gpt-ingress";

/// Create a deployment and a service.
/// Include sidecars if needed.
pub async fn deploy(client: Client, namespace: &str, pgadmin: bool) -> Result<(), Error> {
    let mut annotations = BTreeMap::new();
    annotations.insert(
        "nginx.ingress.kubernetes.io/proxy-buffer-size".to_string(),
        "128k".to_string(),
    );
    annotations.insert(
        "nginx.ingress.kubernetes.io/proxy-body-size".to_string(),
        "50m".to_string(),
    );
    annotations.insert(
        "traefik.ingress.kubernetes.io/router.entrypoints".to_string(),
        "web".to_string(),
    );

    // Define the metadata for the Ingress
    let metadata = ObjectMeta {
        name: Some(INGRESS.to_string()),
        namespace: Some(namespace.into()),
        annotations: Some(annotations),
        ..Default::default()
    };

    let mut paths = vec![
        HTTPIngressPath {
            path: Some("/oidc".to_string()),
            path_type: "Prefix".to_string(),
            backend: IngressBackend {
                service: Some(IngressServiceBackend {
                    name: "keycloak".to_string(),
                    port: Some(ServiceBackendPort {
                        number: Some(7910),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            },
        },
        HTTPIngressPath {
            path: Some("/".to_string()),
            path_type: "Prefix".to_string(),
            backend: IngressBackend {
                service: Some(IngressServiceBackend {
                    name: "oauth2-proxy".to_string(),
                    port: Some(ServiceBackendPort {
                        number: Some(7900),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            },
        },
    ];

    if pgadmin {
        paths.push(HTTPIngressPath {
            path: Some("/pgadmin".to_string()),
            path_type: "Prefix".to_string(),
            backend: IngressBackend {
                service: Some(IngressServiceBackend {
                    name: "pgadmin".to_string(),
                    port: Some(ServiceBackendPort {
                        number: Some(80),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            },
        });
    }

    // Define the spec for the Ingress
    let spec = IngressSpec {
        rules: Some(vec![IngressRule {
            http: Some(HTTPIngressRuleValue { paths }),
            ..Default::default()
        }]),
        ..Default::default()
    };

    // Define the Ingress object
    let ingress = Ingress {
        metadata,
        spec: Some(spec),
        ..Default::default()
    };

    // Create the deployment defined above
    let ingress_api: Api<Ingress> = Api::namespaced(client.clone(), namespace);
    ingress_api.create(&PostParams::default(), &ingress).await?;

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Ingress> = Api::namespaced(client.clone(), namespace);
    api.delete(INGRESS, &DeleteParams::default()).await?;

    Ok(())
}
