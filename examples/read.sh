#!/bin/bash -e
# Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>
#
# By using this software, you agree to the LICENSE TERMS 
# outlined in the file titled LICENSE.md contained in the 
# top-level directory of this project. If you don't agree
# to the LICENSE TERMS, you aren't allowed to use this
# software.

# Read from a serial device.

if [ $# -lt 1 ]; then
	DEVICE="/dev/ttyUSB0"
else
	DEVICE="$1"
fi

stty -F "$DEVICE" 115200 raw

cat -v < "$DEVICE"
