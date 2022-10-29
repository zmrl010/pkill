mod cli;
mod commands;
mod util;

use cli::Command;

fn execute(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Dist { outdir } => commands::dist::exec(outdir),
        Command::Ci => commands::ci::exec(),
    }
}

fn main() -> anyhow::Result<()> {
    let args = cli::parse_args();
    execute(args.command)
}
