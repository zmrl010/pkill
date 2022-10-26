pub mod cli;
pub mod process;

pub use anyhow::Result;
use cli::CommandLineArgs;
use process::ProcessQuery;
use sysinfo::ProcessExt;

/// Iterate `targets` to find and kill any processes that are found
pub fn pkill(targets: Vec<ProcessQuery>) -> Result<()> {
    let sys = process::init_system();

    let processes = targets
        .iter()
        .flat_map(|query| process::search(&sys, &query));

    for process in processes {
        println!("killing process {}", process.pid());
        let signal_sent = process.kill();
        if !signal_sent {
            eprintln!("kill signal failed to send")
        }
    }

    Ok(())
}

/// Launch `pkill`
pub fn run(args: CommandLineArgs) -> Result<()> {
    pkill(args.targets)
}
