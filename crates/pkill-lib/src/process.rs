use std::{num::ParseIntError, str::FromStr};

use sysinfo::{Pid, Process, SystemExt};

/// Search parameters for finding processes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryParam {
    /// Query for process with pid
    Pid(Pid),
    /// Query for processes with name containing
    Name(String),
}

impl FromStr for QueryParam {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<QueryParam, Self::Err> {
        let value = s.parse::<Pid>().map_or_else(
            |_| QueryParam::Name(s.to_string()),
            |pid| QueryParam::Pid(pid),
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
pub fn search<'a, S: SystemExt>(sys: &'a S, query: &'a QueryParam) -> Vec<&'a Process> {
    match query {
        QueryParam::Pid(pid) => sys
            .process(*pid)
            .map_or(Default::default(), |process| vec![process]),
        QueryParam::Name(name) => sys.processes_by_name(&name).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_query_should_parse_string() -> anyhow::Result<()> {
        let result: QueryParam = "argument".parse()?;

        assert_eq!(result, QueryParam::Name("argument".to_string()));

        Ok(())
    }

    #[test]
    fn process_query_should_parse_pid() -> anyhow::Result<()> {
        let result: QueryParam = "1234".parse()?;

        assert_eq!(result, QueryParam::Pid(Pid::from(1234)));

        Ok(())
    }
}
