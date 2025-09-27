#!/bin/bash
set -e

# Rust proje dizini
RUST_PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Kullanıcıya seçim yaptır
echo "Select build mode:"
echo "1) C/FFI mode"
echo "2) Rust backend library"
read -p "Enter 1 or 2: " choice

case "$choice" in
    1)
        export HDM_API_LIB_TYPE="c"
        echo "Compiling C/FFI mode..."
        (cd "$RUST_PROJECT_DIR" && cargo build --release --no-default-features)

        # Dosyaları kopyala
        LIB_SO="$RUST_PROJECT_DIR/target/release/libhdm_api.so"
        BIN="$RUST_PROJECT_DIR/target/release/configmanager"

        if [ -f "$LIB_SO" ]; then
            echo "Copying $LIB_SO to /usr/lib64"
            sudo cp "$LIB_SO" /usr/lib64/
        fi

        if [ -f "$BIN" ]; then
            echo "Copying $BIN to /usr/bin"
            sudo cp "$BIN" /usr/bin/
            sudo chmod +x "/usr/bin/$(basename $BIN)"
        fi
        ;;
    2)
        export HDM_API_LIB_TYPE="rust"
        echo "Compiling Rust backend library..."
        (cd "$RUST_PROJECT_DIR" && cargo build --release)

        # Dosyaları kopyala
        LIB_A="$RUST_PROJECT_DIR/target/release/libhyprdmbackend.a"
        LIB_SO="$RUST_PROJECT_DIR/target/release/libhyprdmbackend.so"
        BIN="$RUST_PROJECT_DIR/target/release/configmanager"

        # Archive kütüphaneyi interaktif kopyala
        if [ -f "$LIB_A" ]; then
            read -p "Copy .a file to /usr/lib64? [y/N]: " answer
            if [[ "$answer" =~ ^[Yy]$ ]]; then
                sudo cp "$LIB_A" /usr/lib64/
            fi
        fi

        # Shared kütüphaneyi kopyala
        if [ -f "$LIB_SO" ]; then
            sudo cp "$LIB_SO" /usr/lib64/
        fi

        # Binary dosyayı kopyala
        if [ -f "$BIN" ]; then
            sudo cp "$BIN" /usr/bin/
            sudo chmod +x "/usr/bin/$(basename $BIN)"
        fi
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo "Build and install completed."
