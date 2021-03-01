#!/usr/bin/env bash

RUST_LOG=server=debug,warp=info cargo watch -i tests/ -d 2 -cx 'run --bin server'
