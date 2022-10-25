use crate::Result;
use anyhow::{anyhow, bail};
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

/// Kill a process by calling `process.kill` and returning a result
/// to signify whether the signal was successfully sent or not
fn kill(process: &Process) -> Result<()> {
    if !process.kill() {
        bail!(
            "there was an issue sending kill signal to process with pid `{}`",
            process.pid()
        );
    }
    Ok(())
}

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
        kill(process)?
    }

    Err(anyhow!("unable to find process with pid `{}`", pid))
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
pub fn kill_processes_by_name(sys: System, name: &str) -> Result<()> {
    let mut processes = sys.processes_by_name(name).peekable();
    if processes.peek().is_none() {
        bail!("unable to find processes with a name containing `{}`", name);
    }

    for process in processes {
        kill(process)?
    }

    Ok(())
}
