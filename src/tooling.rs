use std::process::Command;

pub fn run_fmt_check(path: &str) -> bool {
    let status = Command::new("cargo")
        .arg("fmt")
        .arg("--check")
        .current_dir(path)
        .status()
        .expect("Failed to run cargo fmt");

    status.success()
}

pub fn run_clippy_check(path: &str) -> bool {
    let status = Command::new("cargo")
        .arg("clippy")
        .arg("--quiet")
        .arg("--")
        .arg("-Dwarnings") // fail on warnings
        .current_dir(path)
        .status()
        .expect("Failed to run cargo clippy");

    status.success()
}
