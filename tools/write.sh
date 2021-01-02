#!/bin/bash -e
# Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>
#
# By using this software, you agree to the LICENSE TERMS 
# outlined in the file titled LICENSE.md contained in the 
# top-level directory of this project. If you don't agree
# to the LICENSE TERMS, you aren't allowed to use this
# software.
#
#
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
