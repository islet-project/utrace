use crate::utils::expand_tilde;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn run(path: &Path) {
    let target_dir = expand_tilde(path);
    let target_dir = fs::canonicalize(target_dir).expect("Failed to get the absosulte path.");
    env::set_current_dir(target_dir).expect("Failed to change dir to plugin.");

    let out_dir = utrace_common::config::out_dir();
    let path = Path::new(&out_dir);
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
        fs::create_dir_all(path).unwrap();
    } else {
        fs::create_dir_all(path).unwrap();
    }

    Command::new("cargo")
        .arg("clean")
        .status()
        .expect("Failed to clean the package.");

    Command::new("rustup")
        .arg("run")
        .arg("utrace")
        .arg("cargo")
        .arg("build")
        .status()
        .expect("Failed to utrace.");
}
