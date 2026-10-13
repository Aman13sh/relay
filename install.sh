#!/usr/bin/env bash
#
# install.sh
#
# One-shot installer for relay-server and relay-client.
# Installs pre-built binaries without requiring Rust or Cargo.
#
# Usage:
#   ./install.sh                # installs prebuilt binaries
#   ./install.sh --from-source  # builds with cargo if Rust is installed

set -euo pipefail

# If PREFIX is not set, prefer ~/.local/bin if /usr/local/bin isn't writable.
if [ -z "${PREFIX:-}" ]; then
    if [ -w "/usr/local/bin" ] || [ "$EUID" -eq 0 ]; then
        PREFIX="/usr/local"
    else
        PREFIX="$HOME/.local"
    fi
fi

BINDIR="$PREFIX/bin"
BUILD_DIR="target/release"
SERVER_BIN="relay-server"
CLIENT_BIN="relay-client"

print_step() { echo "▶ $1"; }
print_ok()   { echo "  ✓ $1"; }
print_err()  { echo "  ✗ $1" >&2; }

has_cmd() { command -v "$1" &>/dev/null; }

# Copies prebuilt binaries from ./bin/ into target BINDIR.
install_prebuilt() {
    print_step "Installing pre-built binaries to $BINDIR"

    if [ ! -f "bin/$SERVER_BIN" ] || [ ! -f "bin/$CLIENT_BIN" ]; then
        print_err "Pre-built binaries not found in ./bin/"
        print_err "Run with --from-source to build, or use a release package."
        exit 1
    fi

    mkdir -p "$BINDIR"
    install -m 755 "bin/$SERVER_BIN" "$BINDIR/$SERVER_BIN"
    install -m 755 "bin/$CLIENT_BIN" "$BINDIR/$CLIENT_BIN"
}

# Builds from source using cargo release profile.
install_from_source() {
    print_step "Building from source (release mode)"

    if ! has_cmd cargo; then
        print_err "Rust/Cargo not found. Install from https://rustup.rs"
        exit 1
    fi

    cargo build --release
    mkdir -p "$BINDIR"
    install -m 755 "$BUILD_DIR/$SERVER_BIN" "$BINDIR/$SERVER_BIN"
    install -m 755 "$BUILD_DIR/$CLIENT_BIN" "$BINDIR/$CLIENT_BIN"
}

# Checks if binaries can be found directly from user's PATH.
verify_install() {
    print_step "Checking PATH"

    if ! has_cmd "$SERVER_BIN"; then
        echo ""
        echo "  Notice: $BINDIR is not currently in your PATH."
        echo "  Add it to your ~/.bashrc or ~/.zshrc:"
        echo ""
        echo "    export PATH=\"$BINDIR:\$PATH\""
        echo ""
    else
        print_ok "$SERVER_BIN found at $(command -v $SERVER_BIN)"
        print_ok "$CLIENT_BIN found at $(command -v $CLIENT_BIN)"
    fi
}

main() {
    echo "relay installer"
    echo "──────────────────────────────────"

    if [ "${1:-}" = "--from-source" ]; then
        install_from_source
    else
        install_prebuilt
    fi

    print_ok "Installed $BINDIR/$SERVER_BIN"
    print_ok "Installed $BINDIR/$CLIENT_BIN"
    verify_install

    echo ""
    echo "Done! Run:"
    echo "  relay-server"
    echo "  relay-client"
}

main "$@"
