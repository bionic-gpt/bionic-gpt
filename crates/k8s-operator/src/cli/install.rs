use crate::error::Error;
use crate::operator::crd::Bionic;
use crate::operator::crd::BionicSpec;
use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::api::ObjectMeta;
use kube::api::Patch;
use kube::api::PatchParams;
use kube::api::PostParams;
use kube::Api;
use kube::Client;
use kube::CustomResourceExt;
use local_ip_address::local_ip;
use tracing::info;

pub async fn install(installer: &crate::cli::Installer) -> Result<(), Error> {
    let client = Client::try_default().await?;

    // Define the API object for Namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());

    let ns = namespaces.get(&installer.namespace).await;

    if ns.is_ok() {
        return Err(Error::Cli("Namespace already exists".to_string()));
    } else {
        create_namespace(&installer.namespace, namespaces).await?;
        create_crd(&client).await?;

        let my_local_ip = local_ip().unwrap();
        let bionic_api: Api<Bionic> = Api::all(client);
        let bionic = Bionic {
            metadata: {
                ObjectMeta {
                    namespace: Some(installer.namespace.clone()),
                    ..ObjectMeta::default()
                }
            },
            spec: BionicSpec {
                replicas: 1,
                version: "v1".into(),
                gpu: Some(installer.gpu),
                pgadmin: Some(installer.pgadmin),
                testing: Some(installer.testing),
                hostname_url: format!("{:?}", my_local_ip),
            },
        };
        bionic_api.create(&PostParams::default(), &bionic).await?;
    }
    Ok(())
}

async fn create_crd(client: &Client) -> Result<(), Error> {
    let ssapply = PatchParams::apply("bionic-gpt").force();
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    info!("Creating crd: {:?}", &Bionic::crd());
    crds.patch(
        "bionics.bionic-gpt.com",
        &ssapply,
        &Patch::Apply(Bionic::crd()),
    )
    .await?;
    Ok(())
}

async fn create_namespace(namespace: &str, namespaces: Api<Namespace>) -> Result<(), Error> {
    let new_namespace = Namespace {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some(namespace.to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    namespaces
        .create(&Default::default(), &new_namespace)
        .await?;
    Ok(())
}
