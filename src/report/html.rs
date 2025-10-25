// src/report/html.rs
use crate::report::{FileValidationResult, ValidationSummary};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Exports the validation summary to a readable HTML report.
pub fn export_to_html(summary: &ValidationSummary, path: &str) -> Result<(), String> {
    let output_path = Path::new(path);
    if let Some(parent) = output_path.parent() {
        create_dir_all(parent).map_err(|e| format!("Failed to create output directory: {e}"))?;
    }

    let mut file = File::create(output_path).map_err(|e| format!("Failed to create file: {e}"))?;

    writeln!(
        file,
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Rust Checker Report</title>
<style>
  body {{ font-family: Arial, Helvetica, sans-serif; background: #fdfdfd; color: #333; margin: 24px; }}
  h1 {{ color: #005f8a; }}
  table {{ border-collapse: collapse; width: 100%; }}
  th, td {{ border: 1px solid #ccc; padding: 8px; font-size: 14px; }}
  th {{ background-color: #f0f6fa; text-align: left; }}
  .pass {{ background-color: #e0ffe0; }}
  .fail {{ background-color: #ffe0e0; }}
</style>
</head>
<body>"#
    )
    .map_err(|e| format!("Failed to write HTML header: {e}"))?;

    writeln!(file, "<h1>Rust Checker Validation Summary</h1>")
        .map_err(|e| format!("Failed to write header: {e}"))?;
    writeln!(
        file,
        "<p><strong>Files Checked:</strong> {} | <strong>Passed:</strong> {} | <strong>Failed:</strong> {}</p>",
        summary.total_files, summary.passed, summary.failed
    )
    .map_err(|e| format!("Failed to write stats: {e}"))?;

    writeln!(file, "<table><tr><th>File</th><th>Status</th><th>Error</th></tr>")
        .map_err(|e| format!("Failed to write table header: {e}"))?;

    for FileValidationResult { file: name, passed, error } in &summary.results {
        let class = if *passed { "pass" } else { "fail" };
        let status = if *passed { "Passed" } else { "Failed" };
        let error_text = html_escape(error.as_deref().unwrap_or(""));
        let file_name = html_escape(name);

        writeln!(
            file,
            "<tr class=\"{class}\"><td>{file_name}</td><td>{status}</td><td>{error_text}</td></tr>"
        )
        .map_err(|e| format!("Failed to write table row: {e}"))?;
    }

    writeln!(file, "</table></body></html>")
        .map_err(|e| format!("Failed to write footer: {e}"))?;

    Ok(())
}

/// Simple HTML escaping utility.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
