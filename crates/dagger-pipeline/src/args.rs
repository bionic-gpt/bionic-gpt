use clap::{Parser, Subcommand, ValueEnum};

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
    /// Build the CLI for the given operating system.
    BuildCli {
        #[arg(value_enum)]
        target: CliTarget,
    },
}

#[derive(ValueEnum, Clone, Copy)]
pub enum CliTarget {
    Linux,
    Macos,
    Windows,
}
