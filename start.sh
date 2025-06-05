#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to cleanup background processes
cleanup() {
    echo -e "\n${GREEN}Shutting down servers...${NC}"
    # Kill any process using port 8080
    kill $(lsof -t -i:8080) 2>/dev/null
    if [ ! -z "$BACKEND_PID" ]; then
        kill $BACKEND_PID 2>/dev/null
    fi
    exit 0
}

# Function to check if port is in use
is_port_used() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null ; then
        return 0
    else
        return 1
    fi
}

# Set up trap to catch Ctrl+C and other termination signals
trap cleanup SIGINT SIGTERM

# Kill any existing process using port 8080
if is_port_used 8080; then
    echo -e "${RED}Port 8080 is in use. Killing existing process...${NC}"
    kill $(lsof -t -i:8080) 2>/dev/null
    sleep 2
fi

echo -e "${GREEN}Starting backend server...${NC}"
# Build and run the Rust backend application in background
cd backend
cargo run &
BACKEND_PID=$!

# Wait for backend to start
echo -e "${GREEN}Waiting for backend to start...${NC}"
sleep 2

# Check if backend is running
if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo -e "${RED}Backend failed to start${NC}"
    cleanup
    exit 1
fi

echo -e "${GREEN}Starting frontend server...${NC}"
# Start the frontend application using Trunk
cd ../frontend
if [ ! -f "styles.css" ]; then
    echo -e "${GREEN}Creating styles.css...${NC}"
    echo ".auth-container { max-width: 400px; margin: 2rem auto; padding: 2rem; }" > styles.css
fi

trunk serve --port 3000  # Using port 3000 for frontend

# Cleanup when trunk exits
cleanup