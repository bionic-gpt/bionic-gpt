use super::{bionic::BIONIC_NAME, keycloak::KEYCLOAK_NAME, rag_engine::NAME as RAG_ENGINE_NAME};
use crate::error::Error;
use k8s_openapi::api::networking::v1::NetworkPolicy;
use kube::api::{Patch, PatchParams};
use kube::{Api, Client};
use serde_json::json;

pub async fn default_deny(client: Client, name: &str, namespace: &str) -> Result<(), Error> {
    let policy_name = format!("{}-network-policy", name);

    // Ingress: allow from same namespace + ingress-nginx
    let ingress = json!([{
        "from": [
            {
                "namespaceSelector": {
                    "matchLabels": {
                        "kubernetes.io/metadata.name": namespace
                    }
                }
            },
            {
                "namespaceSelector": {
                    "matchLabels": {
                        "kubernetes.io/metadata.name": "ingress-nginx"
                    }
                }
            }
        ]
    }]);

    // Egress: allow DNS + namespace-local traffic
    let egress = if name == BIONIC_NAME || name == KEYCLOAK_NAME || name == RAG_ENGINE_NAME {
        json!([
            { "to": [{ "ipBlock": { "cidr": "0.0.0.0/0" } }] }
        ])
    } else {
        json!([
            {
                "to": [
                    { "namespaceSelector": { "matchLabels": { "kubernetes.io/metadata.name": namespace } } }
                ]
            },
            {
                "to": [
                    { "namespaceSelector": { "matchLabels": { "kubernetes.io/metadata.name": "kube-system" } } }
                ],
                "ports": [
                    { "protocol": "UDP", "port": 53 },
                    { "protocol": "TCP", "port": 53 }
                ]
            }
        ])
    };

    let policy = json!({
        "apiVersion": "networking.k8s.io/v1",
        "kind": "NetworkPolicy",
        "metadata": {
            "name": policy_name,
            "namespace": namespace
        },
        "spec": {
            "podSelector": { "matchLabels": { "app": name } },
            "policyTypes": ["Ingress", "Egress"],
            "ingress": ingress,
            "egress": egress
        }
    });

    let policies: Api<NetworkPolicy> = Api::namespaced(client, namespace);
    policies
        .patch(
            &policy_name,
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(policy),
        )
        .await?;

    Ok(())
}
