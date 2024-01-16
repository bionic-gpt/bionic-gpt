mod bionic;
mod chunking_engine;
mod crd;
mod deployment;
mod embeddings_engine;
mod envoy;
mod error;
mod finalizer;
mod keycloak;
mod llm;
mod oauth2_proxy;
mod pipeline_job;
mod postgres;
mod reconcile;
use anyhow::Result;
use crd::Bionic;
use futures_util::stream::StreamExt;
use kube::{api::Api, Client};
use kube_runtime::{watcher::Config, Controller};
use reconcile::ContextData;
use std::sync::Arc;

const BIONICGPT_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt";
const BIONICGPT_PIPELINE_JOB_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt-pipeline-job";
const BIONICGPT_DB_MIGRATIONS_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt-db-migrations";

const KEYCLOAK_IMAGE: &str = "quay.io/keycloak/keycloak:23.0";
const ENVOYPROXY_IMAGE: &str = "envoyproxy/envoy:v1.28.0";
const OAUTH2_PROXY_IMAGE: &str = "quay.io/oauth2-proxy/oauth2-proxy:v7.5.1";
const POSTGRES_PGVECTOR_IMAGE: &str = "ankane/pgvector";
const _LITE_LLM_IMAGE: &str = "ghcr.io/berriai/litellm:main-v1.10.3";
const _TGI_IMAGE: &str = "ghcr.io/huggingface/text-generation-inference:1.2";
const CHUNKING_ENGINE_IMAGE: &str =
    "downloads.unstructured.io/unstructured-io/unstructured-api:4ffd8bc";
const EMBEDDINGS_ENGINE_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt-embeddings-api:cpu-0.6";
const LLM_API_IMAGE: &str = "ghcr.io/bionic-gpt/llama-2-7b-chat:1.0.4";

#[tokio::main]
async fn main() -> Result<()> {
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
