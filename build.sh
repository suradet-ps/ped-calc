#!/bin/bash

# Vercel Build Script for Rust + Trunk (WebAssembly)

# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Vercel uses /rust as the cargo home, whereas standard linux uses $HOME/.cargo
if [ -d "/rust" ]; then
    export CARGO_HOME="/rust"
else
    export CARGO_HOME="$HOME/.cargo"
fi

source "$CARGO_HOME/env"

# 2. Add WASM target
rustup target add wasm32-unknown-unknown

# 3. Install Trunk
# We can download the pre-compiled binary for faster CI builds
TRUNK_VERSION="v0.21.4"
curl -fsSLO https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz
tar -xzf trunk-x86_64-unknown-linux-gnu.tar.gz
mkdir -p "$CARGO_HOME/bin"
mv trunk "$CARGO_HOME/bin/"

# 4. Build the project
trunk build --release
