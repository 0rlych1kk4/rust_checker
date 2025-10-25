use serde::Serialize;

pub mod badge;
pub mod html;
pub mod junit;

#[derive(Serialize, Debug, Clone)]
pub struct FileValidationResult {
    pub file: String,
    pub passed: bool,
    pub error: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ValidationSummary {
    pub total_files: usize,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<FileValidationResult>,
}

pub fn print_json_report(summary: &ValidationSummary) {
    match serde_json::to_string_pretty(summary) {
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("Failed to serialize report: {e}"),
    }
}

