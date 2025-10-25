//! Simple check for unused `use` statements.
//! Note: This is a basic heuristic. For full detection, use `cargo clippy`.

/// Checks the Rust source content for patterns that may indicate unused imports.
///
/// This is intentionally lightweight and heuristic-based — it simply looks for
/// the presence of `use` lines and the word "unused" in the content.
/// For robust static analysis, rely on `cargo clippy`.
pub fn check_unused_imports(content: &str) -> Option<String> {
    if content.contains("use ") && content.contains("unused") {
        Some("  Warning: possible unused import detected.".to_string())
    } else {
        None
    }
}

