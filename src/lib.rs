pub mod cli;

use sysinfo::{Pid, ProcessExt, System, SystemExt};
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

pub type Result<T, E = PKillError> = std::result::Result<T, E>;

fn kill_process_by_id(sys: System, pid: Pid) -> Result<()> {
    if let Some(process) = sys.process(pid) {
        process.kill();
        return Ok(());
    }

    Err(PKillError::PidNotFound(pid))
}

fn kill_processes_by_name(sys: System, name: &String) -> Result<()> {
    let mut processes = sys.processes_by_name(name.as_str()).peekable();
    if processes.peek().is_none() {
        return Err(PKillError::ProcessNameNotFound(
            name.clone(),
        ));
    }

    for process in processes {
        process.kill();
    }

    Ok(())
}

pub fn run(args: cli::Args) -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    if let Some(pid) = args.pid {
        return kill_process_by_id(sys, pid);
    } 

    if let Some(name) = args.name {
        return kill_processes_by_name(sys, &name)
    }

    Ok(())
}
