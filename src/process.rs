use std::{num::ParseIntError, str::FromStr};

use crate::Result;
use sysinfo::{Pid, Process, ProcessRefreshKind, RefreshKind, System, SystemExt};

/// Initialize [`System`] instance with only process information loaded
pub fn init_system() -> System {
    let refresh_kind = RefreshKind::new().with_processes(ProcessRefreshKind::everything());
    System::new_with_specifics(refresh_kind)
}

/// Types of process queries
#[derive(Debug, Clone)]
pub enum ProcessQuery {
    Pid(Pid),
    Name(String),
}

/// Search for processes based on `query`
///
/// # Returns
///
/// A vector containing...
///
/// * [`ProcessQuery::Pid`] - A single process if one was found or empty
/// * [`ProcessQuery::Name`] - All processes with a name containing the query value
pub fn search<'a, S: SystemExt>(sys: &'a S, query: &'a ProcessQuery) -> Vec<&'a Process> {
    match query {
        ProcessQuery::Pid(pid) => sys
            .process(*pid)
            .map_or_else(|| Vec::new(), |process| vec![process]),
        ProcessQuery::Name(name) => sys.processes_by_name(&name).collect::<Vec<&Process>>(),
    }
}

impl FromStr for ProcessQuery {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<ProcessQuery, ParseIntError> {
        let value = s
            .parse::<Pid>()
            .map_or(ProcessQuery::Name(s.to_string()), |pid| {
                ProcessQuery::Pid(pid)
            });
        Ok(value)
    }
}
