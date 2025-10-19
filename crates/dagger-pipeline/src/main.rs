mod args;
mod pipeline;

use args::Args;
use clap::Parser;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    pipeline::run(args).await
}
