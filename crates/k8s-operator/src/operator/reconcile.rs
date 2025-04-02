use super::crd::Bionic;
use super::finalizer;
use crate::error::Error;
use crate::services::bionic;
use crate::services::chunking_engine;
use crate::services::database;
use crate::services::embeddings_engine;
use crate::services::envoy;
use crate::services::http_mock;
use crate::services::ingress;
use crate::services::keycloak;
use crate::services::keycloak_db;
use crate::services::llm;
use crate::services::llm_lite;
use crate::services::mailhog;
use crate::services::nginx::deploy_nginx;
use crate::services::oauth2_proxy;
use crate::services::observability;
use crate::services::pgadmin;
use crate::services::rag_engine;
use crate::services::tgi;
use k8s_openapi::api::core::v1::Pod;
use kube::api::ListParams;
use kube::Api;
use kube::Client;
use kube::Resource;
use kube::ResourceExt;
use kube_runtime::controller::Action;
use std::{sync::Arc, time::Duration};

/// Context injected with each `reconcile` and `on_error` method invocation.
pub struct ContextData {
    /// Kubernetes client to make Kubernetes API requests with. Required for K8S resource management.
    client: Client,
}

impl ContextData {
    // Constructs a new instance of ContextData.
    //
    // # Arguments:
    // - `client`: A Kubernetes client to make Kubernetes REST API requests with.
    // Resources will be created and deleted with this client.
    pub fn new(client: Client) -> Self {
        ContextData { client }
    }
}

/// Action to be taken upon an `Bionic` resource during reconciliation
enum BionicAction {
    /// Create the subresources, this includes spawning `n` pods with Bionic service
    Create,
    /// Delete all subresources created in the `Create` phase
    Delete,
    /// CRD version has chnaged, upgrade the installation.
    Upgrade,
    /// This `Bionic` resource is in desired state and requires no actions to be taken
    NoOp,
}

pub async fn reconcile(bionic: Arc<Bionic>, context: Arc<ContextData>) -> Result<Action, Error> {
    let client: Client = context.client.clone(); // The `Client` is shared -> a clone from the reference is obtained

    let namespace: String = bionic.namespace().unwrap_or("default".to_string());
    let name = bionic.name_any();

    let gpu = bionic.spec.gpu.unwrap_or_default();

    let pgadmin = bionic.spec.pgadmin.unwrap_or_default();

    let disable_ingress = bionic.spec.disable_ingress.unwrap_or_default();

    let observability = bionic.spec.observability.unwrap_or_default();

    let testing = bionic.spec.testing.unwrap_or_default();

    let development = bionic.spec.development.unwrap_or_default();

    let bionic_version = get_current_bionic_version(&client, &namespace).await?;

    // Performs action as decided by the `determine_action` function.
    match determine_action(&bionic, bionic_version) {
        BionicAction::Create | BionicAction::Upgrade => {
            // Creates a deployment with `n` Bionic service pods, but applies a finalizer first.
            // Finalizer is applied first, as the operator might be shut down and restarted
            // at any time, leaving subresources in intermediate state. This prevents leaks on
            // the `Bionic` resource deletion.

            // Apply the finalizer first. If that fails, the `?` operator invokes automatic conversion
            // of `kube::Error` to the `Error` defined in this crate.
            finalizer::add(client.clone(), &name, &namespace).await?;

            let override_db_password = if development {
                Some("testpassword".to_string())
            } else {
                None
            };

            // The databases
            let bionic_db_pass = database::deploy(
                client.clone(),
                &namespace,
                bionic.spec.bionic_db_disk_size,
                &override_db_password,
            )
            .await?;
            let keycloak_db_pass = keycloak_db::deploy(
                client.clone(),
                &namespace,
                bionic.spec.keycloak_db_disk_size,
            )
            .await?;

            bionic::deploy(client.clone(), bionic.spec.clone(), &namespace).await?;
            rag_engine::deploy(client.clone(), bionic.spec.clone(), &namespace).await?;
            envoy::deploy(client.clone(), bionic.spec.clone(), &namespace).await?;
            keycloak::deploy(client.clone(), bionic.spec.clone(), &namespace).await?;
            oauth2_proxy::deploy(client.clone(), bionic.spec.clone(), &namespace).await?;
            if !disable_ingress || development {
                ingress::deploy(client.clone(), &namespace, pgadmin, observability).await?;
            }

            if development || testing {
                deploy_nginx(&client, &namespace).await.unwrap();
            }

            mailhog::deploy(client.clone(), &namespace).await?;
            if gpu {
                tgi::deploy(client.clone(), &namespace).await?;
                llm_lite::deploy(client.clone(), &namespace).await?;
            } else if testing {
                http_mock::deploy(client.clone(), llm::NAME, 11434, &namespace).await?;
            } else {
                llm::deploy(client.clone(), bionic.spec.clone(), &namespace).await?;
            }
            if pgadmin {
                pgadmin::deploy(
                    client.clone(),
                    bionic_db_pass.clone(),
                    keycloak_db_pass,
                    &namespace,
                )
                .await?;
            }
            if observability {
                observability::deploy(
                    client.clone(),
                    bionic_db_pass,
                    bionic.spec.clone(),
                    &namespace,
                )
                .await?;
            }
            if testing {
                http_mock::deploy(
                    client.clone(),
                    chunking_engine::NAME,
                    chunking_engine::PORT,
                    &namespace,
                )
                .await?;
                http_mock::deploy(
                    client.clone(),
                    embeddings_engine::NAME,
                    embeddings_engine::PORT,
                    &namespace,
                )
                .await?;
            } else {
                chunking_engine::deploy(client.clone(), bionic.spec.clone(), &namespace).await?;
                embeddings_engine::deploy(client.clone(), bionic.spec.clone(), &namespace).await?;
            }
            Ok(Action::requeue(Duration::from_secs(10)))
        }
        BionicAction::Delete => {
            if gpu {
                tgi::delete(client.clone(), &namespace).await?;
                llm_lite::delete(client.clone(), &namespace).await?;
            } else {
                llm::delete(client.clone(), &namespace).await?;
            }

            envoy::delete(client.clone(), &namespace).await?;
            keycloak::delete(client.clone(), &namespace).await?;
            keycloak_db::delete(client.clone(), &namespace).await?;
            oauth2_proxy::delete(client.clone(), &namespace).await?;

            if !disable_ingress {
                ingress::delete(client.clone(), &namespace).await?;
            }
            mailhog::delete(client.clone(), &namespace).await?;

            if !development {
                rag_engine::delete(client.clone(), &namespace).await?;
                bionic::delete(client.clone(), &namespace).await?;
            }
            database::delete(client.clone(), &namespace).await?;
            chunking_engine::delete(client.clone(), &namespace).await?;
            embeddings_engine::delete(client.clone(), &namespace).await?;
            if pgadmin {
                pgadmin::delete(client.clone(), &namespace).await?;
            }
            if observability {
                observability::delete(client.clone(), &namespace).await?;
            }

            // Once the deployment is successfully removed, remove the finalizer to make it possible
            // for Kubernetes to delete the `Bionic` resource.
            finalizer::delete(client, &name, &namespace).await?;
            Ok(Action::await_change()) // Makes no sense to delete after a successful delete, as the resource is gone
        }
        // The resource is already in desired state, do nothing and re-check after 10 seconds
        BionicAction::NoOp => Ok(Action::requeue(Duration::from_secs(10))),
    }
}

