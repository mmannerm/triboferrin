# CLAUDE.md

Discord Bot for Text-To-Speech in Rust. Type text → bot speaks in voice channel.

## Commands

```bash
cargo build              # build
cargo build --release    # release build
cargo run                # run
cargo test               # test
cargo clippy             # lint
cargo fmt                # format
```

## Container

Multi-arch container using distroless base image. Requires 8GB+ memory for build.

```bash
# macOS (Apple Container)
container system start
container build --memory 8G --tag triboferrin .
container run triboferrin

# Docker
docker build -t triboferrin .
docker run triboferrin
```

Build for specific platform:
```bash
container build --platform linux/amd64 --tag triboferrin:amd64 .
container build --platform linux/arm64 --tag triboferrin:arm64 .
```

## Configuration

Figment-based, precedence (low→high):
1. Defaults in `Config::default()`
2. `triboferrin-config.toml` (override path with `-c`)
3. `TRIBOFERRIN_*` env vars
4. CLI args

Parameters: `host` (localhost), `port` (8080), `log_level` (info), `verbose`

## Logging

Uses `tracing`. Default INFO, override with `RUST_LOG` env var.

## Git Workflow

Conventional Commits: `<type>: <description>`

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`

Enable pre-commit hooks (runs fmt, clippy, test):
```bash
git config core.hooksPath .githooks
```

PR flow:
```bash
git checkout -b feature/<name>
# commit changes
git push -u origin feature/<name>
gh pr create --title "<type>: <desc>" --body "## Summary\n- ...\n\n## Test plan\n- [ ] ..."
```

## Planned Features

- TTS via Google Text-To-Speech (pluggable architecture)
- Speech-To-Text
- LLM Summarization
