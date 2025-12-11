mod config;

use clap::Parser;
use serenity::all::GatewayIntents;
use serenity::client::ClientBuilder;
use serenity::http::HttpBuilder;
use serenity::prelude::*;
use songbird::SerenityInit;

use crate::config::{Args, build_config};

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: serenity::model::gateway::Ready) {
        tracing::info!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = build_config(&args)?;

    tracing_subscriber::fmt()
        .compact()
        .with_thread_names(true)
        .with_env_filter(tracing_subscriber::EnvFilter::new(&config.log_level))
        .init();

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
