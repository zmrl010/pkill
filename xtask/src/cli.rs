use clap::{Parser, Subcommand};

/// pkill library scripts and tasks using xtask pattern
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Package pkill to produce a binary, shell completions, and manpages
    Dist,
    /// Run tests and check that any committed code adheres to quality standards
    Ci,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
