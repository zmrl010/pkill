#![deny(missing_docs)]
//! # Command-line Interface for `pkill`

use clap::ArgGroup;
pub use clap::Parser;
use sysinfo::Pid;

/// Simple tool to kill processes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("query")
        .required(true)
        .args(["name", "pid"]),
))]
pub struct CommandLineArgs {
    /// kill processes containing [name]
    pub name: Option<String>,
    /// kill process with matching [pid]
    pub pid: Option<Pid>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CommandLineArgs::command().debug_assert()
}
