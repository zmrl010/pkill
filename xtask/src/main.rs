mod cli;
mod commands;
mod dirs;

use cli::Command;

fn main() -> anyhow::Result<()> {
    let args = cli::parse_args();
    match args.command {
        Command::Dist => commands::dist::exec(),
        Command::Ci => commands::ci::exec(),
    }
}
