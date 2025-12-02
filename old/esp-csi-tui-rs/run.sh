#!/bin/bash

# Run the ESP32 CSI TUI with proper permissions

cd "$(dirname "$0")" || exit 1

echo "🚀 Starting ESP32 CSI Monitor TUI..."
echo ""

# Check if device is connected
if [ ! -e /dev/ttyUSB0 ]; then
    echo "❌ ESP32 not found on /dev/ttyUSB0"
    echo ""
    echo "Checking available devices:"
    ls /dev/ttyUSB* 2>/dev/null || echo "No USB serial devices found"
    echo ""
    echo "Please:"
    echo "1. Plug in your ESP32-C3 via USB"
    echo "2. Wait a few seconds for it to be detected"
    echo "3. Try again"
    exit 1
fi

echo "✓ ESP32 detected at /dev/ttyUSB0"
echo ""

# Run with sudo (needed for serial port access)
echo "Running TUI (press Ctrl+C to exit)..."
echo ""

sudo ./target/debug/esp-csi-tui-rs
