#!/usr/bin/env bash
set -e

APP_NAME="connl"
VERSION="0.1.1"
REPO_URL="https://github.com/khajer/bindlocal-client/releases/download/v${VERSION}"

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)   FILE="${APP_NAME}-${VERSION}-linux.tar.gz" ;;
    Darwin*)  FILE="${APP_NAME}-${VERSION}-macos-tar.gz" ;;
    *)        echo "❌ Unsupported OS: ${OS}"; exit 1 ;;
esac

URL="${REPO_URL}/${FILE}"
TMP_DIR="$(mktemp -d)"
INSTALL_DIR="/usr/local/bin"

echo "🔍 Detected OS: $OS"
echo "⬇️  Downloading ${FILE}..."
curl -L "$URL" -o "${TMP_DIR}/${FILE}"

echo "📦 Extracting..."
tar -xzf "${TMP_DIR}/${FILE}" -C "${TMP_DIR}"

echo "🚀 Installing ${APP_NAME} to ${INSTALL_DIR}..."
sudo mv "${TMP_DIR}/${APP_NAME}" "${INSTALL_DIR}/${APP_NAME}"
sudo chmod +x "${INSTALL_DIR}/${APP_NAME}"

echo "🧹 Cleaning up..."
rm -rf "${TMP_DIR}"

echo "✅ Installation complete!"
echo "👉 Try running: ${APP_NAME} --help"