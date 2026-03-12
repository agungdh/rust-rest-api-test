#!/bin/bash

echo "Starting Employee API server..."
cd /home/agungdh/RustroverProjects/untitled

# Kill any existing server
pkill -f employee-api 2>/dev/null || true
sleep 1

# Start server in background
cargo run > /tmp/employee-api.log 2>&1 &
SERVER_PID=$!

# Wait for server to start
echo "Waiting for server to start..."
sleep 3

# Check if server is running
if ! kill -0 $SERVER_PID 2>/dev/null; then
    echo "Server failed to start!"
    cat /tmp/employee-api.log
    exit 1
fi

echo "Server started with PID: $SERVER_PID"

# Run tests
echo "Running API tests..."
bash scripts/test_api.sh

# Kill server
echo -e "\nStopping server..."
kill $SERVER_PID 2>/dev/null || true

echo "Done!"
