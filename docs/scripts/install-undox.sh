#!/usr/bin/env bash
# Install or use a cached undox binary (built from source).
#
# Usage:
#   ./docs/scripts/install-undox.sh [git-ref]
#
# The binary is cached at .undox/bin/undox. Set UNDOX_CACHE_DIR to override.
# In CI, point the cache at a directory covered by actions/cache so subsequent
# runs skip the build entirely.
#
# Requires: git, cargo (Rust toolchain)

set -euo pipefail

REF="${1:-v0.1.8}"
CACHE_DIR="${UNDOX_CACHE_DIR:-$(git rev-parse --show-toplevel)/.undox}"
BIN_DIR="${CACHE_DIR}/bin"
BIN="${BIN_DIR}/undox"

# If the cached binary exists and matches the requested ref, reuse it.
STAMP="${CACHE_DIR}/.ref"
if [[ -x "$BIN" && -f "$STAMP" ]]; then
  CACHED_REF=$(cat "$STAMP")
  if [[ "$CACHED_REF" == "$REF" ]]; then
    echo "undox ($REF) already cached at $BIN"
    exit 0
  fi
fi

mkdir -p "$BIN_DIR"

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

echo "Cloning undox at ref ${REF}..."
git clone --depth 1 --branch "$REF" https://github.com/undox-rs/undox.git "$TMPDIR/undox"

echo "Building undox..."
cargo install --path "$TMPDIR/undox" --root "$CACHE_DIR" --force

echo "$REF" > "$STAMP"
echo "Installed undox ($REF) to $BIN"
