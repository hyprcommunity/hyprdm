#!/bin/bash
set -e

RUST_PROJECT_DIR="../../"

# Rust library'nin build edilmesi
LIB_FILE="libhyprdmbackend.a"
if [ ! -f "$RUST_PROJECT_DIR/target/release/$LIB_FILE" ]; then
    echo "Building Rust backend from $RUST_PROJECT_DIR..."
    (cd "$RUST_PROJECT_DIR" && cargo build --release)
fi

# Rust static library yolunu belirleme
if [ -f "$RUST_PROJECT_DIR/target/release/$LIB_FILE" ]; then
    LIB_PATH="$(cd "$RUST_PROJECT_DIR/target/release" && pwd)"
elif [ -f "/usr/lib/$LIB_FILE" ]; then
    LIB_PATH="/usr/lib"
elif [ -f "/usr/local/lib/$LIB_FILE" ]; then
    LIB_PATH="/usr/local/lib"
else
    echo "Error: $LIB_FILE not found!"
    exit 1
fi

# Ortam değişkeni olarak ayarla
export HYPRDM_LIB_DIR="$LIB_PATH"
echo "HYPRDM_LIB_DIR set to $HYPRDM_LIB_DIR"

# Başarılı mesaj
echo "Rust backend ready. HYPRDM_LIB_DIR points to the library directory."
