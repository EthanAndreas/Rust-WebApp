#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing required dependencies...${NC}"

# Check and install Cargo if not installed
if ! command -v cargo &> /dev/null; then
    echo -e "${GREEN}Cargo not found. Installing Rust (includes Cargo)...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo -e "${GREEN}Cargo found. Updating Rust...${NC}"
    rustup update
fi

# Install PostgreSQL, PostgreSQL contrib, and PostgreSQL client
echo -e "${GREEN}Installing PostgreSQL, PostgreSQL contrib, and PostgreSQL client...${NC}"
if [ -x "$(command -v apt-get)" ]; then
    sudo apt-get update
    sudo apt-get install -y postgresql postgresql-contrib postgresql-client
elif [ -x "$(command -v dnf)" ]; then
    sudo dnf install -y postgresql postgresql-contrib
elif [ -x "$(command -v yum)" ]; then
    sudo yum install -y postgresql postgresql-contrib
elif [ -x "$(command -v pacman)" ]; then
    sudo pacman -Sy --noconfirm postgresql postgresql-old-upgrade
else
    echo "Please install postgresql, postgresql-contrib, and postgresql-client manually."
fi

# Install wasm32 target
echo -e "${GREEN}Installing wasm32 target...${NC}"
rustup target add wasm32-unknown-unknown

# Install Trunk
echo -e "${GREEN}Installing Trunk...${NC}"
cargo install trunk

echo -e "${GREEN}Installation complete!${NC}"
echo -e "You can now:"
echo -e "1. Run backend: ${GREEN}cargo run${NC}"
echo -e "2. Run frontend: ${GREEN}cd frontend && trunk serve${NC}"