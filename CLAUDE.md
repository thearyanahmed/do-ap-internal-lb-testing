# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust web service designed to test DigitalOcean App Platform's load balancing behavior. The application creates a simple HTTP server that tracks requests per instance and includes unique instance identifiers for load balancing verification.

## Architecture

- **Framework**: Actix-web HTTP server
- **Language**: Rust (edition 2024)
- **Container**: Docker with Rust nightly runtime
- **Port**: 8080 (exposed and bound to 0.0.0.0:8080)

### Core Components

- `AppState`: Shared state containing request counter and unique UUID per instance
- `handle_request`: Single endpoint handler that increments counter and logs/returns instance info
- Logging configured with `env_logger` at info level

## Development Commands

### Building
```bash
cargo build --release
```

### Running locally
```bash
cargo run
```

### Docker
```bash
docker build -t load-balancer-test .
docker run -p 8080:8080 load-balancer-test
```

### Testing Load Balancing
Use `hey` tool to generate load for testing:
```bash
hey -n 200 -c 4 -z 3m $host
```

## Key Files

- `src/main.rs`: Single-file application with all logic
- `Cargo.toml`: Dependencies (actix-web, log, env_logger, uuid)
- `Dockerfile`: Multi-stage build using rust nightly-slim
- `README.md`: Usage instructions and load testing methodology

## Instance Identification

Each server instance generates a unique UUID v4 on startup. This UUID is included in all responses and log messages to identify which instance handled each request, enabling verification of load balancing distribution.