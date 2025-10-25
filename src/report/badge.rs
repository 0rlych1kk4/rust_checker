// src/report/badge.rs
use super::ValidationSummary;
use std::fs::File;
use std::io::Write as IoWrite;

pub fn export_svg_badge(summary: &ValidationSummary, output_path: &str) -> Result<(), String> {
    let passed = summary.passed;
    let failed = summary.failed;
    let total = passed + failed;
    let percent = if total > 0 {
        (passed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let color = if percent >= 90.0 {
        "#4c1"
    } else if percent >= 70.0 {
        "#dfb317"
    } else {
        "#e05d44"
    };

    let label = "validation";
    let text = format!("{}% passed", percent.round() as i64);
    let width = 150;

    // NOTE: r###"... "### prevents accidental termination by sequences like `"#fff"`
    let svg = format!(
        r###"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="20" role="img" aria-label="{label}: {text}">
  <linearGradient id="a" x2="0" y2="100%">
    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
    <stop offset="1" stop-opacity=".1"/>
  </linearGradient>
  <rect width="{width}" height="20" rx="3" fill="#555"/>
  <rect x="70" width="{right_w}" height="20" rx="3" fill="{color}"/>
  <rect width="{width}" height="20" rx="3" fill="url(#a)"/>
  <g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
    <text x="35" y="14">{label}</text>
    <text x="{text_x}" y="14">{text}</text>
  </g>
</svg>"###,
        right_w = width - 70,
        text_x = 70 + (width - 70) / 2,
        width = width,
        label = label,
        text = text,
        color = color
    );

    let mut file = File::create(output_path).map_err(|e| format!("create badge: {e}"))?;
    file.write_all(svg.as_bytes())
        .map_err(|e| format!("write badge: {e}"))?;
    Ok(())
}
