mod cli;
mod error;
mod operator;
mod services;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Install {
            development,
            testing,
            namespace,
        } => {
            cli::install::install(development, testing, namespace).await?;
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
