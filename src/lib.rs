pub mod cli;
mod error;
mod result;

use cli::CommandLineArgs;
pub use result::Result;

use error::PKillError;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

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
        return Err(PKillError::ProcessNameNotFound(name.clone()));
    }

    for process in processes {
        process.kill();
    }

    Ok(())
}

pub fn run(args: CommandLineArgs) -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    match (args.pid, args.name) {
        (Some(pid), None) => kill_process_by_id(sys, pid),
        (None, Some(name)) => kill_processes_by_name(sys, &name),
        _ => unreachable!(),
    }?;

    Ok(())
}