/// If we already have a deployment, get the version we are running.
/// We can do this by getting the bionic pod by name
async fn get_current_bionic_version(
    client: &Client,
    namespace: &str,
) -> Result<Option<String>, Error> {
    // Get the API for Pod resources in the specified namespace
    let pods: Api<Pod> = Api::namespaced(client.clone(), namespace);
    let lp = ListParams::default().labels("app=bionic-gpt"); // for this app only

    for p in pods.list(&lp).await? {
        if let Some(spec) = p.spec {
            for container in spec.containers {
                if let Some(env_vars) = container.env {
                    for env_var in env_vars {
                        if env_var.name == "VERSION" {
                            return Ok(env_var.value);
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Resources arrives into reconciliation queue in a certain state. This function looks at
/// the state of given `Bionic` resource and decides which actions needs to be performed.
/// The finite set of possible actions is represented by the `BionicAction` enum.
///
/// # Arguments
/// - `echo`: A reference to `Bionic` being reconciled to decide next action upon.
fn determine_action(bionic: &Bionic, bionic_version: Option<String>) -> BionicAction {
    let bionic_version = if let Some(bionic_version) = bionic_version {
        bionic_version
    } else {
        "".to_string()
    };

    if bionic.meta().deletion_timestamp.is_some() {
        BionicAction::Delete
    } else if bionic
        .meta()
        .finalizers
        .as_ref()
        .map(|v| v.is_empty())
        .unwrap_or(true)
    {
        BionicAction::Create
    } else if bionic.spec.version != bionic_version {
        tracing::info!(
            "Upgrade detected from {} to {}",
            bionic_version,
            bionic.spec.version
        );
        BionicAction::Upgrade
    } else {
        BionicAction::NoOp
    }
}

/// Actions to be taken when a reconciliation fails - for whatever reason.
/// Prints out the error to `stderr` and requeues the resource for another reconciliation after
/// five seconds.
///
/// # Arguments
/// - `bionic`: The erroneous resource.
/// - `error`: A reference to the `kube::Error` that occurred during reconciliation.
/// - `_context`: Unused argument. Context Data "injected" automatically by kube-rs.
pub fn on_error(bionic: Arc<Bionic>, error: &Error, _context: Arc<ContextData>) -> Action {
    eprintln!("Reconciliation error:\n{:?}.\n{:?}", error, bionic);
    Action::requeue(Duration::from_secs(5))
}
