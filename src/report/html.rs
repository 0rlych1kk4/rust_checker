use super::ValidationSummary;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Export validation summary as an HTML report
pub fn export_to_html(summary: &ValidationSummary, path: &str) -> Result<(), String> {
    let output_path = Path::new(path);
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut file = File::create(output_path).map_err(|e| e.to_string())?;

    writeln!(
        file,
        "<!DOCTYPE html><html><head><meta charset='utf-8'><title>Rust Checker Report</title>
        <style>
        body {{ font-family: Arial; background: #fdfdfd; color: #333; }}
        h1 {{ color: #005f8a; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ccc; padding: 8px; text-align: left; }}
        .pass {{ background-color: #e0ffe0; }}
        .fail {{ background-color: #ffe0e0; }}
        </style></head><body>"
    )
    .map_err(|e| e.to_string())?;

    writeln!(file, "<h1>Rust Checker Validation Summary</h1>").map_err(|e| e.to_string())?;
    writeln!(
        file,
        "<p><strong>Files Checked:</strong> {} | <strong>Passed:</strong> {} | <strong>Failed:</strong> {}</p>",
        summary.total_files, summary.passed, summary.failed
    )
    .map_err(|e| e.to_string())?;
    writeln!(file, "<table><tr><th>File</th><th>Status</th><th>Error</th></tr>")
        .map_err(|e| e.to_string())?;

    for result in &summary.results {
        let status = if result.passed { "PASS" } else { "FAIL" };
        let row_class = if result.passed { "pass" } else { "fail" };
        writeln!(
            file,
            "<tr class='{row_class}'><td>{}</td><td>{}</td><td>{}</td></tr>",
            result.file,
            status,
            result.error.clone().unwrap_or_default()
        )
        .map_err(|e| e.to_string())?;
    }

    writeln!(file, "</table></body></html>").map_err(|e| e.to_string())?;
    Ok(())
}
