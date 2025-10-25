pub mod html;
pub mod badge;
pub mod junit;

// Re-export commonly used items from this module.
pub use html::export_to_html;
pub use badge::export_svg_badge;

#[derive(Debug, Clone)]
pub struct FileValidationResult {
    pub file: String,
    pub passed: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationSummary {
    pub total_files: usize,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<FileValidationResult>,
}
