use chrono::Utc;
use std::fs;

// Expose internal modules so binaries and external users can import them.
pub mod config;
pub mod report;          // html, badge, junit exporters (via src/report/mod.rs)
pub mod rules;
pub mod scanner;         // file system scanner
pub mod tooling;
pub mod fixer;
pub mod plugin;
pub mod unused_checker;
pub mod web;             // actix-web dashboard (src/web/mod.rs)

use rules::RuleConfig;
use unused_checker::check_unused_imports;

/// Validate a file based on rules provided in RuleConfig
pub fn validate_rust_file(file_path: &str, config: &RuleConfig) -> Result<(), String> {
    println!("[{}] Validating file: {}", Utc::now(), file_path);

    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

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
