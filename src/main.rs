use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};
use serenity::all::GatewayIntents;
use serenity::client::ClientBuilder;
use serenity::http::HttpBuilder;
use serenity::prelude::*;
use songbird::SerenityInit;
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

    /// Discord bot token
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    discord_token: Option<String>,

    /// Discord API base URL (for proxy support)
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    discord_api_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    log_level: String,
    discord_token: String,
    discord_api_url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            log_level: "info".to_string(),
            discord_token: String::new(),
            discord_api_url: None,
        }
    }
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: serenity::model::gateway::Ready) {
        tracing::info!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .compact()
        .with_thread_names(true)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
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
            discord_token: args.discord_token,
            discord_api_url: args.discord_api_url,
        }));

    let config: Config = figment.extract()?;

    tracing::info!("config = {:?}", config);

    if config.discord_token.is_empty() {
        return Err(
            "Discord token is required. Set TRIBOFERRIN_DISCORD_TOKEN or use --discord-token"
                .into(),
        );
    }

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    let http = if let Some(ref api_url) = config.discord_api_url {
        tracing::info!("Using custom Discord API URL: {}", api_url);
        HttpBuilder::new(&config.discord_token)
            .proxy(api_url)
            .ratelimiter_disabled(true)
            .build()
    } else {
        HttpBuilder::new(&config.discord_token).build()
    };

    let mut client = ClientBuilder::new_with_http(http, intents)
        .event_handler(Handler)
        .register_songbird()
        .await?;

    tracing::info!("Starting Discord bot...");
    client.start().await?;

    Ok(())
}
