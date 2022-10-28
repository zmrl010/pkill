use std::{num::ParseIntError, str::FromStr};

use sysinfo::{Pid, Process, SystemExt};

/// Process searching inputs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessQuery {
    /// Query for process with pid
    Pid(Pid),
    /// Query for processes with name containing
    Name(String),
}

impl FromStr for ProcessQuery {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<ProcessQuery, Self::Err> {
        let value = s.parse::<Pid>().map_or_else(
            |_| ProcessQuery::Name(s.to_string()),
            |pid| ProcessQuery::Pid(pid),
        );
        Ok(value)
    }
}

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
            .map_or(Default::default(), |process| vec![process]),
        ProcessQuery::Name(name) => sys.processes_by_name(&name).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_query_should_parse_string() -> anyhow::Result<()> {
        let result: ProcessQuery = "argument".parse()?;

        assert_eq!(result, ProcessQuery::Name("argument".to_string()));

        Ok(())
    }

    #[test]
    fn process_query_should_parse_pid() -> anyhow::Result<()> {
        let result: ProcessQuery = "1234".parse()?;

        assert_eq!(result, ProcessQuery::Pid(Pid::from(1234)));

        Ok(())
    }
}
