use crate::error::Error;
use crate::operator::crd::Bionic;
use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::api::Patch;
use kube::api::PatchParams;
use kube::Api;
use kube::Client;
use kube::CustomResourceExt;
use tracing::info;

const _BIONIC_OPERATOR: &str = include_str!("../../config/bionic-operator.yaml");
const _BIONIC_CONFIG: &str = include_str!("../../config/bionic.yaml");

pub async fn install(namespace: &str) -> Result<(), Error> {
    let client = Client::try_default().await?;

    // Define the API object for Namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());

    let ns = namespaces.get(namespace).await;

    if ns.is_ok() {
        return Err(Error::Cli("Namespace already exists".to_string()));
    } else {
        // Create the namespace
        let new_namespace = Namespace {
            metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                name: Some(namespace.to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        // Send the request to create the namespace
        namespaces
            .create(&Default::default(), &new_namespace)
            .await?;

        let ssapply = PatchParams::apply("bionic-gpt").force();

        // Now create the Custom Resource Definition
        let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
        info!("Creating crd: {:?}", &Bionic::crd());
        crds.patch(
            "bionics.bionic-gpt.com",
            &ssapply,
            &Patch::Apply(Bionic::crd()),
        )
        .await?;
    }
    Ok(())
}
