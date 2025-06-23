use crate::error::Error;
use k8s_openapi::api::networking::v1::NetworkPolicy;
use kube::api::{Patch, PatchParams};
use kube::{Api, Client};
use serde_json::json;

pub async fn default_deny(client: Client, name: &str, namespace: &str) -> Result<(), Error> {
    let policy_name = format!("{}-network-policy", name);

    // Allow bionic-gpt to reach the internet while keeping all other services
    // locked down to the namespace.
    let egress = if name == "bionic-gpt" {
        json!([{ "to": [{ "ipBlock": { "cidr": "0.0.0.0/0" } }] }])
    } else {
        json!([{ "to": [{ "namespaceSelector": { "matchLabels": { "kubernetes.io/metadata.name": namespace } } }] }])
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
            "ingress": [{
                "from": [{
                    "namespaceSelector": { "matchLabels": { "kubernetes.io/metadata.name": namespace } }
                }]
            }],
            "egress": egress
        }
    });

    let policies: Api<NetworkPolicy> = Api::namespaced(client, namespace);
    policies
        .patch(
            &policy_name,
            &PatchParams::apply(crate::MANAGER),
            &Patch::Apply(policy),
        )
        .await?;

    Ok(())
}
