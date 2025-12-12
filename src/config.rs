use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use git_version::git_version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE_TOML: &str = "triboferrin-config.toml";
const VERSION: &str = git_version!(fallback = env!("CARGO_PKG_VERSION"));

#[derive(Parser, Serialize, Deserialize, Default)]
#[command(author, version = VERSION, about, long_about = None)]
pub struct Args {
    /// Path to configuration file (overrides all default locations)
    #[arg(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<PathBuf>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,

    /// Discord bot token
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord_token: Option<String>,

    /// Discord API base URL (for proxy support)
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord_api_url: Option<String>,
}

impl std::fmt::Debug for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Args")
            .field("config", &self.config)
            .field("log_level", &self.log_level)
            .field(
                "discord_token",
                &self.discord_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("discord_api_url", &self.discord_api_url)
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub log_level: String,
    pub discord_token: String,
    pub discord_api_url: Option<String>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("log_level", &self.log_level)
            .field("discord_token", &"[REDACTED]")
            .field("discord_api_url", &self.discord_api_url)
            .finish()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            discord_token: String::new(),
            discord_api_url: None,
        }
    }
}

/// Build configuration from multiple sources with the following precedence (low to high):
/// 1. Default values
/// 2. Configuration file (triboferrin-config.toml or custom path via -c)
/// 3. TRIBOFERRIN_* environment variables
/// 4. RUST_LOG environment variable (for log_level)
/// 5. Command line arguments
#[allow(clippy::result_large_err)]
pub fn build_config(args: &Args) -> Result<Config, figment::Error> {
    build_config_with_path(args, CONFIG_FILE_TOML)
}

