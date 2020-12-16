#!/bin/bash -e
# An example of how to convert part of a 16-bit .wav audio file
# to a compatible WaveCAD file. This clamps all values to the
# supported range of 0 - 4095, and saves a chunk of 2048 values
# from the beginning of the audio file into the correctly 
# formatted .txt file. It also saves a new WaveCAD .wav file,
# which is just the original audio .wav file's first 4096 bytes.
#
# You can already use regular audio .wav files as input for the 
# Rust program anyway, however, this script has the benefit of
# outputting a new WaveCAD .wav file of the correct length to be
# able to be edited in some waveform editing software that 
# WaveCAD .wav files. So if you use this first, you can have a 
# smaller WaveCAD .wav file with just the data you need in it. 
# This feature will probably be implemented in the Rust program 
# at some point, so this is mostly just a proof-of-concept bash 
# script version of some things the Rust program already does.
#
# With this method, you can design WaveCAD files using a DAW or
# other audio production software, and then generate the .txt
# file that's needed if you want to upload your wave to the
# device, as well as a .wav WaveCAD file which you can further
# edit in a wave editing program that supports the WaveCAD 
# format.
#
# If you manually edit the WaveCAD file, you can run it through
# our Rust program to regenerate the .txt file if you want.
# Just go one directory up so you're in the project root, and
# then run this command:
#
#   cargo run --release -- --wav-to-txt examples/defcronyke-sunlink-16bit-8000hz-wavecad.wav
#
# To upload your wave from the .txt file to the first save slot 
# on the device, you can run this command from the project root:
#
#   cargo run --release -- --wws 1 < examples/defcronyke-sunlink-16bit-8000hz-wavecad.txt
#
# Or you can upload using the WaveCAD .wav file instead, but 
# it's less efficient. Run this command from the project root:
#
#   cargo run --release -- --wwc 1,examples/defcronyke-sunlink-16bit-8000hz-wavecad.wav
#

# The main function of this script.
audio_to_wavecad() {
    # Optionally accepts 2 arguments: the input audio file,
    # and the output WaveCAD file. Both must have a .wav
    # file extension.
    if [ $# -gt 0 -a $# -ne 2 ]; then
        echo "usage: $0 [<in_file.wav> <out_file.wav>]"
        return 1
    fi

    # Default input file. Pass a different one as the 
    # first argument to the script if you want.
    IN_FILE="../examples/defcronyke-sunlink-16bit-8000hz.wav"
    if [ $# -eq 2 ]; then
        IN_FILE="$1"
    fi
    
    echo "input audio file: \"$IN_FILE\""

    IN_FILE_TXT_BASE=`basename "$IN_FILE"`
    IN_FILE_WAV_EXT=`echo ".${IN_FILE_TXT_BASE#*.}"`

    if [ "$IN_FILE_WAV_EXT" != ".wav" ]; then
        echo "error: the input file (first argument to this script) must have a .wav file extension"
        return 2
    fi

    if [ ! -f "$IN_FILE" ]; then
        echo "error: the input file (first argument to this script) was not found"
        return 3
    fi

    # Default output file. Pass a different one as the 
    # second argument to the script if you want.
    OUT_FILE="../examples/defcronyke-sunlink-16bit-8000hz-wavecad.wav"
    if [ $# -eq 2 ]; then
        OUT_FILE="$2"
    fi

    OUT_FILE_TXT_DIR="`dirname \"$OUT_FILE\"`/"
    OUT_FILE_TXT_BASE=`basename "$OUT_FILE"`
    OUT_FILE_TXT_NO_EXT=`echo "${OUT_FILE_TXT_BASE%.*}"`
    OUT_FILE_WAV_EXT=`echo ".${OUT_FILE_TXT_BASE#*.}"`
    OUT_FILE_TXT_EXT=".txt"

    if [ "$OUT_FILE_WAV_EXT" != ".wav" ]; then
        echo "error: the output file (second argument to this script) must have a .wav file extension"
        return 4
    fi

    OUT_FILE_TXT="$OUT_FILE_TXT_DIR$OUT_FILE_TXT_NO_EXT$OUT_FILE_TXT_EXT"

    # Create a text file from the audio file, containing the first
    # 2048 numbers, one on each line, adjusting the offset to make
    # all numbers positive by adding 2048, and then clamping them 
    # to the device's supported range of 0 - 4095.
    for n in `cat "$IN_FILE" | od -An -vtd2 -w2 | head -n 2048`; do 
        if [ $n -lt -2047 ]; then 
            echo 0; 
        
        elif [ $n -gt 2047 ]; then 
            echo 4095; 
        
        else
            expr $n + 2048
        fi
    done > "$OUT_FILE_TXT"

    echo "output text file: \"$OUT_FILE_TXT\""

    # Truncate the audio .wav file to the correct length
    # for a WaveCAD file for the device (2048 signed 
    # 16-bit integers, which is 4096 bytes).
    cat "$IN_FILE" | head -c 4096 > "$OUT_FILE"

    RETURN_CODE=$?

    echo "output WaveCAD file: \"$OUT_FILE\""

    return $RETURN_CODE
}

# Call the main function of this script.
audio_to_wavecad $@

EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "exited with error: exit code: $EXIT_CODE"
fi

# Exit with the proper exit code.
exit $EXIT_CODE
