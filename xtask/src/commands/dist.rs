use crate::util;
use anyhow::{bail, Context};
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

fn dist_dir<P: AsRef<Path>>(root: P) -> PathBuf {
    let mut dir = root.as_ref().to_path_buf();
    dir.push("target");
    dir.push("dist");
    dir
}

/// Build shell completions using [`clap_complete`]
///
/// [`clap_complete`]: https://docs.rs/clap_complete
fn dist_shell_completions(outdir: &Path) -> anyhow::Result<()> {
    let mut cmd = pkill_command();
    for shell in Shell::value_variants() {
        generate_to(*shell, &mut cmd, "pkill", &outdir)?;
    }

    Ok(())
}

/// Build manpages using [`clap_mangen`]
///
/// [`clap_mangen`]: https://docs.rs/clap_mangen
fn dist_manpages(out_file: &Path) -> anyhow::Result<()> {
    let cmd = pkill_command();
    let mut file = File::create(out_file)?;
    Man::new(cmd).render(&mut file)?;

    Ok(())
}

fn dist_binary(root_dir: &Path) -> anyhow::Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(root_dir)
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        bail!("cargo build failed")
    }

    let release_dir = {
        let mut release_dir = root_dir.join("target");
        release_dir.push("release");
        release_dir
    };
    let dist_dir = dist_dir(&root_dir);

    fs::copy(&release_dir.join("pkill.exe"), &dist_dir.join("pkill.exe")).with_context(|| {
        format!(
            "failed to copy from {} to {}",
            release_dir.display(),
            dist_dir.display()
        )
    })?;

    Ok(())
}

/// Run `dist` command
pub fn exec() -> anyhow::Result<()> {
    let root_dir = util::project_root();
    let dist_dir = dist_dir(&root_dir);

    // attempt to clear the dist directory before building
    let _ = fs::remove_dir_all(&dist_dir);
    fs::create_dir_all(&dist_dir)
        .with_context(|| format!("failed to create dir `{}`", dist_dir.display()))?;

    dist_binary(&root_dir)?;

    dist_shell_completions(&dist_dir.join("completions"))?;
    dist_manpages(&dist_dir.join("man"))?;

    Ok(())
}
