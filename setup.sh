#!/bin/bash

# Setup script for SceneReleaseParser Rust project
# This script installs mise and sets up the Rust toolchain

set -e

echo "Setting up SceneReleaseParser Rust project..."

# Check if mise is installed
if ! command -v mise &> /dev/null; then
    echo "Installing mise..."
    curl https://mise.run | sh
    
    # Add mise to PATH for this session
    export PATH="$HOME/.local/bin:$PATH"
    
    # Activate mise
    eval "$($HOME/.local/bin/mise activate zsh 2>/dev/null || $HOME/.local/bin/mise activate bash 2>/dev/null || echo 'export PATH=\"$HOME/.local/bin:$PATH\"')"
else
    echo "mise is already installed"
fi

# Install Rust through mise
echo "Installing Rust toolchain via mise..."
mise install

# Verify installation
echo "Verifying Rust installation..."
rustc --version
cargo --version

echo ""
echo "Setup complete!"
echo ""
echo "To activate mise in your shell, add this to your ~/.zshrc or ~/.bashrc:"
echo "  eval \"\$(\$HOME/.local/bin/mise activate zsh)\"  # for zsh"
echo "  eval \"\$(\$HOME/.local/bin/mise activate bash)\"  # for bash"
echo ""
echo "Or run: source setup.sh"
echo ""
echo "Then you can run: cargo test"


