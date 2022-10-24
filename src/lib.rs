pub mod cli;

use sysinfo::{Pid, ProcessExt, System, SystemExt};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PKillError {
    #[error("unable to find process with pid {0}")]
    PidNotFound(Pid),
    #[error("unknown pkill error")]
    Unknown,
}

pub type Result<T, E = PKillError> = std::result::Result<T, E>;

pub fn run(args: cli::Args) -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    if let Some(pid) = args.pid {
        let pid = Pid::from(pid);
        if let Some(process) = sys.process(pid) {
            process.kill();
        } else {
            return Err(PKillError::PidNotFound(pid));
        }
    };

    Ok(())
}
