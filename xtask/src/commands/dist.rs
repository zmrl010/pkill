use crate::util;
use anyhow::bail;
use clap::{self, CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use pkill_cli;
use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
    process::Command,
};

fn pkill_command() -> clap::Command {
    pkill_cli::CommandLineArgs::command()
}

pub fn dist_dir() -> PathBuf {
    util::project_root().join("target/dist")
}

/// Build shell completions using [`clap_complete`]
///
/// [`clap_complete`]: https://docs.rs/clap_complete
fn dist_shell_completions(outdir: &Path) -> anyhow::Result<()> {
    let mut cmd = pkill_command();
    let shells = Shell::value_variants();

    for shell in shells {
        generate_to(*shell, &mut cmd, "pkill", &outdir)?;
    }

    Ok(())
}

/// Build manpages using [`clap_mangen`]
///
/// [`clap_mangen`]: https://docs.rs/clap_mangen
fn dist_manpages(outdir: &Path) -> anyhow::Result<()> {
    let cmd = pkill_command();

    let file = Path::new(outdir).join("man");
    let mut file = File::create(&file)?;

    let man = Man::new(cmd);
    man.render(&mut file)?;

    Ok(())
}

fn dist_binary(outdir: &Path) -> anyhow::Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(util::project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        bail!("cargo build failed")
    }

    Ok(())
}

/// Run `dist` command
pub fn exec(outdir: PathBuf) -> anyhow::Result<()> {
    dist_binary(&outdir)?;

    // Create `target/assets/` folder.
    let path = outdir.ancestors().nth(4).unwrap().join("assets");
    fs::create_dir_all(&path)?;

    dist_shell_completions(&path)?;
    dist_manpages(&path)?;

    Ok(())
}
