use sysinfo::{Process, ProcessRefreshKind, RefreshKind, System, SystemExt};

use crate::cli::ProcessQuery;

/// Initialize [`System`] instance with only process information loaded
pub fn init_system() -> System {
    let refresh_kind = RefreshKind::new().with_processes(ProcessRefreshKind::everything());
    System::new_with_specifics(refresh_kind)
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
