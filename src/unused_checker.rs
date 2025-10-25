// src/unused_checker.rs

/// Simple check for unused `use` statements.
/// Note: This is a basic heuristic. For full detection, use `cargo clippy`.
pub fn check_unused_imports(content: &str) -> Option<String> {
    if content.contains("use ") && content.contains("unused") {
        Some("️  Warning: possible unused import detected.".to_string())
    } else {
        None
    }
}
