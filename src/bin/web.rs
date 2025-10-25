use rust_checker::report::{FileValidationResult, ValidationSummary};
use rust_checker::web::run_dashboard;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Example placeholder summary to serve
    let summary = ValidationSummary {
        total_files: 3,
        passed: 2,
        failed: 1,
        results: vec![
            FileValidationResult {
                file: "src/main.rs".into(),
                passed: true,
                error: None,
            },
            FileValidationResult {
                file: "src/lib.rs".into(),
                passed: false,
                error: Some("Missing `fn main`".into()),
            },
            FileValidationResult {
                file: "src/web/mod.rs".into(),
                passed: true,
                error: None,
            },
        ],
    };

    println!(" Starting Rust Checker Web Dashboard at http://localhost:8080");
    run_dashboard(summary).await
}
