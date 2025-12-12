# Triboferrin

A Discord Bot for Text-To-Speech built in Rust. Users can type text messages, and the bot will relay them as voice in a Discord voice channel.

## Features

### Current
- Discord bot with Serenity framework
- Voice channel support via Songbird
- Discord API proxy support (for custom rate limiting or network configurations)
- Hierarchical configuration system (CLI args, environment variables, TOML files)
- Structured logging with tracing

### Planned
- **Text-To-Speech**: Type text and have it spoken in voice channels
  - Multiple voice model support (starting with Google Text-To-Speech)
- **Speech-To-Text**: Convert voice to text
- **LLM Summarization**: Summarize conversations using language models

## Deployment

The bot can be run in two modes:

1. **Command Line**: Direct execution for development and testing
2. **Containerized**: Production deployment to container platforms (e.g., Google Cloud Run)
   - Configure via environment variables (`TRIBOFERRIN_*` prefix)
   - Or mount a configuration file (`triboferrin-config.toml`)

## Prerequisites

- Rust 1.74+
- System dependencies (macOS):
  ```bash
  brew install cmake opus pkg-config
  ```

## Discord Bot Setup

1. Go to [Discord Developer Portal](https://discord.com/developers/applications) and click **New Application**
2. Navigate to **Bot** → **Reset Token** → copy and save the token

   > **Security:** Never share your bot token publicly; it grants full access to your bot. Avoid exposing it in screenshots, issue reports, public channels, or shell history. If accidentally exposed, immediately regenerate it via **Reset Token**.

3. Under **Privileged Gateway Intents**, enable:
   - **Message Content Intent**
4. Go to **OAuth2** → **URL Generator**:
   - Scopes: `bot`
   - Bot Permissions: `Connect`, `Speak`, `Send Messages`, `Read Message History`
5. Open the generated URL to invite the bot to your server

## Quick Start

1. Build and run:
   ```bash
   TRIBOFERRIN_DISCORD_TOKEN=your-bot-token cargo run --release
   ```

### Configuration

Configure via environment variables, TOML file, or CLI args:

```bash
# Environment variables
TRIBOFERRIN_DISCORD_TOKEN=your-bot-token cargo run
TRIBOFERRIN_DISCORD_API_URL=http://proxy:3000 cargo run  # optional proxy

# CLI arguments
cargo run -- --discord-token your-bot-token
cargo run -- --discord-api-url http://proxy:3000  # optional proxy

# TOML configuration (triboferrin-config.toml)
cargo run -- --config /path/to/config.toml
```

Example `triboferrin-config.toml`:
```toml
discord_token = "your-bot-token"
discord_api_url = "http://proxy:3000"  # optional
host = "localhost"
port = 8080
log_level = "info"
```

## Logging

The application uses `tracing` for structured logging. The default log level is `info`.

Override the log level using the `RUST_LOG` environment variable:

```bash
# Set global log level
RUST_LOG=debug cargo run

# Set log level for specific crates
RUST_LOG=triboferrin=trace cargo run

# Combine multiple directives
RUST_LOG=triboferrin=debug,figment=warn cargo run
```
