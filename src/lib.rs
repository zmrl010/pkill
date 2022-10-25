pub mod cli;
pub mod process;

pub use anyhow::Result;
use cli::CommandLineArgs;

/// Launch the application
pub fn run(args: CommandLineArgs) -> Result<()> {
    let sys = process::init_system();

    match (args.pid, args.name) {
        (Some(pid), None) => process::kill_process_by_id(sys, pid),
        (None, Some(name)) => process::kill_processes_by_name(sys, &name),
        _ => unreachable!(), // clap validates this
    }?;

    Ok(())
}
