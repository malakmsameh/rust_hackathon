#!/bin/bash

# Simple test: send a command and see if we get a response

PORT="/dev/ttyUSB0"
BAUD="115200"

echo "Testing direct serial communication..."
echo ""

# Use stty to configure port
sudo stty -F $PORT ispeed $BAUD ospeed $BAUD
sleep 1

# Send help command
echo "Sending: help"
echo "help" | sudo tee $PORT > /dev/null
sleep 0.5

# Try to read response
echo ""
echo "Response:"
sudo timeout 2 cat $PORT || true

echo ""
echo "✓ Test complete"
