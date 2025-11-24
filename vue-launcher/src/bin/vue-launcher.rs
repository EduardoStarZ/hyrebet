use std::{path::Path, process::Command};

fn main() {
    let path : &Path = Path::new("restless");

    let _ = Command::new("npm")
        .arg("run")
        .arg("dev")
        .current_dir(path)
        .output();
}
