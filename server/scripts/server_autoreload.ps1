$env:RUST_LOG = 'server=debug,warp=info'
cargo watch -i tests/ -cx 'run --bin server'
