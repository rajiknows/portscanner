#!/bin/bash

set -e

BINARY_NAME="sk" # Replace with your desired binary name
INSTALL_PATH="/usr/local/bin"
PROJECT_NAME="portscanner"
REPO_URL="https://github.com/rajiknows/$PROJECT_NAME.git"

# Function to check if a command exists
command_exists() {
  command -v "$1" >/dev/null 2>&1
}

# Check if Rust is installed
if ! command_exists rustc; then
  echo "Rust is not installed. Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
else
  echo "Rust is already installed."
fi

# Check if the project directory already exists
if [ -d "$PROJECT_NAME" ]; then
  echo "Project directory '$PROJECT_NAME' already exists. Checking for updates..."
  cd $PROJECT_NAME
  git fetch origin
  LOCAL=$(git rev-parse HEAD)
  REMOTE=$(git rev-parse @{u})

  if [ "$LOCAL" != "$REMOTE" ]; then
    echo "New version detected. Pulling updates..."
    git pull origin main
  else
    echo "Already up to date."
  fi
else
  echo "Cloning the latest version of the $PROJECT_NAME repository..."
  git clone $REPO_URL
  cd $PROJECT_NAME
fi

# Build the project
echo "Building the project..."
cargo build --release

# Move the binary to the desired location
echo "Installing $BINARY_NAME to $INSTALL_PATH..."
sudo cp target/release/$BINARY_NAME $INSTALL_PATH/$BINARY_NAME


# Confirm the installation
if command_exists $BINARY_NAME; then
  echo "$BINARY_NAME has been installed successfully!"
  echo "You can now run it using '$BINARY_NAME'."
else
  echo "Failed to install $BINARY_NAME."
  exit 1
fi
