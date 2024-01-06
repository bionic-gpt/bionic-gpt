use garde::Validate;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Own custom resource
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, Validate, JsonSchema)]
#[kube(group = "bionic-gpt.com", version = "v1", kind = "Bionic", namespaced)]
#[kube(status = "BionicStatus")]
#[kube(scale = r#"{"specReplicasPath":".spec.replicas", "statusReplicasPath":".status.replicas"}"#)]
#[kube(printcolumn = r#"{"name":"Team", "jsonPath": ".spec.metadata.team", "type": "string"}"#)]
pub struct BionicSpec {
    #[schemars(length(min = 3))]
    #[garde(length(min = 3))]
    pub name: String,
    #[garde(skip)]
    pub info: String,
    #[garde(skip)]
    pub replicas: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct BionicStatus {
    pub is_bad: bool,
    pub replicas: i32,
}
