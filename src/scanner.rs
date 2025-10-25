// src/scanner.rs
use std::fs;
use std::path::{Path, PathBuf};

pub fn scan_rust_files(root: &str) -> Vec<String> {
    let mut out = Vec::new();
    let root = Path::new(root);
    visit(root, &mut out);
    out
}

fn visit(p: &Path, out: &mut Vec<String>) {
    // Skip target folders
    if p.file_name().and_then(|s| s.to_str()) == Some("target") {
        return;
    }

    let Ok(read) = fs::read_dir(p) else { return };
    for entry in read.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            visit(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Some(s) = path.to_str() {
                out.push(s.to_string());
            }
        }
    }
}
