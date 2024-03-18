use crate::crd::BionicSpec;
use crate::error::Error;
use k8s_openapi::api::networking::v1::Ingress;
use kube::api::DeleteParams;
use kube::{
    api::{Api, PostParams},
    Client,
};

const INGRESS: &str = "bionic-gpt-ingress";

/// Create a deployment and a service.
/// Include sidecars if needed.
pub async fn deploy(
    client: Client,
    _name: &str,
    _spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    let ingress = serde_json::from_value(serde_json::json!({
        "apiVersion": "networking.k8s.io/v1",
        "kind": "Ingress",
        "metadata": {
            "name": INGRESS,
            "namespace": namespace,
            "annotations": {
                // We need to set the buffer size or keycloak won't let you register
                "nginx.ingress.kubernetes.io/proxy-buffer-size": "128k",
                // We need toi set this as the max size for document upload
                "nginx.ingress.kubernetes.io/proxy-body-size": "50m",
                // Used by traefik
                "traefik.ingress.kubernetes.io/router.entrypoints": "web"
            }
        },
        "spec": {
            "rules": [
                {
                    "http": {
                        "paths": [
                            {
                                "path": "/oidc",
                                "pathType": "Prefix",
                                "backend": {
                                    "service": {
                                        "name": "keycloak",
                                        "port": {
                                            "number": 7910
                                        }
                                    }
                                }
                            },
                            {
                                "path": "/",
                                "pathType": "Prefix",
                                "backend": {
                                    "service": {
                                        "name": "oauth2-proxy",
                                        "port": {
                                            "number": 7900
                                        }
                                    }
                                }
                            }
                        ]
                    }
                }
            ]
        }
    }))?;

    // Create the deployment defined above
    let ingress_api: Api<Ingress> = Api::namespaced(client.clone(), namespace);
    ingress_api.create(&PostParams::default(), &ingress).await?;

    Ok(())
}

pub async fn delete(client: Client, _name: &str, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Ingress> = Api::namespaced(client.clone(), namespace);
    api.delete(INGRESS, &DeleteParams::default()).await?;

    Ok(())
}
