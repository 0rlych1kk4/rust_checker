use std::fs::File;
use std::io::Write as IoWrite;

use super::ValidationSummary;

/// Export the validation summary to a small self-contained HTML file.
///
/// This returns `Result<(), String>` so we map all I/O errors to strings explicitly.
pub fn export_to_html(summary: &ValidationSummary, path: &str) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| format!("create html: {e}"))?;

    writeln!(
        file,
        "<!DOCTYPE html><html><head><meta charset='utf-8'><title>Rust Checker Report</title>
<style>
body {{ font-family: Arial, Helvetica, sans-serif; background: #fdfdfd; color: #333; }}
h1 {{ color: #005f8a; }}
table {{ border-collapse: collapse; width: 100%; }}
th, td {{ border: 1px solid #ddd; padding: 8px; }}
th {{ background-color: #f5f5f5; text-align: left; }}
.pass {{ background-color: #e6ffed; }}
.fail {{ background-color: #ffe0e0; }}
.status {{ font-weight: bold; }}
</style></head><body>"
    )
    .map_err(|e| format!("write head: {e}"))?;

    writeln!(file, "<h1>Rust Checker Validation Summary</h1>")
        .map_err(|e| format!("write title: {e}"))?;

    writeln!(
        file,
        "<p><strong>Files Checked:</strong> {} &nbsp;|&nbsp; \
         <strong>Passed:</strong> {} &nbsp;|&nbsp; \
         <strong>Failed:</strong> {}</p>",
        summary.total_files, summary.passed, summary.failed
    )
    .map_err(|e| format!("write counts: {e}"))?;

    writeln!(
        file,
        "<table><tr><th>File</th><th>Status</th><th>Error</th></tr>"
    )
    .map_err(|e| format!("write table head: {e}"))?;

    for r in &summary.results {
        let status = if r.passed { "PASS" } else { "FAIL" };
        let row_cls = if r.passed { "pass" } else { "fail" };
        let error = r.error.as_deref().unwrap_or("&nbsp;");
        writeln!(
            file,
            "<tr class=\"{row_cls}\"><td>{}</td><td class=\"status\">{status}</td><td>{error}</td></tr>",
            html_escape::encode_text(&r.file) // optional (add `html-escape` crate) or omit if you prefer
        )
        .map_err(|e| format!("write row: {e}"))?;
    }

    writeln!(file, "</table></body></html>").map_err(|e| format!("write tail: {e}"))?;
    Ok(())
}
