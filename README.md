# rust-timescale-sqlx-rocket
A little integration test of Rust, TimescaleDB, sqlx, and Rocket

## Requirements

- Install PostgreSQL with TimescaleDB
- cargo install sqlx-cli

## Development

    cargo watch -x 'sqlx database reset && cargo run'

## Benchmark

    cargo run --release
    oha localhost:8000
