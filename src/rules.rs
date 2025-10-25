use crate::config::RuleConfig as FileRuleConfig;

#[derive(Debug, Clone)]
pub struct RuleConfig {
    pub check_main: bool,
    pub check_unused_var: bool,
    pub check_unused_import: bool,
}

impl Default for RuleConfig {
    fn default() -> Self {
        Self {
            check_main: true,
            check_unused_var: true,
            check_unused_import: true,
        }
    }
}

impl RuleConfig {
    pub fn from_args(args: &[String]) -> Self {
        Self {
            check_main: !args.contains(&"--skip-main".to_string()),
            check_unused_var: !args.contains(&"--allow-unused-var".to_string()),
            check_unused_import: !args.contains(&"--allow-unused-import".to_string()),
        }
    }

    pub fn from_args_and_config(args: &[String], file: Option<FileRuleConfig>) -> Self {
        let cli = Self::from_args(args);
        if let Some(cfg) = file {
            Self {
                check_main: if args.contains(&"--skip-main".to_string()) {
                    false
                } else {
                    cfg.check_main.unwrap_or(cli.check_main)
                },
                check_unused_var: if args.contains(&"--allow-unused-var".to_string()) {
                    false
                } else {
                    cfg.check_unused_var.unwrap_or(cli.check_unused_var)
                },
                check_unused_import: if args.contains(&"--allow-unused-import".to_string()) {
                    false
                } else {
                    cfg.check_unused_import.unwrap_or(cli.check_unused_import)
                },
            }
        } else {
            cli
        }
    }
}
