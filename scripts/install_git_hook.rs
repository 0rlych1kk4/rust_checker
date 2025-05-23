use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn main() {
    let hook_path = Path::new(".git/hooks/pre-commit");

    let script = r#"#!/bin/sh
echo "🔍 Running rust_checker before commit..."
cargo run -- . || {
    echo " rust_checker validation failed. Commit aborted."
    exit 1
}
"#;

    if let Some(parent) = hook_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create hooks directory");
    }

    let mut file = fs::File::create(&hook_path).expect("Failed to create pre-commit hook");
    file.write_all(script.as_bytes())
        .expect("Failed to write pre-commit script");

    let mut perms = fs::metadata(&hook_path).expect("Failed to read permissions").permissions();
    perms.set_mode(0o755); // make it executable
    fs::set_permissions(&hook_path, perms).expect("Failed to set executable permissions");

    println!(" Git pre-commit hook installed successfully.");
}

