use sysinfo::Pid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PKillError {
    #[error("unable to find process with pid {0}")]
    PidNotFound(Pid),
    #[error("unable to find process containing the name {0}")]
    ProcessNameNotFound(String),
    #[error("unknown pkill error")]
    Unknown,
}
