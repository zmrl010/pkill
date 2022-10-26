pub mod cli;
pub mod process;

pub use anyhow::Result;
use cli::{CommandLineArgs, ProcessQuery};
use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

/// Initialize [`System`] instance with only process information loaded
fn init_system() -> System {
    let refresh_kind = RefreshKind::new().with_processes(ProcessRefreshKind::everything());
    let mut sys = System::new_with_specifics(refresh_kind);
    sys.refresh_processes();
    sys
}

/// Iterate `targets` to find and kill any processes that are found
pub fn pkill(targets: Vec<ProcessQuery>) -> Result<()> {
    let sys = init_system();

    let processes = targets
        .iter()
        .flat_map(|query| process::search(&sys, &query));

    for process in processes {
        println!("killing process {}", process.pid());
        if !process.kill() {
            eprintln!("kill signal failed to send")
        }
    }

    Ok(())
}

/// Launch `pkill`
pub fn run(args: CommandLineArgs) -> Result<()> {
    pkill(args.targets)
}
