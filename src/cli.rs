#![deny(missing_docs)]
//! # Command-line Interface for `pkill`

pub use clap::Parser;

use crate::process::ProcessQuery;

/// Simple tool to kill processes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// target processes to kill
    #[arg(name = "pid|name")]
    pub targets: Vec<ProcessQuery>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CommandLineArgs::command().debug_assert()
}
