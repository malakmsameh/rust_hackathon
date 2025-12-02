#!/bin/bash

# Simple interactive shell that talks to ESP32
# Much simpler and more reliable than trying to do it all in Rust

PORT="/dev/ttyUSB0"
BAUD="115200"

echo "╭─ ESP32 CSI Monitor (Simple Interface) ─╮"
echo "│ Device: $PORT @ $BAUD                    │"
echo "│ Press Ctrl+D to exit                    │"
echo "╰────────────────────────────────────────╯"
echo ""
echo "Quick commands: h=help w=sniffer c=start 1=traffic 2=csi s=config"
echo ""

# Configure serial port properly
sudo stty -F $PORT ispeed $BAUD ospeed $BAUD -crtscts

# Start monitoring and provide interactive shell
(
  echo "Connecting to device..."
  while IFS= read -p "> " -r cmd; do
    case "$cmd" in
      h) cmd="help" ;;
      w) cmd="set-wifi --mode=sniffer --set-channel=6" ;;
      c) cmd="start --duration=120" ;;
      1) cmd="set-traffic --frequency-hz=10" ;;
      2) cmd="set-csi" ;;
      s) cmd="show-config" ;;
      q|exit) break ;;
      "") continue ;;
    esac

    echo "Sending: $cmd"
    echo "$cmd" | sudo tee $PORT > /dev/null
    sleep 0.2

    echo "Response:"
    sudo timeout 1 cat $PORT 2>/dev/null | head -20
    echo ""
  done
) < <(cat)
