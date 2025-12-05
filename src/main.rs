use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE_TOML: &str = "triboferrin-config.toml";

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to configuration file (overrides all default locations)
    #[arg(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<PathBuf>,

    /// Server host    
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<String>,

    /// Server port
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<u16>,

    /// Log level (debug, info, warn, error)
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    log_level: String,
}

// The default configuration.
impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            log_level: "info".to_string(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut figment = Figment::from(Serialized::defaults(Config::default()));

    if let Some(config_path) = args.config.as_ref() {
        figment = figment.merge(Toml::file(config_path));
    } else {
        figment = figment.merge(Toml::file(CONFIG_FILE_TOML));
    }

    figment = figment
        .merge(Env::prefixed("TRIBOFERRIN_"))
        .merge(Serialized::defaults(Args {
            config: None,
            host: args.host,
            port: args.port,
            log_level: args.log_level,
            verbose: args.verbose,
        }));

    let config: Config = figment.extract().expect("Failed to load configuration");

    dbg!(config);

    println!("Hello, world!");
}
