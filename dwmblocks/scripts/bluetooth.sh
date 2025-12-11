#!/bin/sh

# Get the connected Bluetooth device name
name=$(bluetoothctl devices Connected | cut -d ' ' -f 3-)

if [ -z "$name" ]; then
    echo "No device connected"
else
    echo "$name"
fi

