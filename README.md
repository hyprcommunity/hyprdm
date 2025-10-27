# Introduction
**hyprdm**, or **Hyprland Display Manager**, is a dynamic display manager developed for **Hyprland Tiling Window Management**, providing customization and integration for Hyprland.


<img src="HyprlandDM.png" alt="HyprlandDM" width="400" height="400"/>


## Features
- HOTP and TOTP support for user login, PAM, and 2FA.
- Customization based on Qt QuickShell patches or configurations.
- Thanks to the backend system, we now have **Rust** and **C++** support for developing graphical interfaces.
- **Theme Manager support:** Full support for GTK and Qt themes, including custom theme support.
- With **sessiondm**, you can use Hyprland Display Manager as a session-based login manager.
- Manage full window settings with the layout manager.

In short, we support many integrations and features that carry DM functionality, and we plan to offer more advanced security and integration support in the future.

# Installation

If you want to compile directly with Cargo:

```bash
cd gui-api
cargo build --release
```
If you want to compile the Rust backend FFI library:
```
HDM_API_LIB_TYPE=c cargo build --release --workspace --features c_ffi
```
If you want to compile the Rust backend library directly:
```
HDM_API_LIB_TYPE=rust cargo build --release
```
or if you want to install directly:
```
./build.sh
```
If you want to install the HyprDM-greeterd:

```bash
cd gui-api/greeterd
````

## 🛠️ Build Instructions

### 🔧 Using Makefile

```bash
# Build (Release mode — default)
make -j$(nproc)

# Build (Debug mode)
make MODE=debug -j$(nproc)

# Install
sudo make install

# Clean build artifacts
make clean
````

### ⚙️ Using CMake

```bash
# Configure (Release build)
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j$(nproc)

# Configure (Debug build)
cmake -B build -DCMAKE_BUILD_TYPE=Debug
cmake --build build -j$(nproc)

# Install binary
sudo cmake --install build
```

Hyproman interface:
<img src="/interfaces-examples/Hyproman.png" alt="HyprlandDM" width="1200" height="1200"/>





