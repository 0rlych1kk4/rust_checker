use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Export a small SVG badge representing validation results
pub fn export_badge(passed: usize, failed: usize, output_path: &str) -> Result<(), String> {
    let total = passed + failed;
    let percent = if total > 0 {
        (passed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let color = if percent >= 90.0 {
        "#4c1" // green
    } else if percent >= 70.0 {
        "#dfb317" // yellow
    } else {
        "#e05d44" // red
    };

    let path = Path::new(output_path);
    let mut file = File::create(path)
        .map_err(|e| format!("Failed to create badge file: {}", e))?;

    // NOTE: The entire SVG is wrapped as a single *raw string literal*.
    // Using r#""# avoids needing to escape quotes or hashtags.
    let svg = format!(
r#"<svg xmlns="http://www.w3.org/2000/svg" width="150" height="20">
  <linearGradient id="a" x2="0" y2="100%">
    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
    <stop offset="1" stop-opacity=".1"/>
  </linearGradient>
  <rect width="150" height="20" rx="3" fill="#fff" />
  <rect width="150" height="20" fill="{color}" />
  <path fill="{color}" d="M0 0h150v20H0z"/>
  <g fill="#fff" text-anchor="middle"
     font-family="DejaVu Sans, Verdana, Geneva, sans-serif"
     font-size="11">
    <text x="75" y="14">Validation {percent:.0}% Passed</text>
  </g>
</svg>"#
    );

    file.write_all(svg.as_bytes())
        .map_err(|e| format!("Failed to write SVG badge: {}", e))?;
    Ok(())
}
