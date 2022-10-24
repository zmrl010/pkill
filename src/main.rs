use pkill::cli::{Args, Parser};

fn main() -> pkill::Result<()> {
    let args = Args::parse();
    pkill::run(args)
}
