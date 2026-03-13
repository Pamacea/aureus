#!/bin/bash
# Aureus One-Line Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/Pamacea/aureus/main/install.sh | sh

set -e

REPO="Pamacea/aureus"
BINARY_NAME="aureus"
INSTALL_DIR="$HOME/.cargo/bin"
FALLBACK_DIR="$HOME/.local/bin"

echo "⚡ Aureus Installer"
echo "==================="
echo

# Check if cargo is available
if command -v cargo &>/dev/null; then
    echo "📦 Installing via cargo..."
    cargo install --git https://github.com/$REPO aureus
    echo
    echo "✓ Installed via cargo"
else
    echo "⚠️  cargo not found"
    echo
    echo "To install Aureus, you need Rust toolchain:"
    echo "  1. Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "  2. Run: cargo install aureus"
    exit 1
fi

# Initialize for Claude Code
if command -v aureus &>/dev/null; then
    echo
    echo "🔧 Initializing for Claude Code..."
    aureus init --global
fi

echo
echo "✓ Installation complete!"
echo
echo "Verify with:"
echo "  aureus --version"
echo "  aureus suggest"
