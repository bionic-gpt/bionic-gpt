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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { k9s, k3s } => {
            println!("{} {}", k3s, k9s);
        }
        Commands::Upgrade {} => {
            println!("Not Implemented");
        }
    }
}
