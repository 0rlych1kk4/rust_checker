// src/scanner.rs
use std::fs;
use std::path::{Path, PathBuf};

/// Recursively scans `root` for `.rs` files, skipping common build and VCS directories.
///
/// Skipped directories: `target`, `.git`, `.cargo`, `node_modules`.
pub fn scan_rust_files(root: &str) -> Vec<String> {
    let mut out = Vec::new();
    visit(Path::new(root), &mut out);
    out
}

fn visit(dir: &Path, out: &mut Vec<String>) {
    if !dir.is_dir() {
        return;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path: PathBuf = entry.path();

            // Skip symlinks to avoid cycles.
            if let Ok(ft) = entry.file_type() {
                if ft.is_symlink() {
                    continue;
                }
            }

            if path.is_dir() {
                // Collapse inner match per clippy::collapsible_match
                if let Some("target" | ".git" | ".cargo" | "node_modules") =
                    path.file_name().and_then(|s| s.to_str())
                {
                    continue;
                }
                visit(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                if let Some(s) = path.to_str() {
                    out.push(s.to_string());
                }
            }
        }
    }
}
