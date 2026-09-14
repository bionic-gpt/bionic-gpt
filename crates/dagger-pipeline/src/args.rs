use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    /// Build and test for pull request validation (no publish).
    PullRequest,
    /// Build for main branch and publish all artifacts.
    All,
    /// Generate the combined eval mocks OpenAPI spec for local Mockoon testing.
    GenerateEvalMocksSpec,
    /// Build the Office tools image locally, optionally publishing it to GHCR.
    OfficeTools {
        /// Archipelago branch, tag, or commit to package.
        #[arg(long, default_value = "main")]
        archipelago_ref: String,
        /// Local image tag. Defaults to bionic-gpt-office-tools:local.
        #[arg(long)]
        tag: Option<String>,
        /// Publish immutable upstream-SHA and latest tags instead of exporting locally.
        #[arg(long)]
        publish: bool,
    },
}
