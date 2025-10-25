// src/report.rs
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct FileValidationResult {
    pub file: String,
    pub passed: bool,
    pub error: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct ValidationSummary {
    pub total_files: usize,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<FileValidationResult>,
}

/// Prints the validation summary as formatted JSON.
pub fn print_json_report(summary: &ValidationSummary) {
    match serde_json::to_string_pretty(summary) {
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("Failed to serialize report: {e}"),
    }
}

// Expose submodules for HTML, JUnit XML, and Badge export
// These will fix unresolved imports in main.rs.
pub mod badge;
pub mod html;
pub mod junit;
