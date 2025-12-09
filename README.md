# Triboferrin

A Discord Bot for Text-To-Speech built in Rust. Users can type text messages, and the bot will relay them as voice in a Discord voice channel.

## Features

### Current
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

## Quick Start

```bash
# Build
cargo build --release

# Run locally
cargo run

# Run with custom configuration
cargo run -- --config /path/to/config.toml

# Run with environment variables
TRIBOFERRIN_HOST=0.0.0.0 TRIBOFERRIN_PORT=8080 cargo run
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
