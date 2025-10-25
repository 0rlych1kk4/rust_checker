use chrono::Utc;
use std::fs;

// Expose internal modules to the library users and binaries
pub mod config;
pub mod fixer;
pub mod parallel;
pub mod plugin;
pub mod report; // reports (html, junit, badge) live under report/
pub mod rules;
pub mod scanner;
pub mod summary;
pub mod tooling;
pub mod unused_checker;
pub mod web;

use rules::RuleConfig;
use unused_checker::check_unused_imports;

/// Validate a file based on rules provided in RuleConfig
pub fn validate_rust_file(file_path: &str, config: &RuleConfig) -> Result<(), String> {
    println!("[{}] Validating file: {}", Utc::now(), file_path);

    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    // --- Rule checks ---
    if config.check_main && !content.contains("fn main") {
        return Err("Missing `fn main` entry point.".into());
    }

    if config.check_unused_var && content.contains("let _") {
        return Err("Contains unused variable pattern `let _`.".into());
    }

    if config.check_unused_import {
        if let Some(warning) = check_unused_imports(&content) {
            println!("{}", warning);
        }
    }

    Ok(())
}
