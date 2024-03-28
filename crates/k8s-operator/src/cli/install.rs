use crate::error::Error;
use crate::operator::crd::Bionic;
use crate::operator::crd::BionicSpec;
use crate::services::deployment;
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
use local_ip_address::local_ip;

const BIONIC_OPERATOR_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt-k8s-operator";
const VERSION: &str = env!("CARGO_PKG_VERSION");

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
        create_bionic(&client, installer).await?;
        if !installer.development {
            create_bionic_operator(&client, installer, &installer.namespace).await?;
        }
    }
    Ok(())
}

async fn create_bionic_operator(
    client: &Client,
    installer: &super::Installer,
    namespace: &str,
) -> Result<(), Error> {
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

    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "bionic-operator".to_string(),
            image_name: format!("{}:{}", BIONIC_OPERATOR_IMAGE, VERSION),
            replicas: installer.replicas,
            port: 11434,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![],
            }),
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await?;

    Ok(())
}

async fn create_bionic(client: &Client, installer: &super::Installer) -> Result<(), Error> {
    let my_local_ip = local_ip().unwrap();
    let bionic_api: Api<Bionic> = Api::namespaced(client.clone(), &installer.namespace);
    let bionic = Bionic::new(
        "bionic",
        BionicSpec {
            replicas: 1,
            version: VERSION.into(),
            gpu: Some(installer.gpu),
            pgadmin: Some(installer.pgadmin),
            testing: Some(installer.testing),
            hostname_url: format!("{:?}", my_local_ip),
        },
    );
    bionic_api.create(&Default::default(), &bionic).await?;
    Ok(())
}

async fn create_crd(client: &Client) -> Result<(), Error> {
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    crds.create(&Default::default(), &Bionic::crd()).await?;
    Ok(())
}

async fn create_namespace(namespace: &str, namespaces: Api<Namespace>) -> Result<(), Error> {
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