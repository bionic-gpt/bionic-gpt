use std::collections::BTreeMap;

use super::deployment;
use crate::error::Error;
use k8s_openapi::api::core::v1::{Container, PodSpec, ResourceRequirements};
use k8s_openapi::api::core::v1::{Pod, Service};
use k8s_openapi::api::node::v1::RuntimeClass;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use kube::api::{DeleteParams, ObjectMeta, PatchParams, PostParams};
use kube::{Api, Client};

pub const MODEL_NAME: &str = "phi-2-gptq";
pub const MODEL_REPOSITORY: &str = "TheBloke/phi-2-GPTQ";

// Large Language Model
pub async fn deploy(client: Client, namespace: &str) -> Result<(), Error> {
    let runtime_class = RuntimeClass {
        metadata: ObjectMeta {
            name: Some("nvidia".to_string()),
            ..ObjectMeta::default()
        },
        ..Default::default()
    };

    let runtimeclass_api: Api<RuntimeClass> = Api::all(client.clone());
    runtimeclass_api
        .create(&PostParams::default(), &runtime_class)
        .await?;

    let metadata = ObjectMeta {
        name: Some(MODEL_NAME.to_string()),
        ..Default::default()
    };

    let mut limits = BTreeMap::new();
    limits.insert("nvidia.com/gpu".to_string(), Quantity("1".to_string()));

    // Define the container
    let container = Container {
        name: MODEL_NAME.to_string(),
        image: Some(super::TGI_IMAGE.to_string()),
        args: Some(vec![
            "--model-id".to_string(),
            MODEL_REPOSITORY.to_string(),
            "--quantize".to_string(),
            "gptq".to_string(),
        ]),
        resources: Some(ResourceRequirements {
            limits: Some(limits),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Define the Pod
    let pod = Pod {
        metadata,
        spec: Some(PodSpec {
            runtime_class_name: Some("nvidia".to_string()),
            containers: vec![container],
            ..Default::default()
        }),
        ..Default::default()
    };

    let api: Api<Pod> = Api::namespaced(client.clone(), namespace);
    api.patch(
        MODEL_NAME,
        &PatchParams::apply(crate::MANAGER),
        &kube::api::Patch::Apply(pod),
    )
    .await?;

    deployment::service(client, MODEL_NAME, 80, namespace).await?;

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Pod> = Api::namespaced(client.clone(), namespace);
    if api.get(MODEL_NAME).await.is_ok() {
        api.delete(MODEL_NAME, &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get(MODEL_NAME).await.is_ok() {
        api.delete(MODEL_NAME, &DeleteParams::default()).await?;
    }

    Ok(())
}
