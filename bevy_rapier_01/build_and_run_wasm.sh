#!/bin/zsh
# Build and run Bevy WASM project

set -e

echo "Building for wasm32-unknown-unknown..."
cargo build --target wasm32-unknown-unknown --release

echo "Running wasm-bindgen..."
wasm-bindgen --out-dir ./pkg --target web ./target/wasm32-unknown-unknown/release/rust_bevy_play.wasm

echo "Starting Python HTTP server..."
python3 -m http.server 8080 --bind 127.0.0.1
