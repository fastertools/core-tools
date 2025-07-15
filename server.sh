#!/bin/bash

# Improved server management script for core-tools
# This script ensures clean server management and handles orphaned processes

# Function to kill any process on port 3000
kill_port_3000() {
    echo "Checking for processes on port 3000..."
    local pids=$(lsof -ti:3000 2>/dev/null)
    if [ ! -z "$pids" ]; then
        echo "Found process(es) on port 3000: $pids"
        echo "Killing process(es)..."
        echo $pids | xargs kill -9 2>/dev/null || true
        sleep 1
        echo "Port 3000 cleared"
    else
        echo "No process found on port 3000"
    fi
}

# Function to start the server
start_server() {
    # Always ensure port is free before starting
    kill_port_3000
    
    echo "Starting Spin server..."
    ./test_server
}

# Function to stop the server
stop_server() {
    # First try the normal stop
    if [ -f "spin.pid" ]; then
        ./test_server stop
        sleep 1
    fi
    
    # Then ensure port is really free
    kill_port_3000
    
    # Clean up any leftover PID file
    rm -f spin.pid
}

# Function to restart the server
restart_server() {
    echo "Restarting server..."
    stop_server
    sleep 1
    start_server
}

# Function to check server status
status_server() {
    local pids=$(lsof -ti:3000 2>/dev/null)
    if [ ! -z "$pids" ]; then
        echo "Server is running on port 3000 (PID: $pids)"
        if [ -f "spin.pid" ]; then
            local saved_pid=$(cat spin.pid)
            echo "Saved PID in spin.pid: $saved_pid"
        fi
    else
        echo "Server is not running on port 3000"
    fi
}

# Main script logic
case "${1:-start}" in
    start)
        start_server
        ;;
    stop)
        stop_server
        ;;
    restart)
        restart_server
        ;;
    status)
        status_server
        ;;
    clean-start)
        # Force clean start - useful when things are really messed up
        echo "Performing clean start..."
        kill_port_3000
        rm -f spin.pid spin_*.log
        sleep 1
        start_server
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|status|clean-start}"
        echo "  start       - Start the server (kills any existing process on port 3000)"
        echo "  stop        - Stop the server"
        echo "  restart     - Stop and start the server"
        echo "  status      - Check if server is running"
        echo "  clean-start - Kill port 3000, remove logs, and start fresh"
        exit 1
        ;;
esac