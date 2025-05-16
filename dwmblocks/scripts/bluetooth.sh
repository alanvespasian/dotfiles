#!/bin/bash

# Bluetooth status for dwmblocks (checks for ANY connected device)
# Output: Connected / Disconnected

# Check if Bluetooth is enabled
if ! bluetoothctl show | grep -q "Powered: yes"; then
    echo "Off"
    exit 0
fi

# Check if ANY device is connected
if bluetoothctl devices | awk '{print $2}' | while read -r mac; do
    if bluetoothctl info "$mac" | grep -q "Connected: yes"; then
        echo "Connected"
        exit 0  # Exit early if any device is connected
    fi
done; then
    : # Do nothing (exit 0 already printed "Connected")
else
    echo "Disconnected"
fi
