#!/bin/bash
# An example of how to convert part of a 16-bit .wav audio file
# to a compatible WaveCAD file. This clamps all values to the
# supported range of 0 - 4095, and saves a chunk of 2048 values
# from the audio file into the correctly formatted .wav and 
# .txt files.
#
# With this method, you can design WaveCAD files using a DAW or
# other audio production software, and then upload them to the
# device.

for n in `cat ../examples/defcronyke-sunlink-16bit.wav | od -An -vtd2 -w2 | head -n 2048`; do 
    if [ $n -lt -2047 ]; then 
        echo 0; 
    
    elif [ $n -gt 2047 ]; then 
        echo 4095; 
    
    else
        expr $n + 2048
    fi
done > ../examples/defcronyke-sunlink-16bit-wavecad.txt

pwd="$PWD"
cd ..

cargo run -- --txt-to-wav examples/defcronyke-sunlink-16bit-wavecad.txt

cd "$pwd"
