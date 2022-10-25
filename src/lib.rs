pub mod cli;
mod error;
pub mod process;
mod result;

use cli::CommandLineArgs;
pub use result::Result;

use sysinfo::{System, SystemExt};

pub fn run(args: CommandLineArgs) -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    match (args.pid, args.name) {
        (Some(pid), None) => process::kill_process_by_id(sys, pid),
        (None, Some(name)) => process::kill_processes_by_name(sys, &name),
        _ => unreachable!(),
    }?;

    Ok(())
}
