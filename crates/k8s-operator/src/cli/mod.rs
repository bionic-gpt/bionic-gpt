pub mod apply;
pub mod install;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub struct CloudflareInstaller {
    /// The cloudflare tuneel token
    #[arg(long)]
    pub token: String,
    /// The tunnel name i.e. bionic-gpt
    #[arg(long, default_value = "bionic-gpt")]
    pub name: String,
    /// In which namespace shall we install Bionic
    #[arg(long, default_value = "bionic-gpt")]
    pub namespace: String,
}

#[derive(Parser)]
pub struct Installer {
    /// Run a cut down version of Bionic for integration testing
    #[arg(long, default_value_t = false)]
    testing: bool,
    /// Don't install the operator
    #[arg(long, default_value_t = false)]
    no_operator: bool,
    /// Install ingress
    #[arg(long, default_value_t = false)]
    disable_ingress: bool,
    /// The setup needed for development. See CONTRIBUTING.md in the main project.
    #[arg(long, default_value_t = false)]
    development: bool,
    /// In which namespace shall we install Bionic
    #[arg(long, default_value = "bionic-gpt")]
    namespace: String,
    /// In which namespace shall we install Bionic
    #[arg(long, default_value = "bionic-system")]
    operator_namespace: String,
    /// The number of Bionic replicas
    #[arg(long, default_value_t = 1)]
    replicas: i32,
    /// Install A GPU based inference engine?
    #[arg(long, default_value_t = false)]
    gpu: bool,
    /// Are we a saas
    #[arg(long, default_value_t = false)]
    saas: bool,
    /// Install pgAdmin?
    #[arg(long, default_value_t = false)]
    pgadmin: bool,
    /// Install Observability?
    #[arg(long, default_value_t = false)]
    observability: bool,
    /// The hostname we are deploying on. By default use the local ip address
    #[arg(long)]
    hostname_url: Option<String>,
    /// Disk size for the Bionic Postgres database (in GB)
    #[arg(long, default_value_t = 1)]
    bionic_db_disk_size: i32,
    /// Disk size for the Keycloak Postgres database (in GB)
    #[arg(long, default_value_t = 1)]
    keycloak_db_disk_size: i32,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install Bionic into Kubernetes
    Install(Installer),
    /// Run the Bionic Kubernetes Operator
    Operator {},
    /// Run the Bionic Kubernetes Operator
    Cloudflare(CloudflareInstaller),
}
