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
        cli::Commands::Install { k9s, k3s } => {
            dbg!(k9s, k3s);
            cli::install::install("bionic-gpt").await?;
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
