#!/bin/bash

# Configuration
EXEC_PATH="/app/web-server"          # Path to the current executable
NEW_EXEC_PATH="/app/new-server"      # Path to the new executable
TRIGGER_FILE="/app/new-server.txt"   # Trigger file to signal an update

# Function to start the executable
start_executable() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - Starting executable..."
    nohup "$EXEC_PATH" > /dev/stdout 2>&1 &
    EXEC_PID=$!
    echo "$(date '+%Y-%m-%d %H:%M:%S') - Executable started with PID $EXEC_PID"
}

# Function to stop the executable
stop_executable() {
    if [ -n "$EXEC_PID" ]; then
        echo "$(date '+%Y-%m-%d %H:%M:%S') - Stopping executable with PID $EXEC_PID..."
        kill "$EXEC_PID"
        wait "$EXEC_PID" 2>/dev/null
        echo "$(date '+%Y-%m-%d %H:%M:%S') - Executable stopped."
        EXEC_PID=""
    else
        echo "$(date '+%Y-%m-%d %H:%M:%S') - No executable process found to stop."
    fi
}

# Function to perform the update
perform_update() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - Update triggered by $TRIGGER_FILE."

    # Stop the current executable
    stop_executable

    # Replace the executable
    if [ -f "$NEW_EXEC_PATH" ]; then
        echo "$(date '+%Y-%m-%d %H:%M:%S') - Replacing $EXEC_PATH with $NEW_EXEC_PATH..."
        mv "$NEW_EXEC_PATH" "$EXEC_PATH"
        chmod +x "$EXEC_PATH"
        echo "$(date '+%Y-%m-%d %H:%M:%S') - Replacement complete."
    else
        echo "$(date '+%Y-%m-%d %H:%M:%S') - New executable $NEW_EXEC_PATH not found."
        return
    fi

    # Remove the trigger file
    rm -f "$TRIGGER_FILE"
    echo "$(date '+%Y-%m-%d %H:%M:%S') - Removed trigger file."

    # Restart the executable
    start_executable
}

# Trap SIGTERM and SIGINT to gracefully stop the executable
trap 'echo "$(date '+%Y-%m-%d %H:%M:%S') - Caught termination signal. Shutting down..."; stop_executable; exit 0' SIGTERM SIGINT

# Initial start
start_executable

# Ensure the directory exists (optional, as /app should exist)
mkdir -p /app

# Monitor the trigger file for updates
echo "$(date '+%Y-%m-%d %H:%M:%S') - Monitoring for $TRIGGER_FILE to trigger updates..."
while true; do
    if [ -f "$TRIGGER_FILE" ]; then
        perform_update
    fi
    # Poll every second; adjust as needed
    sleep 1
done