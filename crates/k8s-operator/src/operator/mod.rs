pub mod crd;
mod finalizer;
mod reconcile;
use anyhow::Result;
use crd::Bionic;
use futures_util::stream::StreamExt;
use kube::{api::Api, Client};
use kube_runtime::{watcher::Config, Controller};
use reconcile::ContextData;
use std::sync::Arc;

pub async fn operator() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let kubernetes_client = Client::try_default().await?;

    // Preparation of resources used by the `kube_runtime::Controller`
    let crd_api: Api<Bionic> = Api::all(kubernetes_client.clone());
    let context: Arc<ContextData> = Arc::new(ContextData::new(kubernetes_client.clone()));

    // The controller comes from the `kube_runtime` crate and manages the reconciliation process.
    // It requires the following information:
    // - `kube::Api<T>` this controller "owns". In this case, `T = Bionic`, as this controller owns the `Bionic` resource,
    // - `kube::runtime::watcher::Config` can be adjusted for precise filtering of `Bionic` resources before the actual reconciliation, e.g. by label,
    // - `reconcile` function with reconciliation logic to be called each time a resource of `Bionic` kind is created/updated/deleted,
    // - `on_error` function to call whenever reconciliation fails.
    Controller::new(crd_api.clone(), Config::default())
        .run(reconcile::reconcile, reconcile::on_error, context)
        .for_each(|reconciliation_result| async move {
            match reconciliation_result {
                Ok(echo_resource) => {
                    println!("Reconciliation successful. Resource: {:?}", echo_resource);
                }
                Err(reconciliation_err) => {
                    eprintln!("Reconciliation error: {:?}", reconciliation_err)
                }
            }
        })
        .await;

    Ok(())
}
