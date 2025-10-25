use super::ValidationSummary;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Export a JUnit-compatible XML report (suitable for CI parsers)
pub fn export_to_junit_xml(summary: &ValidationSummary, path: &str) -> Result<(), String> {
    let output_path = Path::new(path);
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }

    let mut file = File::create(output_path)
        .map_err(|e| format!("Failed to create JUnit XML file: {e}"))?;

    writeln!(
        file,
        r#"<testsuite name="RustChecker" tests="{}" failures="{}">"#,
        summary.total_files, summary.failed
    )
    .map_err(|e| format!("Failed to write header: {e}"))?;

    for result in &summary.results {
        writeln!(
            file,
            r#"  <testcase classname="rust_checker" name="{}">"#,
            xml_escape(&result.file)
        )
        .map_err(|e| format!("Failed to write testcase header: {e}"))?;

        if let Some(error) = &result.error {
            writeln!(
                file,
                r#"    <failure message="{}"/>"#,
                xml_escape(error)
            )
            .map_err(|e| format!("Failed to write failure node: {e}"))?;
        }

        writeln!(file, r#"  </testcase>"#)
            .map_err(|e| format!("Failed to write testcase footer: {e}"))?;
    }

    writeln!(file, r#"</testsuite>"#)
        .map_err(|e| format!("Failed to write footer: {e}"))?;

    Ok(())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
