use crate::bionic;
use crate::crd::Bionic;
use crate::envoy;
use crate::error::Error;
use crate::finalizer;
use crate::keycloak;
use crate::oauth2_proxy;
use crate::postgres;
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
    /// Constructs a new instance of ContextData.
    ///
    /// # Arguments:
    /// - `client`: A Kubernetes client to make Kubernetes REST API requests with. Resources
    /// will be created and deleted with this client.
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
    /// This `Bionic` resource is in desired state and requires no actions to be taken
    NoOp,
}

pub async fn reconcile(bionic: Arc<Bionic>, context: Arc<ContextData>) -> Result<Action, Error> {
    let client: Client = context.client.clone(); // The `Client` is shared -> a clone from the reference is obtained

    let namespace: String = bionic.namespace().unwrap_or("default".to_string());
    let name = bionic.name_any();

    // Performs action as decided by the `determine_action` function.
    match determine_action(&bionic) {
        BionicAction::Create => {
            // Creates a deployment with `n` Bionic service pods, but applies a finalizer first.
            // Finalizer is applied first, as the operator might be shut down and restarted
            // at any time, leaving subresources in intermediate state. This prevents leaks on
            // the `Bionic` resource deletion.

            // Apply the finalizer first. If that fails, the `?` operator invokes automatic conversion
            // of `kube::Error` to the `Error` defined in this crate.
            finalizer::add(client.clone(), &name, &namespace).await?;
            // Invoke creation of a Kubernetes built-in resource named deployment with `n` echo service pods.
            postgres::deploy(client.clone(), &name, bionic.spec.clone(), &namespace).await?;
            bionic::deploy(client.clone(), &name, bionic.spec.clone(), &namespace).await?;
            keycloak::deploy(client.clone(), &name, bionic.spec.clone(), &namespace).await?;
            envoy::deploy(client.clone(), &name, bionic.spec.clone(), &namespace).await?;
            oauth2_proxy::deploy(client.clone(), &name, bionic.spec.clone(), &namespace).await?;
            Ok(Action::requeue(Duration::from_secs(10)))
        }
        BionicAction::Delete => {
            // Deletes any subresources related to this `Bionic` resources. If and only if all subresources
            // are deleted, the finalizer is removed and Kubernetes is free to remove the `Bionic` resource.

            //First, delete the deployment. If there is any error deleting the deployment, it is
            // automatically converted into `Error` defined in this crate and the reconciliation is ended
            // with that error.
            // Note: A more advanced implementation would check for the Deployment's existence.
            keycloak::delete(client.clone(), &name, &namespace).await?;
            envoy::delete(client.clone(), &name, &namespace).await?;
            oauth2_proxy::delete(client.clone(), &name, &namespace).await?;
            bionic::delete(client.clone(), &name, &namespace).await?;
            postgres::delete(client.clone(), &name, &namespace).await?;

            // Once the deployment is successfully removed, remove the finalizer to make it possible
            // for Kubernetes to delete the `Bionic` resource.
            finalizer::delete(client, &name, &namespace).await?;
            Ok(Action::await_change()) // Makes no sense to delete after a successful delete, as the resource is gone
        }
        // The resource is already in desired state, do nothing and re-check after 10 seconds
        BionicAction::NoOp => Ok(Action::requeue(Duration::from_secs(10))),
    }
}

/// Resources arrives into reconciliation queue in a certain state. This function looks at
/// the state of given `Bionic` resource and decides which actions needs to be performed.
/// The finite set of possible actions is represented by the `BionicAction` enum.
///
/// # Arguments
/// - `echo`: A reference to `Bionic` being reconciled to decide next action upon.
fn determine_action(bionic: &Bionic) -> BionicAction {
    if bionic.meta().deletion_timestamp.is_some() {
        BionicAction::Delete
    } else if bionic
        .meta()
        .finalizers
        .as_ref()
        .map_or(true, |finalizers| finalizers.is_empty())
    {
        BionicAction::Create
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
