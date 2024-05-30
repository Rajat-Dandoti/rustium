#!/bin/bash

# Install Rust if not already installed
if ! command -v rustup &> /dev/null
then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
fi

# Update Rust to the latest stable version
rustup update stable

# Install necessary Rust components
rustup component add clippy
rustup component add rustfmt

# Install Chromedriver Manager
cargo install chromedriver-manager

# Install any additional system dependencies
sudo apt-get update
sudo apt-get install -y libssl-dev chromium-chromedriver

# Ensure the Chromedriver binary is in the PATH
if ! command -v chromedriver &> /dev/null
then
    echo "Chromedriver could not be found. Please ensure it is installed correctly."
    exit 1
else
    echo "Chromedriver is installed."
fi

echo "Setup complete."
