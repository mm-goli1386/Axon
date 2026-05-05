# Axon - Matrix Homeserver in Rust

Axon is a high-performance, fully-featured Matrix homeserver written in Rust,
compatible with Synapse's API and functionality.

## Status

🚧 **Alpha** - Under active development

## Features

- Matrix API v1.11 compatibility
- Full federation support
- Sliding Sync (MSC3575)
- End-to-end encryption support
- Media repository
- Push notifications
- Application services
- Room version 11 support
- PostgreSQL storage (SQLite for testing)

## Quick Start

### Using Docker

```bash
docker run -p 8008:8008 ghcr.io/mm-goli1386/axon:latest
```

### From source

```bash
git clone https://github.com/mm-goli1386/Axon.git
cd Axon
cargo build --release
./target/release/axon
```

## Configuration

Create a `config.yaml` file:

```yaml
server_name: "localhost"
listen_port: 8008
database:
  backend: "postgres"
  url: "postgres://user:pass@localhost/axon"
```

## Documentation

- [Architecture](docs/architecture.md)
- [API Compatibility](docs/api_comparison.md)
- [Contributing](CONTRIBUTING.md)

## License

AGPL-3.0-only