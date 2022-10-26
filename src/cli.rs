// # Command-line Interface for `pkill`

use std::{num::ParseIntError, str::FromStr};

/// required parser for [`clap`]
pub use clap::Parser;
use sysinfo::Pid;

/// Types of process queries
#[derive(Debug, Clone)]
pub enum ProcessQuery {
    Pid(Pid),
    Name(String),
}

impl FromStr for ProcessQuery {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<ProcessQuery, Self::Err> {
        let value = s
            .parse::<Pid>()
            .map_or(ProcessQuery::Name(s.to_string()), |pid| {
                ProcessQuery::Pid(pid)
            });
        Ok(value)
    }
}

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
