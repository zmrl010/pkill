use pkill_cli::parse_args;
use pkill_lib::pkill;

fn main() -> anyhow::Result<()> {
    let args = parse_args();
    pkill(args.targets)
}
