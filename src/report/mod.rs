// src/report/mod.rs
use serde::Serialize;

pub mod badge;
pub mod html;
pub mod junit;

pub use badge::export_svg_badge;
pub use html::export_to_html;

#[derive(Debug, Clone, Serialize)]
pub struct FileValidationResult {
    pub file: String,
    pub passed: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationSummary {
    pub total_files: usize,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<FileValidationResult>,
}

pub fn print_json_report(summary: &ValidationSummary) {
    println!("{}", serde_json::to_string_pretty(summary).unwrap());
}
