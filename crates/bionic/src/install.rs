use crate::error::Error;
use k8s_openapi::api::core::v1::Namespace;
use kube::Api;
use kube::Client;

pub async fn install(namespace: &str) -> Result<(), Error> {
    let client = Client::try_default().await?;

    // Define the API object for Namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());

    let ns = namespaces.get(namespace).await;

    if ns.is_ok() {
        return Err(Error::General("Namespace already exists".to_string()));
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
        let _created_namespace = namespaces
            .create(&Default::default(), &new_namespace)
            .await?;
    }
    Ok(())
}
