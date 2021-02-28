#!/usr/bin/env bash

RUST_LOG=server=debug,warp=info cargo watch -i tests/ -cx 'run --bin server'
