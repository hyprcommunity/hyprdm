#!/bin/bash
set -e

# Rust library için derleme (üst dizindeki proje)
RUST_PROJECT_DIR="../"

if [ ! -f "$RUST_PROJECT_DIR/target/release/libhyprdmbackend.a" ]; then
    echo "Building Rust backend from $RUST_PROJECT_DIR..."
    (cd "$RUST_PROJECT_DIR" && cargo build --release)
fi

# libhyprdmbackend.a yolunu tespit
if [ -f "$RUST_PROJECT_DIR/target/release/libhyprdmbackend.a" ]; then
    LIB_PATH="$(cd "$RUST_PROJECT_DIR/target/release" && pwd)"
elif [ -f "/usr/lib/libhyprdmbackend.a" ]; then
    LIB_PATH="/usr/lib"
elif [ -f "/usr/local/lib/libhyprdmbackend.a" ]; then
    LIB_PATH="/usr/local/lib"
else
    echo "Error: libhyprdmbackend.a not found!"
    exit 1
fi

# Ortam değişkeni olarak ayarla
export HYPRDM_LIB_DIR="$LIB_PATH"
echo "HYPRDM_LIB_DIR set to $HYPRDM_LIB_DIR"
