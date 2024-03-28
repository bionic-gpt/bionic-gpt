mod error;
mod install;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install Bionic
    Install {
        /// Install K9s
        #[arg(long)]
        k9s: bool,
        /// Install a K3s Kubernetes Node
        #[arg(long)]
        k3s: bool,
    },
    /// Upgrade Bionic (Not Yet Complete)
    Upgrade {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { k9s, k3s } => {
            dbg!(k9s, k3s);
            install::install("bionic-gpt").await?;
        }
        Commands::Upgrade {} => {
            println!("Not Implemented");
        }
    }

    Ok(())
}
