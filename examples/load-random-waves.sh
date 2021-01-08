#!/bin/bash -e
# Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>
#
# By using this software, you agree to the LICENSE TERMS 
# outlined in the file titled LICENSE.md contained in the 
# top-level directory of this project. If you don't agree
# to the LICENSE TERMS, you aren't allowed to use this
# software.

# Overwrite all of the user-defined arbitrary waveform save 
# slots on the signal generator device with new randomly 
# generated waveforms. This uses the bash shell's built-in
# $RANDOM variable, which is a simple, faster, and insecure 
# source of randomness.
#
# WARNING: This will erase all of your custom waveforms on
# your signal generator device. The previous data cannot
# ever be recovered. Use this script at your own risk.

# The main function of this script.
load_random_waves() {
  # Default to running the program with the cargo run command.
  COMMAND="cargo run --release -q --"

  # Optionally override the signal-gen-cjds66 program command 
  # to use, with the value of the first command line argument
  # that's passed to this script, if present.
  if [ $# -gt 0 ]; then
    COMMAND="$1"
  fi

  echo -e "\nUsing command: \"$COMMAND\"\n"

  echo -e "Generating new random waveforms and loading 
them onto the signal generator device, overwriting all 
currently saved user waveforms. This usually takes about 
8 or 9 minutes...\n"

  # Create a stdout alias so we can echo to stdout in the 
  # subshell below.
  exec 3>&1

  # Run the main logic with a timer to see how long the
  # operation takes once it's finished.
  time \
  $(\
    # Print this message to stdout of the parent shell using 
    # our alias from above.
    echo -e "Displaying user-defined arbitrary waveform save slot:\t1 of 60" >&3

    $COMMAND -a 1   # Switch channel1 to the Arbitrary01 slot.
    $COMMAND -b 1   # Switch channel2 to the Arbitrary01 slot.

    # For each user-defined arbitrary waveform save slot.
    for i in {1..60}; do
      if [ $i -gt 2 ]; then   # Don't run this block for the first two iterations.        
        # Print this message to stdout of the parent shell using 
        # our alias from above.
        echo -e "Displaying user-defined arbitrary waveform save slot:\t`expr $i - 1` of 60" >&3
      
        $COMMAND -a `expr $i - 1`  # Switch channel1 to the next Arbitrary slot.
        $COMMAND -b `expr $i - 1`  # Switch channel2 to the next Arbitrary slot.
      fi

      # Print this message to stdout of the parent shell using 
      # our alias from above.
      echo -e "Generating new random waveform and saving it in slot:\t$i of 60" >&3

      # Upload ASCII waveform data from stdin to the device.
      $COMMAND --wws $i <<< \
        `for j in {0..2047}; do
          # Output random values in the range of 0 - 4095,
          # 2048 times, one value per line.
          echo $RANDOM % 4096 | bc
        done`

        # Print an extra newline to make the output nicer.
        echo >&3
    done

    # Switch channel1 to the Arbitrary60 slot for a brief preview.
    $COMMAND -a 60
    
    # Switch channel2 to the Arbitrary60 slot for a brief preview.
    $COMMAND -b 60

    # Wait a bit so you can briefly see the new waveform.
    sleep 1

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
load_random_waves $@
