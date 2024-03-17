use crate::crd::BionicSpec;
use crate::deployment;
use crate::error::Error;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{Secret, Service};
use kube::api::{DeleteParams, PostParams};
use kube::{Api, Client};
use rand::{rngs::OsRng, RngCore};
use serde_json::json;

const PGADMIN: &str = "pgadmin";

// Large Language Model
pub async fn deploy(
    client: Client,
    _name: &str,
    _spec: BionicSpec,
    namespace: &str,
) -> Result<(), Error> {
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: PGADMIN.to_string(),
            image_name: crate::PGADMIN_IMAGE.to_string(),
            replicas: 1,
            port: 80,
            env: vec![
                json!({
                    "name":
                    "PGADMIN_DEFAULT_EMAIL",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "pgadmin",
                            "key": "email"
                        }
                    }
                }),
                json!({
                    "name":
                    "PGADMIN_DEFAULT_PASSWORD",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "pgadmin",
                            "key": "password"
                        }
                    }
                }),
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

    let secret = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "Secret",
        "metadata": {
            "name": PGADMIN,
            "namespace": namespace
        },
        "stringData": {
            "email": "pgadmin@pgadmin.com",
            "password": rand_base64()
        }
    }))?;

    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    secret_api.create(&PostParams::default(), &secret).await?;

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

pub async fn delete(client: Client, _name: &str, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    api.delete(PGADMIN, &DeleteParams::default()).await?;

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    api.delete(PGADMIN, &DeleteParams::default()).await?;

    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    secret_api.delete(PGADMIN, &DeleteParams::default()).await?;

    Ok(())
}
