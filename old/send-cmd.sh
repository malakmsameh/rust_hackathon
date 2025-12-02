#!/bin/bash

# Direct command sender for ESP32-C3
# Usage: ./send-cmd.sh "your command here"

PORT="/dev/ttyUSB0"
BAUD="115200"

if [ -z "$1" ]; then
    echo "Usage: $0 'command'"
    echo ""
    echo "Examples:"
    echo "  $0 'set-wifi --mode=station --sta-ssid=42_Students --sta-password=12345678 --set-channel=6'"
    echo "  $0 'start --duration=120'"
    echo "  $0 'set-traffic --frequency-hz=10'"
    echo "  $0 'show-config'"
    exit 1
fi

CMD="$1"

echo "Sending: $CMD"
echo "$CMD" | sudo tee "$PORT" > /dev/null

sleep 0.5
echo "✓ Command sent!"
