signal-gen-cjds66  
=================  
Control the GH-CJDS66 60MHz Signal Generator  
--------------------------------------------  
  
[![docs](https://docs.rs/signal-gen-cjds66-lib/badge.svg)](https://docs.rs/signal-gen-cjds66-lib) [![crate](https://img.shields.io/crates/v/signal_gen_cjds66_lib)](https://crates.io/crates/signal-gen-cjds66-lib) [![pipeline status](https://gitlab.com/defcronyke/signal-gen-cjds66/badges/master/pipeline.svg)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/commits/master) [![coverage report](https://gitlab.com/defcronyke/signal-gen-cjds66/badges/master/coverage.svg)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/commits/master) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/LICENSE.md)  
  
https://gitlab.com/defcronyke/signal-gen-cjds66  
  
Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>  
  
MIT License  
  
By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.  
  
  
**Usage (Linux): `./signal-gen-cjds66 -h`**  
  
**Usage (Windows): `.\signal-gen-cjds66.exe -h`**  
  
**Usage (when building from source): `cargo run --release -- -h`**  
  
**[Full Usage Info (from the project Wiki)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/wikis/usage)**  
  
  
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
Manufacturer page with info on where to buy it:  
[https://www.koolertron.com/koolertron-upgraded-60mhz-dds-signal-generator-counterhigh-precision-dualchannel-arbitrary-waveform-function-generator-frequency-meter-200msas-60mhz-p-867.html](https://www.koolertron.com/koolertron-upgraded-60mhz-dds-signal-generator-counterhigh-precision-dualchannel-arbitrary-waveform-function-generator-frequency-meter-200msas-60mhz-p-867.html)  
  
Linux `lsusb` output:  
```shell
ID 1a86:7523 QinHeng Electronics CH340 serial converter
```  
  
  
HOW TO INSTALL AND USE:  
----------------------  
  
----------  
  
**NOTE:** After downloading the Linux binary version, before you can
run it you have to make it executable by running this command:  
```shell
chmod 755 signal-gen-cjds66
```  
  
**Download the latest release version here:**  
  
* **Linux x86_64 Binary:**
  - [signal-gen-cjds66 (v0.1.8 Linux Release Build)](https://gitlab.com/defcronyke/signal-gen-cjds66/builds/artifacts/v0.1.8/raw/target/release/signal-gen-cjds66?job=release-linux-x86_64)

  - [signal-gen-cjds66 (v0.1.8 Linux Debug Build)](https://gitlab.com/defcronyke/signal-gen-cjds66/builds/artifacts/v0.1.8/raw/target/debug/signal-gen-cjds66?job=debug-linux-x86_64)
  
* **Windows x86_64 Binary:**
  - [signal-gen-cjds66.exe (v0.1.8 Windows Release Build)](https://gitlab.com/defcronyke/signal-gen-cjds66/builds/artifacts/v0.1.8/raw/target/x86_64-pc-windows-gnu/release/signal-gen-cjds66.exe?job=release-windows-x86_64)

  - [signal-gen-cjds66.exe (v0.1.8 Windows Debug Build)](https://gitlab.com/defcronyke/signal-gen-cjds66/builds/artifacts/v0.1.8/raw/target/x86_64-pc-windows-gnu/debug/signal-gen-cjds66.exe?job=debug-windows-x86_64)
  
----------  
  
**Download the latest development version here:**  
  
(Built from the newest master branch commit which passed all the automated tests. 
See the **[Pipelines page](https://gitlab.com/defcronyke/signal-gen-cjds66/-/pipelines)** for build success/failure history.)  
  
* **Linux x86_64 Binary:**
  - [signal-gen-cjds66 (Development Linux Release Build)](https://gitlab.com/defcronyke/signal-gen-cjds66/builds/artifacts/master/raw/target/release/signal-gen-cjds66?job=release-linux-x86_64)

  - [signal-gen-cjds66 (Development Linux Debug Build)](https://gitlab.com/defcronyke/signal-gen-cjds66/builds/artifacts/master/raw/target/debug/signal-gen-cjds66?job=debug-linux-x86_64)
  
* **Windows x86_64 Binary:**
  - [signal-gen-cjds66.exe (Development Windows Release Build)](https://gitlab.com/defcronyke/signal-gen-cjds66/builds/artifacts/master/raw/target/x86_64-pc-windows-gnu/release/signal-gen-cjds66.exe?job=release-windows-x86_64)

  - [signal-gen-cjds66.exe (Development Windows Debug Build)](https://gitlab.com/defcronyke/signal-gen-cjds66/builds/artifacts/master/raw/target/x86_64-pc-windows-gnu/debug/signal-gen-cjds66.exe?job=debug-windows-x86_64)
  
The development version above has passed all of the automated tests, 
but it should be considered somewhat experimental, since it's not a 
numbered release version. Feel free to try it if you aren't concerned 
about potential bugs and you just want to get going quickly with the 
newest (possibly broken) version of the program.  
  
----------  
  
**Access the auto-generated documentation here:**  
  
* **HTML / Javascript Cargo Doc Format:**  
  
  - [signal-gen-cjds66-lib docs (View Release Version)](https://docs.rs/signal-gen-cjds66-lib)

  - [signal-gen-cjds66 docs (Download Release Version)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/jobs/artifacts/v0.1.8/download?job=docs)

  - [signal-gen-cjds66 docs (Download Development Version)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/jobs/artifacts/master/download?job=docs)
  
----------  
  
Binary releases for other platforms may be available at some point, 
but this is developed using the Rust programming language, which makes 
it easy enough to compile the project from source yourself if you need
to.  
  
**Here's how to compile signal-gen-cjds66 from source yourself, and run it easily with Rust's built-in `cargo run` command:**  
  
1. Install the Rust programming language toolchain from:  
[https://rustup.rs](https://rustup.rs)  
  
2. Make sure you have [Git](https://git-scm.com/) installed, 
and clone this project's git repository onto your computer:  
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
any command line switches and args must come after the two dashes `--`, 
like where you see the `-h` flag above. You can also optionally omit the 
`--release` switch if you'd prefer to be using a debug build of the 
program:  
```shell
cargo run --release -- <any switches or args go here>
```  
  
5. If you prefer to run the compiled program directly, instead of with the 
Rust toolchain's `cargo run` helper command, after running the above cargo 
command for the first time, you can find the compiled binary for this 
program in the `target/release` folder, which will have just been added
to the project root folder on your computer.  
  
You can run it like this on Linux:  
```shell
cd target/release
./signal-gen-cjds66 -h
```  
  
On Windows you can run it like this:  
```powershell
cd target\release
.\signal-gen-cjds66.exe -h
```  
  
6. You can add the `-v` switch to the end of any command to get a 
different and more verbose style of output that's useful for 
debugging, and to better understand what's happening when you run a 
command. For example, to get the verbose output while requesting 
the device's model number and serial number, run this command:  
```shell
cargo run --release -- --ms -v
```  

7. You can specify the device location if the default isn't correct 
for your computer or configuration. On Linux it defaults to 
`/dev/ttyUSB0`, and on Windows it defaults to `COM3`. You can change 
it or even specify multiple devices to control with the `-d` switch, 
like this on Linux:  
```shell
./signal-gen-cjds66 -d /dev/ttyUSB2 -d /dev/ttyUSB5
```  
  
Or on Windows you would do the same, but with names like `COM4` or 
`COM5`, like this:  
```powershell
.\signal-gen-cjds66.exe -d COM4 -d COM5
```  
  
**IMPORTANT:** When you specify multiple devices, all commands will 
be executed on the first device first, and then on the next device 
immediately afterwards.  
  
You can run this program with multiple command line switches all in 
one invocation, and when you do that, the commands will be executed 
in a predefined order, NOT the order which you add the switches in 
the command invocation.  
  
An attempt has been made to define execution in a reasonable order 
which should work for most use-cases, but if you need things to run 
in a different order, you'll need to run the program multiple times 
instead. Maybe you can write a script and call this program from 
inside there multiple times if you want it automated nicely, with 
things happening in a custom order.  
  
If you'd like to see the exact order that things will run in, you 
can take a look in the 
[src/main.rs](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/src/main.rs) 
file. The commands will run top-to-bottom in the order their 
corresponding functions appear in that source file. If you aren't 
sure which command line switches those functions correspond with, 
you can cross-reference the switch names with the entries of the same 
name listed in the file named 
[clap.yaml](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/clap.yaml).  
  
The execution order might change in newer versions of this program, 
if a better order is determined after receiving feedback from users 
and using it for longer, so please don't assume this execution order 
will never change between versions.  
  
  
Update To The Latest Commit Or Use A Specific Numbered Release Version:  
----------------------------------------------------------------------  
If you cloned this project awhile ago and you'd like to update to a newer 
version, run this command:  
```shell
git pull
```  
  
Numbered 
[release versions](https://gitlab.com/defcronyke/signal-gen-cjds66/-/releases)
of this project are intended to be more stable, and it's recommended for 
you to always run the highest-numbered release version listed in the project 
repo, rather than the newest commit in the master branch, for best results. 
Using the master branch at a non-numbered version should be considered 
experimental. It's not recommended and results may vary.  
  
To find the currently available numbered
[release versions](https://gitlab.com/defcronyke/signal-gen-cjds66/-/releases)
of this project, run this command:  
```shell
git tag
```  
  
If there are any numbered versions available, it should output some versions, 
such as for example:  
```shell
v0.1.1
v0.1.2
...
v0.1.6
v0.1.7
```  
  
In the example above, `v0.1.7` is the highest-numbered version available, 
so it should be preferred unless you have some personal reason not to 
prefer it. To use that version of the project, run this command:  
```shell
git checkout v0.1.7
```  
  
**IMPORTANT:** If you're running a numbered version and you'd like to update
the project with any newly-available code, switch back from the numbered
version to the master branch, then update with the git pull command, and
after that you can see any new available versions and switch to using them
as you like. For example:  
```shell
git checkout master
git pull
git tag
git checkout v0.1.8
```  
  
  
Tutorial - Design a Custom Wave:  
-------------------------------  
1. Download Waveform Manager Plus (this has been tested with v4.13):  
[https://www.aimtti.com/resources/waveform-manager-plus-v413](https://www.aimtti.com/resources/waveform-manager-plus-v413)  
  
2. Unzip and install it as normal. It's Windows-only but it works 
well on Linux if you use [wine](https://www.winehq.org).  
  
Linux example, installing with wine:  
```shell
wine start waveman.msi
```  
  
3. Run the program as normal, or if you're on Linux, run it with 
wine like this:  
```shell
cd ~/".wine/drive_c/Program Files (x86)/Waveman"
wine waveman.exe
```  
  
4. Design a new waveform of amplitude 4096 and length 2048, and save 
it as format: `"WaveCAD *.wav"`  
  
5. Upload the wave to the device, saving it into slot 2:  
```shell
cargo run --release -- --wwc 2,<the-filename-here.wav>
```  
  
6. Set the device to use the custom wave you just uploaded, from slot 2, 
for channel 1 output:  
```shell
cargo run --release -- -a 2
```  
  
  
Tutorial - Upload an Audio File to the Device:  
---------------------------------------------  
1. Using some program such as Audacity 
([https://www.audacityteam.org](https://www.audacityteam.org)), 
export your desired audio as a 16-bit .wav file, and it's recommended 
that you use a very low-quality project rate such as 8000 Hz, so you 
can fit a longer amount of the audio onto the device.  
  
2. Upload the .wav file as an arbitrary waveform onto the device, 
saving it in slot 1:  
```shell
cargo run --release -- --wwc 1,<the-filename-here.wav>
```  
  
3. Set the device to use the custom wave you just uploaded, from slot 1, 
for channel 2 output:  
```shell
cargo run --release -- -b 1
```  
  
**IMPORTANT:** The device can only store the first `2048` numbers 
contained in the `.wav` file, which is a very short duration of audio 
data. The `.wav` file should have at least `2048` numbers in it. At a 
project rate of `8000 Hz`, that should require a `stereo audio file` with 
duration of at least `126.5 milliseconds`. The extra data will be omitted 
from the upload to the device.  
  
  
Extra Info:  
----------  
**Rust crate signal-gen-cjds66-lib is available on crates.io:**  
  
  * The library powering this command line program is available as a 
  standalone Rust crate here:  
  [https://crates.io/crates/signal-gen-cjds66-lib](https://crates.io/crates/signal-gen-cjds66-lib)

  * You can use it to make your own Rust programs if you want, since 
  nearly all the signal generator's features are exposed through
  functions in this crate. Documentation and examples are available
  at the above link.

  * The easiest way to get started using the crate in your own project
  is by putting these lines in your Rust project's 
  [Cargo.toml](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/Cargo.toml) 
  file (double-check the `crates.io` link above though and substitute the
  latest available version of the crate into the text below):
  ```toml
  [dependencies]
  signal-gen-cjds66-lib = { version = "0.1" }
  ```

  * **IMPORTANT:** Note that this crate uses three-part version numbers, 
  such as `0.1.7`, but it's better to leave off the third part of the 
  version number in your 
  [Cargo.toml](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/Cargo.toml) 
  file, because that way you can more easily and automatically update to 
  the latest `0.1.x` versions, without having to update that third number 
  by hand. It would be annoying otherwise, since that third number may 
  need to change frequently.

  * The source code for the crate lives in the 
  [signal-gen-cjds66-lib/](https://gitlab.com/defcronyke/signal-gen-cjds66/-/tree/master/signal-gen-cjds66-lib)
  subfolder in this project. Take a look in there for more info and 
  examples on how to use it.  
  
  
**Helper scripts for common tasks:**  
  
  * Notice in the root folder of this project, and also in the 
  [signal-gen-cjds66-lib/](https://gitlab.com/defcronyke/signal-gen-cjds66/-/tree/master/signal-gen-cjds66-lib)
  subfolder, there are several convenience helper scripts in both 
  `Linux bash shell` (\*.sh) and `Windows batch file` (\*.bat) formats.

    - You can take a look inside them to get an idea of what they do,
    and you may enjoy using them sometimes, because it requires less
    typing to do a desired task. They are mostly there for the 
    convenience of this project's author, so they won't be thoroughly
    explained, but most of them are very simple, so you can probably
    guess what they do based on their filenames and their contents.  
  
  * There are a few more `Linux bash shell` scripts in the 
  [examples/](https://gitlab.com/defcronyke/signal-gen-cjds66/-/tree/master/examples)
  subfolder, and one in particular called 
  [examples/audio-to-wavecad.sh](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/examples/audio-to-wavecad.sh)
  may be interesting to you if you're on Linux. It isn't really
  needed for anything important, but it does show how to do a few
  things in Bash which the Rust program is doing in Rust, so maybe
  you can learn a bit from it. It may be removed in the future,
  once its features are fully added to the Rust program.

  * Some other neat scripts in 
  [examples/](https://gitlab.com/defcronyke/signal-gen-cjds66/-/tree/master/examples)
  which you might want to examine or try out are:  
  
    - [examples/clear-user-waves.sh](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/examples/clear-user-waves.sh) - 
    Erase all of the currently saved user-defined arbitrary waveform
    slots, by overwriting every slot with a flat line wave, centered
    at 0 volts.

    - [examples/load-random-waves.sh](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/examples/load-random-waves.sh) - 
    Overwrite all of the currently saved user-defined arbitrary 
    waveform slots with new randomly generated waveforms, using 
    a simple, faster, and insecure source of randomness.

    - [examples/load-crypto-random-waves.sh](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/examples/load-crypto-random-waves.sh) - 
    Overwrite all of the currently saved user-defined arbitrary 
    waveform slots with new randomly generated waveforms, using 
    the operating system's built-in more secure source of 
    randomness.  
  
