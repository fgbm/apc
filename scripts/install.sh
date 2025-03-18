#!/bin/sh
set -e

detect_arch() {
  arch=$(uname -m)
  case $arch in
    x86_64)
      echo "amd64"
      ;;
    arm64|aarch64)
      echo "arm64"
      ;;
    *)
      echo "Unsupported architecture: $arch" >&2
      exit 1
      ;;
  esac
}

detect_os() {
  os=$(uname -s)
  case $os in
    Linux)
      echo "linux"
      ;;
    Darwin)
      echo "macos"
      ;;
    *)
      echo "Unsupported OS: $os" >&2
      exit 1
      ;;
  esac
}

get_latest_version() {
  curl -s https://api.github.com/repos/fgbm/apc/releases/latest |
    grep '"tag_name":' |
    sed -E 's/.*"([^"]+)".*/\1/'
}

main() {
  ARCH=$(detect_arch)
  OS=$(detect_os)

  VERSION=$(get_latest_version)
  VERSION_NUM=${VERSION#v}

  echo "Installing APC $VERSION for $OS-$ARCH..."

  if [ "$OS" = "linux" ]; then
    FILENAME="apc-linux-amd64"
  elif [ "$OS" = "macos" ]; then
    FILENAME="apc-macos-amd64"
  fi

  INSTALL_DIR="/usr/local/bin"
  if [ ! -w "$INSTALL_DIR" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
  fi

  DOWNLOAD_URL="https://github.com/fgbm/apc/releases/download/$VERSION/$FILENAME"

  echo "Downloading from $DOWNLOAD_URL..."
  curl -L "$DOWNLOAD_URL" -o "$INSTALL_DIR/apc"
  chmod +x "$INSTALL_DIR/apc"

  echo "APC $VERSION installed successfully to $INSTALL_DIR/apc"

  if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "Warning: $INSTALL_DIR is not in your PATH. You might want to add it."
    if [ "$INSTALL_DIR" = "$HOME/.local/bin" ]; then
      echo "You can add it by running:"
      echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc && source ~/.bashrc"
    fi
  fi
}

main
