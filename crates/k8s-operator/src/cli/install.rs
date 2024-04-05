use crate::error::Error;
use crate::operator::crd::Bionic;
use crate::operator::crd::BionicSpec;
use anyhow::{bail, Result};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::api::core::v1::ServiceAccount;
use k8s_openapi::api::rbac::v1::ClusterRole;
use k8s_openapi::api::rbac::v1::ClusterRoleBinding;
use k8s_openapi::api::rbac::v1::PolicyRule;
use k8s_openapi::api::rbac::v1::RoleRef;
use k8s_openapi::api::rbac::v1::Subject;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::api::ObjectMeta;
use kube::Api;
use kube::Client;
use kube::CustomResourceExt;
use kube_runtime::conditions;
use kube_runtime::wait::await_condition;
use kube_runtime::wait::Condition;
use local_ip_address::local_ip;
use serde_json::json;

const BIONIC_OPERATOR_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt-k8s-operator";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const CNPG_YAML: &str = include_str!("../../config/cnpg-1.22.1.yaml");

pub async fn install(installer: &crate::cli::Installer) -> Result<()> {
    let client = Client::try_default().await?;

    // Define the API object for Namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());

    let ns = namespaces.get(&installer.namespace).await;

    if ns.is_ok() {
        bail!("Namespace already exists");
    } else {
        install_postgres_operator(&client).await?;
        create_namespace(&installer.namespace, namespaces).await?;
        create_crd(&client).await?;
        create_bionic(&client, installer).await?;
        create_roles(&client, installer).await?;
        if !installer.no_operator {
            create_bionic_operator(&client, &installer.namespace).await?;
        }
        let my_local_ip = local_ip().unwrap();
        println!("When ready you can access bionic on http://{}", my_local_ip);
    }
    Ok(())
}

async fn install_postgres_operator(client: &Client) -> Result<()> {
    super::apply::apply(client, CNPG_YAML, None).await?;

    fn is_deployment_available() -> impl Condition<Deployment> {
        |obj: Option<&Deployment>| {
            if let Some(deployment) = &obj {
                if let Some(status) = &deployment.status {
                    if let Some(phase) = &status.available_replicas {
                        return phase >= &1;
                    }
                }
            }
            false
        }
    }

    println!("Waiting for Cloud Native Postgres Controller Manager");
    let deploys: Api<Deployment> = Api::namespaced(client.clone(), "cnpg-system");
    let establish = await_condition(
        deploys,
        "cnpg-controller-manager",
        is_deployment_available(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(120), establish)
        .await
        .unwrap();

    Ok(())
}

async fn create_bionic_operator(client: &Client, namespace: &str) -> Result<()> {
    let app_labels = serde_json::json!({
        "app": "bionic-gpt-operator",
    });

    let deployment = serde_json::from_value(serde_json::json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": "bionic-gpt-operator-deployment",
            "namespace": namespace
        },
        "spec": {
            "replicas": 1,
            "selector": {
                "matchLabels": app_labels
            },
            "template": {
                "metadata": {
                    "labels": app_labels
                },
                "spec": {
                    "serviceAccountName": "bionic-gpt-operator-service-account",
                    "containers": json!([{
                        "name": "bionic-gpt-operator",
                        "image": format!("{}:{}", BIONIC_OPERATOR_IMAGE, VERSION)
                    }]),
                }
            }
        }
    }))?;

    // Create the deployment defined above
    let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    deployment_api
        .create(&Default::default(), &deployment)
        .await?;

    Ok(())
}

async fn create_roles(client: &Client, installer: &super::Installer) -> Result<()> {
    let sa_api: Api<ServiceAccount> = Api::namespaced(client.clone(), &installer.namespace);
    let service_account = ServiceAccount {
        metadata: ObjectMeta {
            name: Some("bionic-gpt-operator-service-account".to_string()),
            namespace: Some(installer.namespace.clone()),
            ..Default::default()
        },
        ..Default::default()
    };
    sa_api.create(&Default::default(), &service_account).await?;
    let role_api: Api<ClusterRole> = Api::all(client.clone());
    let role = ClusterRole {
        metadata: ObjectMeta {
            name: Some("bionic-gpt-operator-cluster-role".to_string()),
            ..Default::default()
        },
        rules: Some(vec![PolicyRule {
            api_groups: Some(vec!["*".to_string()]),
            resources: Some(vec!["*".to_string()]),
            verbs: vec!["*".to_string()],
            ..Default::default()
        }]),
        ..Default::default()
    };
    role_api.create(&Default::default(), &role).await?;
    let role_binding_api: Api<ClusterRoleBinding> = Api::all(client.clone());
    let role_binding = ClusterRoleBinding {
        metadata: ObjectMeta {
            name: Some("bionic-gpt-operator-cluster-role-binding".to_string()),
            ..Default::default()
        },
        role_ref: RoleRef {
            api_group: "rbac.authorization.k8s.io".to_string(),
            kind: "ClusterRole".to_string(),
            name: "bionic-gpt-operator-cluster-role".to_string(),
        },
        subjects: Some(vec![Subject {
            kind: "ServiceAccount".to_string(),
            name: "bionic-gpt-operator-service-account".to_string(),
            namespace: Some(installer.namespace.clone()),
            ..Default::default()
        }]),
    };
    role_binding_api
        .create(&Default::default(), &role_binding)
        .await?;
    Ok(())
}

async fn create_bionic(client: &Client, installer: &super::Installer) -> Result<()> {
    let hostname_url = if let Some(hostname_url) = &installer.hostname_url {
        hostname_url.into()
    } else {
        let my_local_ip = local_ip().unwrap();
        format!("http://{:?}", my_local_ip)
    };
    let bionic_api: Api<Bionic> = Api::namespaced(client.clone(), &installer.namespace);
    let bionic = Bionic::new(
        "bionic-gpt",
        BionicSpec {
            replicas: 1,
            version: VERSION.into(),
            gpu: Some(installer.gpu),
            pgadmin: Some(installer.pgadmin),
            testing: Some(installer.testing),
            development: Some(installer.development),
            hostname_url,
            hash_bionicgpt: "".to_string(),
            hash_bionicgpt_pipeline_job: "".to_string(),
            hash_bionicgpt_db_migrations: "".to_string(),
        },
    );
    bionic_api.create(&Default::default(), &bionic).await?;
    Ok(())
}

async fn create_crd(client: &Client) -> Result<(), Error> {
    let crd = Bionic::crd();
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    crds.create(&Default::default(), &crd).await?;

    println!("Waiting for Bionic CRD");
    let establish = await_condition(
        crds,
        "bionics.bionic-gpt.com",
        conditions::is_crd_established(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish)
        .await
        .unwrap();
    Ok(())
}

async fn create_namespace(namespace: &str, namespaces: Api<Namespace>) -> Result<()> {
    let new_namespace = Namespace {
        metadata: ObjectMeta {
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
