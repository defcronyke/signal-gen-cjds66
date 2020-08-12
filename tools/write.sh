#!/bin/bash -e
# Write to a serial device.

if [ $# -eq 1 ]; then
    DEVICE="/dev/ttyUSB0"
    COMMAND="$1"
elif [ $# -eq 2 ]; then
    DEVICE="$1"
    COMMAND="$2"
else
    echo "Usage: $0 [device] \"command\""
    exit 1
fi

stty -F "$DEVICE" 115200 raw

printf "$COMMAND" > "$DEVICE"
