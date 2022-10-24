pub use clap::Parser;

/// Simple CLI tool to kill processes
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    /// process ID of task to kill
    pub pid: Option<usize>,
}
