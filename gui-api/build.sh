#!/bin/bash
set -e

# Rust proje dizini
RUST_PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Derleme
echo "Building Rust backend..."
(cd "$RUST_PROJECT_DIR" && cargo build --release)

# Kütüphane ve binary dosyaları
LIB_A="$RUST_PROJECT_DIR/target/release/libhyprdmbackend.a"
LIB_SO="$RUST_PROJECT_DIR/target/release/libhyprdmbackend.so"
BIN="$RUST_PROJECT_DIR/target/release/configmanager"

# Hedef dizinler
LIB_DIR="/usr/lib64"
BIN_DIR="/usr/bin"

# Archive kütüphaneyi interaktif kopyala
if [ -f "$LIB_A" ]; then
    echo "Are you sure you want to copy this archive file?"
    echo "The .a file will be compiled by linking it (greeterd, some components)."
    echo "Therefore, if you want to use the target directory, you can do so without copying it."
    read -p "Copy .a file to $LIB_DIR? [y/N]: " answer
    if [[ "$answer" =~ ^[Yy]$ ]]; then
        echo "Copying $LIB_A to $LIB_DIR"
        sudo cp "$LIB_A" "$LIB_DIR/"
    else
        echo "Skipping .a file copy."
    fi
fi

# Shared kütüphaneyi kopyala
if [ -f "$LIB_SO" ]; then
    echo "Copying $LIB_SO to $LIB_DIR"
    sudo cp "$LIB_SO" "$LIB_DIR/"
fi

# Binary dosyayı kopyala
if [ -f "$BIN" ]; then
    echo "Copying $BIN to $BIN_DIR"
    sudo cp "$BIN" "$BIN_DIR/"
    sudo chmod +x "$BIN_DIR/$(basename $BIN)"
fi

echo "Rust backend build and install completed."
