#!/usr/bin/env bash
# Pull the connl client source and build the executable.
#
# Usage:
#   ./build_from_source.sh [-d DIR] [-b BRANCH] [-i]
#     -d DIR     directory to clone into (default: ./bindlocal-client)
#     -b BRANCH  branch/tag to build (default: repository default branch)
#     -i         install the binary to /usr/local/bin after building
set -euo pipefail

REPO_URL="https://github.com/khajer/bindlocal-client.git"
TARGET_DIR="bindlocal-client"
BRANCH=""
INSTALL=0

while getopts "d:b:ih" opt; do
  case "$opt" in
    d) TARGET_DIR="$OPTARG" ;;
    b) BRANCH="$OPTARG" ;;
    i) INSTALL=1 ;;
    h|*) sed -n '2,9p' "$0"; exit 0 ;;
  esac
done

# Require git and a Rust toolchain
command -v git >/dev/null 2>&1 || { echo "error: git is not installed" >&2; exit 1; }
if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo not found. Install Rust first: https://rustup.rs" >&2
  exit 1
fi

# Pull the code (clone, or update an existing checkout)
if [ -d "$TARGET_DIR/.git" ]; then
  echo "==> Updating existing checkout in $TARGET_DIR"
  git -C "$TARGET_DIR" fetch --all --tags
  [ -n "$BRANCH" ] && git -C "$TARGET_DIR" checkout "$BRANCH"
  git -C "$TARGET_DIR" pull --ff-only || true
else
  echo "==> Cloning $REPO_URL into $TARGET_DIR"
  if [ -n "$BRANCH" ]; then
    git clone --branch "$BRANCH" "$REPO_URL" "$TARGET_DIR"
  else
    git clone "$REPO_URL" "$TARGET_DIR"
  fi
fi

# Build
echo "==> Building release binary"
(cd "$TARGET_DIR" && cargo build --release)

BIN="$TARGET_DIR/target/release/connl"
echo "==> Built: $BIN"

if [ "$INSTALL" -eq 1 ]; then
  echo "==> Installing to /usr/local/bin/connl"
  if [ -w /usr/local/bin ]; then
    cp "$BIN" /usr/local/bin/connl
  else
    sudo cp "$BIN" /usr/local/bin/connl
  fi
fi

echo "Done. Run: $BIN 3000"
