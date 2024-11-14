use crate::error::Error;
use crate::operator::crd::Bionic;
use crate::operator::crd::BionicSpec;
use anyhow::Result;
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
use kube::api::Patch;
use kube::api::PatchParams;
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
const NGINX_YAML: &str = include_str!("../../config/nginx-ingress.yaml");
const POSTGRES_SERVICE: &str = include_str!("../../config/postgres-service-dev.yaml");
const APPLICATION_SERVICE: &str = include_str!("../../config/bionic-service-dev.yaml");

pub async fn install(installer: &crate::cli::Installer) -> Result<()> {
    println!("Connecting to the cluster...");
    let client = Client::try_default().await?;
    println!("Connected");

    install_postgres_operator(&client).await?;
    if !installer.disable_ingress {
        install_nginx_operator(&client).await?;
    }
    create_namespace(&client, &installer.namespace).await?;
    create_namespace(&client, &installer.operator_namespace).await?;
    create_crd(&client).await?;
    create_bionic(&client, installer).await?;
    create_roles(&client, installer).await?;
    if !installer.no_operator {
        create_bionic_operator(&client, &installer.operator_namespace).await?;
    }

    if installer.development {
        // Open up the postgres port to the devcontainer
        println!("🚀 Mapping Postgres to port 30001");
        super::super::cli::apply::apply(&client, POSTGRES_SERVICE, Some(&installer.namespace))
            .await
            .unwrap();
        println!("🚀 Mapping Nginx to port 30000");
        super::apply::apply(&client, APPLICATION_SERVICE, Some(&installer.namespace)).await?;
    }
    let my_local_ip = local_ip().unwrap();
    println!("When ready you can access bionic on http://{}", my_local_ip);
    Ok(())
}

async fn install_nginx_operator(client: &Client) -> Result<()> {
    println!("Installing Nginx Ingress Operator");
    super::apply::apply(client, NGINX_YAML, None).await?;

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

    println!("Waiting for Nginx Operator to be Available");
    let deploys: Api<Deployment> = Api::namespaced(client.clone(), "ingress-nginx");
    let establish = await_condition(
        deploys,
        "ingress-nginx-controller",
        is_deployment_available(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(120), establish)
        .await
        .unwrap();

    Ok(())
}

async fn install_postgres_operator(client: &Client) -> Result<()> {
    println!("Installing Cloud Native Postgres Operator (CNPG)");
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
    println!("Installing the Bionic Operator into {}", namespace);
    let app_labels = serde_json::json!({
        "app": "bionic-gpt-operator",
    });

    let deployment = serde_json::json!({
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
    });

    // Create the deployment defined above
    let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    deployment_api
        .patch(
            "bionic-gpt-operator-deployment",
            &PatchParams::apply(crate::MANAGER),
            &Patch::Apply(deployment),
        )
        .await?;

    Ok(())
}

async fn create_roles(client: &Client, installer: &super::Installer) -> Result<()> {
    println!("Setting up roles");
    let sa_api: Api<ServiceAccount> =
        Api::namespaced(client.clone(), &installer.operator_namespace);
    let service_account = ServiceAccount {
        metadata: ObjectMeta {
            name: Some("bionic-gpt-operator-service-account".to_string()),
            namespace: Some(installer.operator_namespace.clone()),
            ..Default::default()
        },
        ..Default::default()
    };
    sa_api
        .patch(
            "bionic-gpt-operator-service-account",
            &PatchParams::apply(crate::MANAGER),
            &Patch::Apply(service_account),
        )
        .await?;
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
    role_api
        .patch(
            "bionic-gpt-operator-cluster-role",
            &PatchParams::apply(crate::MANAGER),
            &Patch::Apply(role),
        )
        .await?;

    // Now the cluster role
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
            namespace: Some(installer.operator_namespace.clone()),
            ..Default::default()
        }]),
    };
    role_binding_api
        .patch(
            "bionic-gpt-operator-cluster-role-binding",
            &PatchParams::apply(crate::MANAGER),
            &Patch::Apply(role_binding),
        )
        .await?;
    Ok(())
}

async fn create_bionic(client: &Client, installer: &super::Installer) -> Result<()> {
    println!("Installing Bionic Services into {}", &installer.namespace);
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
            saas: Some(installer.saas),
            disable_ingress: Some(installer.disable_ingress),
            pgadmin: Some(installer.pgadmin),
            observability: Some(installer.observability),
            testing: Some(installer.testing),
            development: Some(installer.development),
            hostname_url,
            hash_bionicgpt: "".to_string(),
            hash_bionicgpt_pipeline_job: "".to_string(),
            hash_bionicgpt_db_migrations: "".to_string(),
            bionic_db_disk_size: installer.bionic_db_disk_size,
            keycloak_db_disk_size: installer.keycloak_db_disk_size,
        },
    );
    bionic_api
        .patch(
            "bionic-gpt",
            &PatchParams::apply(crate::MANAGER),
            &Patch::Apply(bionic),
        )
        .await?;
    Ok(())
}

async fn create_crd(client: &Client) -> Result<(), Error> {
    println!("Installing Bionic CRD");
    let crd = Bionic::crd();
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    crds.patch(
        "bionics.bionic-gpt.com",
        &PatchParams::apply(crate::MANAGER),
        &Patch::Apply(crd),
    )
    .await?;

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

async fn create_namespace(client: &Client, namespace: &str) -> Result<Namespace> {
    println!("Creating namespace {}", namespace);
    // Define the API object for Namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());

    let new_namespace = Namespace {
        metadata: ObjectMeta {
            name: Some(namespace.to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let ns = namespaces
        .patch(
            namespace,
            &PatchParams::apply(crate::MANAGER),
            &Patch::Apply(new_namespace),
        )
        .await?;
    Ok(ns)
}
