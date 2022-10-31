//! # Directories
//!
//! This module defines helper functions that retrieve directory paths

use std::path::{Path, PathBuf};

pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

pub fn target_dir() -> PathBuf {
    let mut dir = project_root();
    dir.push("target");
    dir
}

pub fn release_dir() -> PathBuf {
    let mut dir = target_dir();
    dir.push("release");
    dir
}

pub fn dist_dir() -> PathBuf {
    let mut dir = target_dir();
    dir.push("dist");
    dir
}
