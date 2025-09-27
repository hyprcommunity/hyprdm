#!/usr/bin/env bash
# or any POSIX shell: works for bash/zsh/fish

set -e

# Rust projesinin yolu: script'in bulunduğu dizinin bir üstü
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
RUST_PROJECT_DIR="$SCRIPT_DIR/../"

LIB_FILE="libhyprdmbackend.a"

# Rust backend build
if [ ! -f "$RUST_PROJECT_DIR/target/release/$LIB_FILE" ]; then
    echo "Building Rust backend from $RUST_PROJECT_DIR..."
    (cd "$RUST_PROJECT_DIR" && cargo build --release)
fi

# LIB_PATH tespiti
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

# Export değişkeni hem bash/zsh hem de fish için
if [ -n "$BASH_VERSION" ] || [ -n "$ZSH_VERSION" ]; then
    export HYPRDM_LIB_DIR="$LIB_PATH"
    echo "HYPRDM_LIB_DIR set to $HYPRDM_LIB_DIR"
elif [ -n "$FISH_VERSION" ]; then
    set -x HYPRDM_LIB_DIR "$LIB_PATH"
    echo "HYPRDM_LIB_DIR set to $HYPRDM_LIB_DIR (fish syntax)"
else
    export HYPRDM_LIB_DIR="$LIB_PATH"
    echo "HYPRDM_LIB_DIR set to $HYPRDM_LIB_DIR"
fi

echo "Rust backend ready. HYPRDM_LIB_DIR points to the library directory."
