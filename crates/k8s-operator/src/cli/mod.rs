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
    /// Install Bionic into Kubernetes
    Install {
        /// Run a cut down version of Bionic with some services as HttpMock
        #[arg(long, default_value_t = false)]
        testing: bool,
        /// Don't install the operator
        #[arg(long, default_value_t = false)]
        development: bool,
        /// In which namespace shall we install Bionic
        #[arg(long, default_value = "bionic-gpt")]
        namespace: String,
    },
    /// Run the Bionic Kubernetes Operator
    Operator {},
    /// Upgrade Bionic (Not Yet Complete)
    Upgrade {},
}
