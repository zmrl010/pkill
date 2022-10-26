//! Pkill build scripts
//!
//! Based on https://github.com/sondr3/clap-man-example
use clap::CommandFactory;
use clap_mangen::Man;
use std::{
    env,
    fs::{self, File},
    io::{ErrorKind, Result},
    path::{Path, PathBuf},
};

mod cli {
    include!("src/cli.rs");
}

/// Print cargo instructions to be recognized during build
///
/// https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script
fn print_cargo_instructions() {
    let instructions = [
        "cargo:rerun-if-changed=src/cli.rs",
        "cargo:rerun-if-changed=man",
    ];

    for instr in instructions {
        println!("{}", instr);
    }
}

/// Build manpages using [`clap_mangen`]
///
/// [`clap_mangen`]: https://docs.rs/clap_mangen
fn build_manpages(outdir: &Path) -> Result<()> {
    let cmd = cli::CommandLineArgs::command();

    let file = Path::new(&outdir).with_file_name("man");
    let mut file = File::create(&file)?;

    let man = Man::new(cmd);
    man.render(&mut file)?;

    Ok(())
}

fn main() -> Result<()> {
    print_cargo_instructions();
    let outdir = env::var_os("OUT_DIR").ok_or_else(|| ErrorKind::NotFound)?;
    let outdir = PathBuf::from(outdir);

    // Create `target/assets/` folder.
    let out_path = PathBuf::from(outdir);
    let path = out_path.ancestors().nth(4).unwrap().join("assets");

    fs::create_dir_all(&path)?;

    build_manpages(&path)?;

    Ok(())
}
