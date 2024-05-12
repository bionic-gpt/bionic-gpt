mod cli;
mod error;
mod operator;
mod services;
use anyhow::Result;
use clap::Parser;

const MANAGER: &str = "bionic-gpt-operator";

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Install(installer) => {
            cli::install::install(installer).await?;
        }
        cli::Commands::Upgrade {} => {
            println!("Not Implemented");
        }
        cli::Commands::Operator {} => {
            operator::operator().await?;
        }
    }

    Ok(())
}
