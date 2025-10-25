// src/plugin.rs
use std::fs::{self, write};
use std::path::PathBuf;
use std::process::Command;

/// Discovers and returns the content of all plugin files (*.rs) under /plugins.
pub fn load_plugin_sources(plugin_dir: &str) -> Vec<(String, String)> {
    let mut plugins = Vec::new();

    if let Ok(entries) = fs::read_dir(plugin_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        plugins.push((name.to_string(), content));
                    }
                }
            }
        }
    }

    plugins
}

/// Compiles and runs plugin scripts under /plugins.
/// Each plugin should print `ok` or `err: <msg>` to stdout.
pub fn compile_and_run_plugins(plugin_dir: &str) -> Vec<(String, Result<(), String>)> {
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(plugin_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let plugin_name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let binary_path = PathBuf::from(format!("target/{}_plugin", plugin_name));

                let compile_status = Command::new("rustc")
                    .arg(&path)
                    .arg("-o")
                    .arg(&binary_path)
                    .status();

                if let Ok(status) = compile_status {
                    if status.success() {
                        match Command::new(&binary_path).output() {
                            Ok(out) => {
                                let stdout = String::from_utf8_lossy(&out.stdout);
                                if stdout.trim() == "ok" {
                                    results.push((plugin_name, Ok(())));
                                } else {
                                    results.push((plugin_name, Err(stdout.trim().to_string())));
                                }
                            }
                            Err(e) => results.push((plugin_name, Err(format!("Failed to run plugin: {}", e)))),
                        }
                    } else {
                        results.push((plugin_name, Err("Compilation failed".into())));
                    }
                } else {
                    results.push((plugin_name, Err("Failed to invoke rustc".into())));
                }
            }
        }
    }

    results
}

/// Generates a template plugin `.rs` file under `/plugins/`.
pub fn generate_plugin_template(name: &str) -> Result<(), String> {
    let filename = format!("plugins/{}.rs", name);
    let code = r#"fn main() {
    // TODO: Implement your plugin logic here
    // Output "ok" if pass, or "err: <reason>" if fail
    println!("ok");
}
"#;

    write(&filename, code).map_err(|e| format!("Failed to write plugin file: {}", e))?;
    Ok(())
}
