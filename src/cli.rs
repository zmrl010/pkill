use clap::ArgGroup;
pub use clap::Parser;
use sysinfo::Pid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("query")
        .required(true)
        .args(["name", "pid"]),
))]
pub struct CommandLineArgs {
    /// kill processes containing [name]
    pub name: Option<String>,
    /// kill process with matching [pid]
    pub pid: Option<Pid>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CommandLineArgs::command().debug_assert()
}
