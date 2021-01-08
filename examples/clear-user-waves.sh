#!/bin/bash -e
# Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>
#
# By using this software, you agree to the LICENSE TERMS 
# outlined in the file titled LICENSE.md contained in the 
# top-level directory of this project. If you don't agree
# to the LICENSE TERMS, you aren't allowed to use this
# software.

# Clear all of the user-defined arbitrary waveform save 
# slots on the signal generator device, by overwriting 
# each wave with a flat line centered at 0 volts (each of 
# the waveform's 2048 values being set to the number 2048 
# in every save slot).
#
# WARNING: This will erase all of your custom waveforms on
# your signal generator device. The previous data cannot
# ever be recovered. Use this script at your own risk.

# The main function of this script.
clear_user_waves() {
  # Default to running the program with the cargo run command.
  COMMAND="cargo run --release -q --"

  # Optionally override the signal-gen-cjds66 program command 
  # to use, with the value of the first command line argument
  # that's passed to this script, if present.
  if [ $# -gt 0 ]; then
    COMMAND="$1"
  fi

  echo -e "\nUsing command: \"$COMMAND\"\n"

  echo -e "Clearing all saved user waveforms on the signal \
generator device. This usually takes about 1 or 2 \
minutes...\n\n"

  # Create a stdout alias so we can echo to stdout in the 
  # subshell below.
  exec 3>&1

  # Run the main logic with a timer to see how long the
  # operation takes once it's finished.
  time \
  $(\
    $COMMAND -a 1   # Switch channel1 to the Arbitrary01 slot.
    $COMMAND -b 1   # Switch channel2 to the Arbitrary01 slot.

    # For each user-defined arbitrary waveform save slot.
    for i in {1..60}; do
      # Print this message to stdout of the parent shell using 
      # our alias from above.
      echo "Clearing user-defined arbitrary waveform save slot: $i of 60" >&3
      
      if [ $i -gt 1 ]; then   # Skip the first iteration.
        $COMMAND -a $i  # Switch channel1 to the next Arbitrary slot.
        $COMMAND -b $i  # Switch channel2 to the next Arbitrary slot.
      fi

      # Upload ASCII waveform data from stdin to the device.
      $COMMAND --wws $i <<< \
        `for j in {0..2047}; do
          echo 2048   # Output the value 2048, 2048 times, one value per line.
        done`
    done

    # Switch channel1 to the Arbitrary01 slot.
    $COMMAND -a 1
    
    # Switch channel2 to the Arbitrary01 slot.
    $COMMAND -b 1\
  )

  # Save the exit code from the previous command, so we can
  # return it at the end of this bash function.
  return_value=$?

  echo  # Output an extra newline to make things look nicer.

  # Return the exit code of the main command in this function.
  return $return_value
}

# Run the main function of this script.
clear_user_waves $@