/// Build configuration with a custom default config file path.
/// Useful for testing.
#[allow(clippy::result_large_err)]
pub fn build_config_with_path(
    args: &Args,
    default_config_path: &str,
) -> Result<Config, figment::Error> {
    let mut figment = Figment::from(Serialized::defaults(Config::default()));

    if let Some(config_path) = args.config.as_ref() {
        figment = figment.merge(Toml::file(config_path));
    } else {
        figment = figment.merge(Toml::file(default_config_path));
    }

    figment = figment
        .merge(Env::prefixed("TRIBOFERRIN_"))
        .merge(Env::raw().only(&["RUST_LOG"]).map(|_| "log_level".into()))
        .merge(Serialized::defaults(Args {
            config: None,
            log_level: args.log_level.clone(),
            discord_token: args.discord_token.clone(),
            discord_api_url: args.discord_api_url.clone(),
        }));

    figment.extract()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::io::Write;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.log_level, "info");
        assert_eq!(config.discord_token, "");
        assert_eq!(config.discord_api_url, None);
    }

    #[test]
    fn test_args_default() {
        let args = Args::default();
        assert!(args.config.is_none());
        assert!(args.log_level.is_none());
        assert!(args.discord_token.is_none());
        assert!(args.discord_api_url.is_none());
    }

    #[test]
    fn test_build_config_with_defaults() {
        // Clear env vars that could affect test
        temp_env::with_vars(
            [
                ("RUST_LOG", None::<&str>),
                ("TRIBOFERRIN_LOG_LEVEL", None::<&str>),
                ("TRIBOFERRIN_DISCORD_TOKEN", None::<&str>),
                ("TRIBOFERRIN_DISCORD_API_URL", None::<&str>),
            ],
            || {
                let args = Args::default();
                // Use non-existent config file to test defaults
                let config = build_config_with_path(&args, "/nonexistent/config.toml").unwrap();

                assert_eq!(config.log_level, "info");
                assert_eq!(config.discord_token, "");
                assert_eq!(config.discord_api_url, None);
            },
        );
    }

    #[test]
    fn test_build_config_cli_overrides_defaults() {
        let args = Args {
            config: None,
            log_level: Some("debug".to_string()),
            discord_token: Some("test_token".to_string()),
            discord_api_url: Some("https://api.example.com".to_string()),
        };
        let config = build_config_with_path(&args, "/nonexistent/config.toml").unwrap();

        assert_eq!(config.log_level, "debug");
        assert_eq!(config.discord_token, "test_token");
        assert_eq!(
            config.discord_api_url,
            Some("https://api.example.com".to_string())
        );
    }

    #[rstest]
    #[case("trace")]
    #[case("debug")]
    #[case("info")]
    #[case("warn")]
    #[case("error")]
    fn test_build_config_log_levels(#[case] level: &str) {
        let args = Args {
            log_level: Some(level.to_string()),
            ..Default::default()
        };
        let config = build_config_with_path(&args, "/nonexistent/config.toml").unwrap();
        assert_eq!(config.log_level, level);
    }

    #[test]
    fn test_build_config_env_triboferrin_prefix() {
        temp_env::with_vars(
            [
                ("TRIBOFERRIN_DISCORD_TOKEN", Some("env_token")),
                ("TRIBOFERRIN_LOG_LEVEL", Some("warn")),
            ],
            || {
                let args = Args::default();
                let config = build_config_with_path(&args, "/nonexistent/config.toml").unwrap();

                assert_eq!(config.discord_token, "env_token");
                assert_eq!(config.log_level, "warn");
            },
        );
    }

    #[test]
    fn test_build_config_rust_log_env() {
        temp_env::with_vars([("RUST_LOG", Some("trace"))], || {
            let args = Args::default();
            let config = build_config_with_path(&args, "/nonexistent/config.toml").unwrap();

            assert_eq!(config.log_level, "trace");
        });
    }

    #[test]
    fn test_build_config_cli_overrides_env() {
        temp_env::with_vars(
            [
                ("TRIBOFERRIN_DISCORD_TOKEN", Some("env_token")),
                ("RUST_LOG", Some("warn")),
            ],
            || {
                let args = Args {
                    log_level: Some("error".to_string()),
                    discord_token: Some("cli_token".to_string()),
                    ..Default::default()
                };
                let config = build_config_with_path(&args, "/nonexistent/config.toml").unwrap();

                // CLI should override env
                assert_eq!(config.discord_token, "cli_token");
                assert_eq!(config.log_level, "error");
            },
        );
    }

    #[test]
    fn test_build_config_rust_log_overrides_triboferrin_log_level() {
        temp_env::with_vars(
            [
                ("TRIBOFERRIN_LOG_LEVEL", Some("warn")),
                ("RUST_LOG", Some("debug")),
            ],
            || {
                let args = Args::default();
                let config = build_config_with_path(&args, "/nonexistent/config.toml").unwrap();

                // RUST_LOG should override TRIBOFERRIN_LOG_LEVEL
                assert_eq!(config.log_level, "debug");
            },
        );
    }

    #[test]
    fn test_build_config_from_toml_file() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_triboferrin_config.toml");

        let mut file = std::fs::File::create(&config_path).unwrap();
        writeln!(
            file,
            r#"
log_level = "trace"
discord_token = "file_token"
discord_api_url = "https://file.example.com"
"#
        )
        .unwrap();

        // Clear env vars that could affect test
        temp_env::with_vars(
            [
                ("RUST_LOG", None::<&str>),
                ("TRIBOFERRIN_LOG_LEVEL", None::<&str>),
                ("TRIBOFERRIN_DISCORD_TOKEN", None::<&str>),
                ("TRIBOFERRIN_DISCORD_API_URL", None::<&str>),
            ],
            || {
                let args = Args::default();
                let config = build_config_with_path(&args, config_path.to_str().unwrap()).unwrap();

                assert_eq!(config.log_level, "trace");
                assert_eq!(config.discord_token, "file_token");
                assert_eq!(
                    config.discord_api_url,
                    Some("https://file.example.com".to_string())
                );
            },
        );

        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_build_config_custom_config_path() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("custom_config.toml");

        let mut file = std::fs::File::create(&config_path).unwrap();
        writeln!(
            file,
            r#"
discord_token = "custom_token"
"#
        )
        .unwrap();

        // Clear env vars that could affect test
        temp_env::with_vars(
            [
                ("RUST_LOG", None::<&str>),
                ("TRIBOFERRIN_LOG_LEVEL", None::<&str>),
                ("TRIBOFERRIN_DISCORD_TOKEN", None::<&str>),
                ("TRIBOFERRIN_DISCORD_API_URL", None::<&str>),
            ],
            || {
                let args = Args {
                    config: Some(config_path.clone()),
                    ..Default::default()
                };
                // Even with a different default path, the custom path should be used
                let config = build_config_with_path(&args, "/nonexistent/config.toml").unwrap();

                assert_eq!(config.discord_token, "custom_token");
            },
        );

        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_config_precedence_full() {
        // Test full precedence: file < TRIBOFERRIN_ < RUST_LOG < CLI
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("precedence_config.toml");

        let mut file = std::fs::File::create(&config_path).unwrap();
        writeln!(
            file,
            r#"
log_level = "trace"
discord_token = "file_token"
"#
        )
        .unwrap();

        temp_env::with_vars(
            [
                ("TRIBOFERRIN_DISCORD_TOKEN", Some("env_token")),
                ("RUST_LOG", Some("warn")),
            ],
            || {
                let args = Args {
                    discord_token: Some("cli_token".to_string()),
                    ..Default::default()
                };
                let config = build_config_with_path(&args, config_path.to_str().unwrap()).unwrap();

                // CLI overrides env for discord_token
                assert_eq!(config.discord_token, "cli_token");
                // RUST_LOG overrides file for log_level
                assert_eq!(config.log_level, "warn");
            },
        );

        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_config_equality() {
        let config1 = Config {
            log_level: "info".to_string(),
            discord_token: "token".to_string(),
            discord_api_url: None,
        };
        let config2 = Config {
            log_level: "info".to_string(),
            discord_token: "token".to_string(),
            discord_api_url: None,
        };
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_config_clone() {
        let config = Config {
            log_level: "debug".to_string(),
            discord_token: "token".to_string(),
            discord_api_url: Some("https://api.example.com".to_string()),
        };
        let cloned = config.clone();
        assert_eq!(config, cloned);
    }

    #[test]
    fn test_config_debug_redacts_token() {
        let config = Config {
            log_level: "info".to_string(),
            discord_token: "super_secret_token".to_string(),
            discord_api_url: None,
        };
        let debug_output = format!("{:?}", config);
        assert!(
            !debug_output.contains("super_secret_token"),
            "Debug output should not contain the actual token"
        );
        assert!(
            debug_output.contains("[REDACTED]"),
            "Debug output should show [REDACTED] for the token"
        );
    }

    #[test]
    fn test_args_debug_redacts_token() {
        let args = Args {
            discord_token: Some("super_secret_token".to_string()),
            ..Default::default()
        };
        let debug_output = format!("{:?}", args);
        assert!(
            !debug_output.contains("super_secret_token"),
            "Debug output should not contain the actual token"
        );
        assert!(
            debug_output.contains("[REDACTED]"),
            "Debug output should show [REDACTED] for the token"
        );
    }
}
