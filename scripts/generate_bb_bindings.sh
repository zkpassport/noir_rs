#!/usr/bin/env bash
set -euo pipefail

# Ideally this shouldn't be needed and be handled by the barretenberg-rs itself.

# Generate barretenberg-rs Rust bindings (api.rs + generated_types.rs) for the
# upstream aztec-packages git dependency.
#
# Run this once after changing the barretenberg-rs tag in Cargo.toml, or
# after a fresh clone. It:
#   1. Parses the barretenberg-rs git tag from Cargo.toml
#   2. Updates .cargo/config.toml with the matching BARRETENBERG_VERSION
#   3. Runs `cargo fetch` to populate the cargo git checkout
#   4. Downloads the `bb` CLI binary for the current platform
#   5. Installs TS dependencies and runs the codegen in the checkout
#
# Prerequisites: Node.js (>=16.9), corepack, curl, tar

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# --- Parse barretenberg-rs git tag from Cargo.toml ---
# Matches: tag = "v4.0.0-nightly.20260212" in the barretenberg-rs line
BB_TAG=$(grep 'barretenberg-rs' "$PROJECT_DIR/Cargo.toml" | sed -n 's/.*tag = "\([^"]*\)".*/\1/p')
if [ -z "$BB_TAG" ]; then
    echo "Error: Could not find barretenberg-rs git tag in Cargo.toml"
    echo "Expected a line like: barretenberg-rs = { git = \"...\", tag = \"v...\", ... }"
    exit 1
fi
# Strip the leading 'v' for BARRETENBERG_VERSION (e.g. "v4.0.0-nightly.20260212" -> "4.0.0-nightly.20260212")
BB_VERSION="${BB_TAG#v}"
echo "Barretenberg version: ${BB_TAG} (from Cargo.toml)"

# --- Update .cargo/config.toml with BARRETENBERG_VERSION ---
mkdir -p "$PROJECT_DIR/.cargo"
cat > "$PROJECT_DIR/.cargo/config.toml" << EOF
[env]
RUST_LOG = "info"
BARRETENBERG_VERSION = "${BB_VERSION}"
EOF
echo "Updated .cargo/config.toml with BARRETENBERG_VERSION = \"${BB_VERSION}\""

# --- Ensure cargo has fetched the git dependency ---
echo "Fetching cargo dependencies..."
cd "$PROJECT_DIR"
cargo fetch 2>&1 | tail -5 || true

# --- Find the barretenberg-rs source directory via cargo metadata ---
BB_RS_SRC_DIR=$(cargo metadata --format-version 1 --features barretenberg 2>/dev/null \
    | python3 -c "
import sys, json
meta = json.load(sys.stdin)
for pkg in meta['packages']:
    if pkg['name'] == 'barretenberg-rs':
        # manifest_path is .../barretenberg-rs/Cargo.toml
        import os
        print(os.path.join(os.path.dirname(pkg['manifest_path']), 'src'))
        break
")
if [ -z "$BB_RS_SRC_DIR" ]; then
    echo "Error: Could not find barretenberg-rs source directory via cargo metadata"
    echo "Make sure Cargo.toml has the barretenberg-rs git dependency."
    exit 1
fi

BB_RS_SRC="$BB_RS_SRC_DIR"
# TS dir is at ../../ts relative to barretenberg-rs/src
BB_TS_DIR="$(cd "$BB_RS_SRC/../../../ts" 2>/dev/null && pwd)"
if [ -z "$BB_TS_DIR" ] || [ ! -d "$BB_TS_DIR" ]; then
    echo "Error: Could not find barretenberg/ts directory relative to barretenberg-rs"
    exit 1
fi
echo "barretenberg-rs src: $BB_RS_SRC"
echo "barretenberg ts:     $BB_TS_DIR"

# --- Check if files already exist ---
if [ -f "$BB_RS_SRC/api.rs" ] && [ -f "$BB_RS_SRC/generated_types.rs" ]; then
    echo "Generated files already exist. Regenerating..."
fi

# --- Determine platform for bb binary download ---
ARCH=$(uname -m)
OS=$(uname -s)
case "${OS}-${ARCH}" in
    Darwin-arm64)  PLATFORM="arm64-darwin" ;;
    Darwin-x86_64) PLATFORM="amd64-darwin" ;;
    Linux-aarch64) PLATFORM="arm64-linux" ;;
    Linux-x86_64)  PLATFORM="amd64-linux" ;;
    *)
        echo "Error: Unsupported platform ${OS}-${ARCH}"
        exit 1
        ;;
esac

# --- Download bb binary ---
BB_URL="https://github.com/AztecProtocol/aztec-packages/releases/download/${BB_TAG}/barretenberg-${PLATFORM}.tar.gz"
BB_TMP_DIR=$(mktemp -d)
trap "rm -rf '$BB_TMP_DIR'" EXIT

echo "Downloading bb binary from $BB_URL..."
curl -sL -f -o "$BB_TMP_DIR/bb.tar.gz" "$BB_URL" || {
    echo "Error: Failed to download bb binary. Check that ${BB_TAG} release exists."
    exit 1
}

tar -xzf "$BB_TMP_DIR/bb.tar.gz" -C "$BB_TMP_DIR"
BB_BINARY=$(find "$BB_TMP_DIR" -name "bb" -type f | head -1)
if [ -z "$BB_BINARY" ] || [ ! -x "$BB_BINARY" ]; then
    echo "Error: bb binary not found in downloaded archive"
    exit 1
fi
echo "bb binary ready at $BB_BINARY"

# --- Install TS dependencies and run codegen ---
echo "Enabling corepack for Yarn 4.x..."
corepack enable 2>/dev/null || true

echo "Installing TypeScript dependencies..."
cd "$BB_TS_DIR"
yarn install 2>&1 | tail -5

echo "Running code generation..."
BB_BINARY_PATH="$BB_BINARY" yarn generate

# --- Verify output ---
if [ -f "$BB_RS_SRC/api.rs" ] && [ -f "$BB_RS_SRC/generated_types.rs" ]; then
    echo ""
    echo "Successfully generated:"
    echo "  - $BB_RS_SRC/api.rs"
    echo "  - $BB_RS_SRC/generated_types.rs"
    echo ""
    echo "You can now run: cargo build --features barretenberg"
else
    echo "Error: Generated files not found after codegen"
    exit 1
fi
