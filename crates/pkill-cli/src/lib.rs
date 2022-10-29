pub use clap::Parser;
use pkill_lib::ProcessQuery;

/// Simple tool to kill processes
#[derive(Parser, Debug)]
#[command(name = "pkill", author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// target processes to kill
    #[arg(name = "pid|name")]
    pub targets: Vec<ProcessQuery>,
}

/// Parse command-line args
pub fn parse_args() -> CommandLineArgs {
    CommandLineArgs::parse()
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CommandLineArgs::command().debug_assert()
}
