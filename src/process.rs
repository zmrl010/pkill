use crate::{error::PKillError, result::Result};
use sysinfo::{Pid, ProcessExt, System, SystemExt};

/// Kill a single process matching [pid]
///
/// # Examples
///
/// ```no_run
/// use sysinfo::{System, SystemExt};
/// use pkill::Result;
///
/// fn main() -> Result<()> {
///     let mut sys = System::new_all();
///     sys.refresh_all();
///
///     kill_processes_by_id(sys, Pid::from(123))
/// }
/// ```
pub fn kill_process_by_id(sys: System, pid: Pid) -> Result<()> {
    if let Some(process) = sys.process(pid) {
        process.kill();
        return Ok(());
    }

    Err(PKillError::PidNotFound(pid))
}

/// Kill all processes containing the given [name]
///
/// # Examples
///
/// ```no_run
/// use sysinfo::{System, SystemExt};
/// use pkill::Result;
///
/// fn main() -> Result<()> {
///     let mut sys = System::new_all();
///     sys.refresh_all();
///
///     kill_processes_by_name(sys, String::from("node"))
/// }
/// ```
pub fn kill_processes_by_name(sys: System, name: &String) -> Result<()> {
    let mut processes = sys.processes_by_name(name.as_str()).peekable();
    if processes.peek().is_none() {
        return Err(PKillError::ProcessNameNotFound(name.clone()));
    }

    for process in processes {
        process.kill();
    }

    Ok(())
}
