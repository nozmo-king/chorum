# Chorum

A Rust-based web application built with Axum framework.

## Features

- Web server using Axum
- SQLite database with SQLx
- Template rendering with Askama
- Cryptographic operations with secp256k1
- Proof of Work system
- CORS support
- File serving capabilities

## Dependencies

- **Axum**: Modern web framework
- **Tokio**: Async runtime
- **SQLx**: Async SQL toolkit
- **Askama**: Template engine
- **secp256k1**: Elliptic curve cryptography
- **SQLite**: Database storage

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Running the Application

```bash
cargo run
```

### Building for Production

```bash
cargo build --release
```

## Project Structure

- `src/` - Source code
  - `main.rs` - Application entry point
  - `config.rs` - Configuration management
  - `db.rs` - Database operations
  - `error.rs` - Error handling
  - `models.rs` - Data models
  - `pow.rs` - Proof of Work implementation
  - `handlers/` - Request handlers
  - `templates/` - Template files
- `migrations/` - Database migrations
- `target/` - Build artifacts

## License

This project is open source.