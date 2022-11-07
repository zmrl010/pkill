use cli::parse_args;
use lib::{pkill, Result};

fn main() -> Result<()> {
    let args = parse_args();
    pkill(args.targets)
}
