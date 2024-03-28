pub mod install;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install Bionic
    Install {
        /// Install K9s
        #[arg(long)]
        k9s: bool,
        /// Install a K3s Kubernetes Node
        #[arg(long)]
        k3s: bool,
    },
    /// Run the Bionic Kubernetes Operator
    Operator {},
    /// Upgrade Bionic (Not Yet Complete)
    Upgrade {},
}
