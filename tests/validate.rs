// tests/validate.rs
use rust_checker::{rules::RuleConfig, validate_rust_file};
use std::fs::File;
use std::io::Write;

#[test]
fn test_validate_valid_file() {
    let path = "tests/temp_valid.rs";
    let mut file = File::create(path).unwrap();
    writeln!(file, "fn main() {{ println!(\"Hello\"); }}").unwrap();

    let cfg = RuleConfig {
        check_main: true,
        check_unused_var: true,
        check_unused_import: false,
    };
    let result = validate_rust_file(path, &cfg);
    assert!(result.is_ok());

    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_validate_missing_main() {
    let path = "tests/temp_invalid.rs";
    let mut file = File::create(path).unwrap();
    writeln!(file, "fn helper() {{}}").unwrap();

    let cfg = RuleConfig {
        check_main: true,
        check_unused_var: true,
        check_unused_import: false,
    };
    let result = validate_rust_file(path, &cfg);
    assert!(result.is_err());

    std::fs::remove_file(path).unwrap();
}
