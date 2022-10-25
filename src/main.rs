use pkill::cli::{CommandLineArgs, Parser};

fn main() -> pkill::Result<()> {
    let args = CommandLineArgs::parse();
    pkill::run(args)
}
