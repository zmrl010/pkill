use crate::dirs;
use anyhow::{bail, Context};
use clap::{self, CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use pkill_cli;
use std::{
    env,
    fs::{self, File},
    path::Path,
    process::Command,
};

/// Binary executable's name. Should match output binary
const BIN_FILE_NAME: &str = if cfg!(windows) { "pkill.exe" } else { "pkill" };

fn pkill_command() -> clap::Command {
    pkill_cli::CommandLineArgs::command()
}

fn create_dir_all<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    fs::create_dir_all(&path)
        .with_context(|| format!("failed to create dir `{}`", path.as_ref().display()))?;
    Ok(())
}

/// Build shell completions using [`clap_complete`]
///
/// [`clap_complete`]: https://docs.rs/clap_complete
fn dist_shell_completions(outdir: &Path) -> anyhow::Result<()> {
    let mut cmd = pkill_command();
    for shell in Shell::value_variants() {
        generate_to(*shell, &mut cmd, "pkill", &outdir).with_context(|| {
            format!(
                "failed to save completions for `{}` to `{}`",
                shell.to_string(),
                outdir.display()
            )
        })?;
    }

    Ok(())
}

/// Build manpages using [`clap_mangen`]
///
/// [`clap_mangen`]: https://docs.rs/clap_mangen
fn dist_manpages(outdir: &Path) -> anyhow::Result<()> {
    let cmd = pkill_command();
    let out_file = outdir.join("man");
    let mut file = File::create(out_file)
        .with_context(|| format!("failed to create file `{}`", outdir.display()))?;
    Man::new(cmd).render(&mut file)?;

    Ok(())
}

fn dist_binary() -> anyhow::Result<()> {
    let current_dir = dirs::project_root();
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(&current_dir)
        .args(&["build", "--release"])
        .status()
        .with_context(|| {
            format!(
                "failed executing `build --release` for `{}`",
                current_dir.display()
            )
        })?;

    if !status.success() {
        bail!("cargo build failed")
    }

    let release_dir = dirs::release_dir();
    let dist_dir = dirs::dist_dir();
    fs::copy(
        &release_dir.join(BIN_FILE_NAME),
        &dist_dir.join(BIN_FILE_NAME),
    )
    .with_context(|| {
        format!(
            "failed to copy from `{}` to `{}`",
            release_dir.display(),
            dist_dir.display()
        )
    })?;

    Ok(())
}

/// Run `dist` command
pub fn exec() -> anyhow::Result<()> {
    let dist_dir = dirs::dist_dir();
    let completions_dir = dist_dir.join("completions");
    let _ = fs::remove_dir_all(&dist_dir);
    // creating the completions dir will create the dist dir
    // create_dir_all(&dist_dir)?;
    create_dir_all(&completions_dir)?;

    dist_binary()?;

    dist_shell_completions(&dist_dir)?;
    dist_manpages(&dist_dir)?;

    Ok(())
}
