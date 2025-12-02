#!/bin/bash

# ESP32-C3 CSI Configuration Helper Script
# Makes it easy to configure the device without typing long commands

PORT="${1:-/dev/ttyUSB0}"
BAUD="115200"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to send command to device
send_command() {
    local cmd="$1"
    echo -e "${BLUE}Sending: ${cmd}${NC}"
    echo "$cmd" | sudo tee /dev/$( ls -la "$PORT" | awk '{print $NF}' ) > /dev/null 2>&1 || \
    { echo "$cmd" | sudo socat - "$PORT,b$BAUD"; }
}

# Function to display menu
show_menu() {
    echo -e "\n${GREEN}=== ESP32-C3 CSI Configuration Tool ===${NC}"
    echo "1. Connect to WiFi (Station mode)"
    echo "2. Start Sniffer mode"
    echo "3. Generate WiFi traffic"
    echo "4. Start CSI collection"
    echo "5. Show current configuration"
    echo "6. Stop collection"
    echo "7. Custom command"
    echo "8. Open serial monitor"
    echo "9. Exit"
    echo -e "${YELLOW}Select option (1-9):${NC} "
}

# Function to connect to WiFi
connect_wifi() {
    echo -e "\n${BLUE}WiFi Connection Setup${NC}"
    read -p "Enter SSID: " ssid
    read -sp "Enter Password: " password
    echo ""
    read -p "Enter Channel (default 6): " channel
    channel=${channel:-6}

    # Replace spaces with underscores if needed
    ssid_formatted="${ssid// /_}"

    cmd="set-wifi --mode=station --sta-ssid=${ssid_formatted} --sta-password=${password} --set-channel=${channel}"
    send_command "$cmd"
    echo -e "${GREEN}WiFi configuration sent!${NC}"
}

# Function to start sniffer mode
start_sniffer() {
    echo -e "\n${BLUE}Starting Sniffer Mode${NC}"
    read -p "Enter Channel (default 6): " channel
    channel=${channel:-6}

    cmd="set-wifi --mode=sniffer --set-channel=${channel}"
    send_command "$cmd"
    echo -e "${GREEN}Sniffer mode started!${NC}"
}

# Function to generate traffic
generate_traffic() {
    echo -e "\n${BLUE}WiFi Traffic Generation${NC}"
    read -p "Enter frequency in Hz (default 10): " freq
    freq=${freq:-10}

    cmd="set-traffic --frequency-hz=${freq}"
    send_command "$cmd"
    echo -e "${GREEN}Traffic generation started at ${freq}Hz!${NC}"
}

# Function to start CSI collection
start_collection() {
    echo -e "\n${BLUE}CSI Collection Setup${NC}"
    read -p "Enter duration in seconds (default 120): " duration
    duration=${duration:-120}

    cmd="start --duration=${duration}"
    send_command "$cmd"
    echo -e "${GREEN}CSI collection started for ${duration} seconds!${NC}"
}

# Function to show config
show_config() {
    echo -e "\n${BLUE}Fetching current configuration...${NC}"
    send_command "show-config"
}

# Function to stop collection
stop_collection() {
    echo -e "\n${BLUE}Stopping collection...${NC}"
    send_command "stop"
    echo -e "${GREEN}Collection stopped!${NC}"
}

# Function for custom command
custom_command() {
    echo -e "\n${BLUE}Enter custom command:${NC}"
    read -p "> " cmd
    send_command "$cmd"
}

# Function to open serial monitor
open_monitor() {
    echo -e "\n${BLUE}Opening serial monitor on ${PORT}...${NC}"
    echo "Press Ctrl+A then X to exit minicom"
    sleep 1
    sudo minicom -D "$PORT" -b "$BAUD"
}

# Main loop
main() {
    echo -e "${GREEN}ESP32-C3 Configuration Tool${NC}"
    echo "Port: $PORT"
    echo ""

    while true; do
        show_menu
        read -r choice

        case $choice in
            1) connect_wifi ;;
            2) start_sniffer ;;
            3) generate_traffic ;;
            4) start_collection ;;
            5) show_config ;;
            6) stop_collection ;;
            7) custom_command ;;
            8) open_monitor ;;
            9)
                echo -e "${GREEN}Goodbye!${NC}"
                exit 0
                ;;
            *) echo -e "${YELLOW}Invalid option. Try again.${NC}" ;;
        esac
    done
}

# Check if port exists
if [ ! -e "$PORT" ]; then
    echo -e "${YELLOW}Warning: Port $PORT not found${NC}"
    echo "Checking for available ports..."
    ls /dev/ttyUSB* 2>/dev/null || echo "No USB devices found"
    echo ""
    read -p "Enter correct port (or press Enter to use $PORT): " new_port
    PORT="${new_port:-$PORT}"
fi

main
