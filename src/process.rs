use sysinfo::{Process, SystemExt};

use crate::cli::ProcessQuery;

/// Search for processes based on `query`
///
/// # Returns
///
/// A vector containing a number of processes based on the `query`
///
/// * [`ProcessQuery::Pid`] - A single process if one was found
/// * [`ProcessQuery::Name`] - All processes with a name containing the query string
///
/// If no results were found, an empty vector will be returned
pub fn search<'a, S: SystemExt>(sys: &'a S, query: &'a ProcessQuery) -> Vec<&'a Process> {
    match query {
        ProcessQuery::Pid(pid) => sys
            .process(*pid)
            .map_or_else(|| Vec::new(), |process| vec![process]),
        ProcessQuery::Name(name) => sys.processes_by_name(&name).collect::<Vec<&Process>>(),
    }
}
