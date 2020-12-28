Control the GH-CJDS66 60MHz Signal Generator  
--------------------------------------------  
  
Copyright © 2020 Jeremy Carter <jeremy@jeremycarter.ca>  
  
By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.  
  
  
Usage: `cargo run --release -- -h`  
  
  
Purpose:  
-------  
Control one or many GH-CJDS66 60MHz Signal Generators from the
command line. Tested and working on Linux and Windows, and might
also work on Mac.  
  
This is an unofficial project which fully implements the official 
communication spec for the DDS Signal Generator and Counter device
known as the "Koolertron Upgraded 60MHz DDS Signal Generator Counter, 
High Precision Dual-channel Arbitrary Waveform Function Generator 
Frequency Meter 200MSa/s (60MHz) Model: GH-CJDS66-FU" (less a few spec 
errata, which you can read about in a commit message here:  
[713b026a4e10807d23f7436d26649dcc4c584019](https://gitlab.com/defcronyke/signal-gen-cjds66/-/commit/713b026a4e10807d23f7436d26649dcc4c584019))  
  
  
Device and USB Interface info:  
-----------------------------  
Manufacturer page with info on where to buy it: [https://www.koolertron.com/koolertron-upgraded-60mhz-dds-signal-generator-counterhigh-precision-dualchannel-arbitrary-waveform-function-generator-frequency-meter-200msas-60mhz-p-867.html](https://www.koolertron.com/koolertron-upgraded-60mhz-dds-signal-generator-counterhigh-precision-dualchannel-arbitrary-waveform-function-generator-frequency-meter-200msas-60mhz-p-867.html)  
  
Linux `lsusb` output:  
```shell
ID 1a86:7523 QinHeng Electronics CH340 serial converter
```  
  
  
HOW TO INSTALL AND USE:  
----------------------  
Binary releases may be available at some point, but this is written in 
the Rust programming language, which makes it easy enough to compile the 
project from source yourself, and it's recommended you do that anyway 
for less chance of problems.  
  
1. Install the Rust programming language toolchain from:  
[https://rustup.rs](https://rustup.rs)  
  
2. Make sure you have Git installed, and clone this project's git 
repository onto your computer:  
```shell
git clone https://gitlab.com/defcronyke/signal-gen-cjds66.git
cd signal-gen-cjds66
```  
  
3. Compile and run the program (with the -h flag to display the help 
output so you can see how to use it):  
```shell
cargo run --release -- -h
```  
  
4. You can now use the program the same way as above, and it will only be
recompiled as-needed if any changes are made to the source code. Note that
any command line switches and args must come after the two dashes "--", 
like where you see the -h flag above. You can also optionally omit the 
--release switch if you'd prefer to be using a debug build of the program:  
```shell
cargo run --release -- <any switches or args go here>
```  
  
  
UPDATE TO THE LATEST COMMIT OR USE A SPECIFIC NUMBERED VERSION:  
--------------------------------------------------------------  
If you cloned this project awhile ago and you'd like to update
to a newer version, run this command:  
```shell
git pull
```  
  
Numbered release versions (a.k.a. git "tags") of this project are intended 
to be more stable, and it's recommended for you to always run the
highest-numbered tag version listed in the project repo, rather than the newest
commit in the master branch, for best results. Using the master branch at a 
non-numbered version should be considered experimental. It's not recommended 
and results may vary.  
  
To find the currently available numbered versions of this project, run this 
command:  
```shell
git tag
```  
  
If there's any numbered versions available, it should output some versions, 
such as for example:  
```shell
v0.0.1
v0.1.0
```  
  
In the example above, v0.1.0 is the highest-numbered version available, 
so it should be preferred unless you have some personal reason not to 
prefer it. To use that version of the project, run this command:  
```shell
git checkout v0.1.0
```  
  
IMPORTANT: If you're running a numbered version and you'd like to update
the project with any newly-available code, switch back from the numbered
version to the master branch, then update with the git pull command, and
after that you can see any new available versions and switch to using them
as you like. For example:  
```shell
git checkout master
git pull
git tag
git checkout v0.1.1
```  
  
  
Tutorial - Design a Custom Wave:  
-------------------------------  
1. Download Waveform Manager Plus (this has been tested with v4.13):  
[https://www.aimtti.com/resources/waveform-manager-plus-v413](https://www.aimtti.com/resources/waveform-manager-plus-v413)  
  
2. Unzip it and install it with wine (it's Windows-only but works well 
in wine):  
```shell
wine start waveman.msi
```  
  
3. Run the program with wine:  
```shell
cd ~/".wine/drive_c/Program Files (x86)/Waveman"
wine waveman.exe
```  
  
4. Design a new waveform of amplitude 4096 and length 2048, and save 
it as format: `"WaveCAD *.wav"`  
  
5. Upload the wave to the device, saving it into slot 1:  
```shell
cargo run --release -- --wwc 1,<the-filename-here.wav>
```  
  
6. Set the device to use the custom wave you just uploaded, for channel
1 output:  
```shell
cargo run --release -- -a 1
```  
  
  
Tutorial - Upload an Audio File to the Device  
---------------------------------------------  
1. Using some program such as Audacity 
([https://www.audacityteam.org/](https://www.audacityteam.org/)), 
export your desired audio as a 16-bit .wav file, and it's recommended 
that you use a very low-quality project rate such as 8000 Hz, so you 
can fit a longer amount of the audio onto the device.  
  
2. Upload the .wav file as an arbitrary waveform onto the device, 
saving it in slot 1:  
```shell
cargo run --release -- --wwc 1,<the-filename-here.wav>
```  
  
IMPORTANT: The device can only store the first 2048 numbers contained 
in the .wav file, which is a very short duration of audio data. It's 
important that you provide a .wav file with at least 2048 numbers in 
it. At a project rate of 8000 Hz, that should require a stereo audio 
file with duration of at least 126.5 milliseconds. The extra data will 
be omitted from the upload to the device.  
  
