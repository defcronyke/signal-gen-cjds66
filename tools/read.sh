#!/bin/bash -e
# Read from a serial device.

if [ $# -lt 1 ]; then
	DEVICE="/dev/ttyUSB0"
else
	DEVICE="$1"
fi

stty -F "$DEVICE" 115200 raw

cat -v < "$DEVICE"
