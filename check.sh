#!/usr/bin/bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
