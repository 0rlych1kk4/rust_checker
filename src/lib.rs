// src/lib.rs
use chrono::Utc;
use std::fs;

// Public modules used across the crate
pub mod config;
pub mod fixer;
pub mod plugin;
pub mod report;
pub mod rules;
pub mod scanner;
pub mod tooling;
pub mod unused_checker;
pub mod web;

use rules::RuleConfig;
use unused_checker::check_unused_imports;

/// Validate a file based on rules provided in `RuleConfig`.
/// Returns `Ok(())` when all checks pass, or `Err(String)` describing the first failure.
pub fn validate_rust_file(file_path: &str, config: &RuleConfig) -> Result<(), String> {
    println!("[{}] Validating file: {}", Utc::now(), file_path);

    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // 1) Check for `fn main`
    if config.check_main && !content.contains("fn main") {
        return Err("Missing `fn main` entry point.".into());
    }

    // 2) Heuristic unused var check
    if config.check_unused_var && content.contains("let _") {
        return Err("Found `let _` pattern (allow with --allow-unused-var).".into());
    }

    // 3) Heuristic unused import check (delegate to unused_checker)
    if config.check_unused_import {
        if let Some(msg) = check_unused_imports(&content) {
            return Err(msg);
        }
    }

    Ok(())
}
