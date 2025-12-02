#!/bin/bash

# Diagnostic script to check ESP32 connectivity

echo "🔍 ESP32 Diagnostics"
echo "===================="
echo ""

# Check device exists
echo "1. Checking USB device..."
if [ -e /dev/ttyUSB0 ]; then
    echo "   ✓ /dev/ttyUSB0 found"
    ls -la /dev/ttyUSB0
else
    echo "   ✗ /dev/ttyUSB0 not found!"
    echo "   Try: ls /dev/ttyUSB*"
    exit 1
fi

echo ""
echo "2. Testing basic connection with espflash..."
echo "   (This will show device boot messages)"
echo "   Press Ctrl+C after a few seconds to exit"
echo ""

sudo /home/tuqa-saeed/.cargo/bin/espflash monitor --port /dev/ttyUSB0 2>&1 | head -50

echo ""
echo "✓ Diagnostics complete"
echo ""
echo "If you see boot messages above, the device IS responsive."
echo "If you see nothing, the device may be hung or unflashed."
