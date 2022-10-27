use cli::parse_args;
use lib::pkill;

fn main() -> anyhow::Result<()> {
    let args = parse_args();
    pkill(args.targets)
}
