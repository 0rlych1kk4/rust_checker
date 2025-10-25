use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub rules: Option<RuleConfig>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct RuleConfig {
    pub check_main: Option<bool>,
    pub check_unused_var: Option<bool>,
    pub check_unused_import: Option<bool>,
}

impl Config {
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    }
}
