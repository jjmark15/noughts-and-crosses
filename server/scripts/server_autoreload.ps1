$env:RUST_LOG = 'info'
cargo watch -i tests/ -cx 'run --bin server'
