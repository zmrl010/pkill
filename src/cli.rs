pub use clap::Parser;
use sysinfo::Pid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// kill processes containing [name]
    #[arg(short, long)]
    pub name: Option<String>,
    /// kill process by [pid]
    #[arg(short, long)]
    pub pid: Option<Pid>,
}
