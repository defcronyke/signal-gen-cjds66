signal-gen-cjds66-lib  
=====================  
Control the GH-CJDS66 60MHz Signal Generator  
--------------------------------------------  
  
[![docs](https://docs.rs/signal-gen-cjds66-lib/badge.svg)](https://docs.rs/signal-gen-cjds66-lib) [![crate](https://img.shields.io/crates/v/signal_gen_cjds66_lib)](https://crates.io/crates/signal-gen-cjds66-lib) [![gitlab ci/cd status](https://gitlab.com/defcronyke/signal-gen-cjds66/badges/master/pipeline.svg)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/pipelines) [![coverage report](https://gitlab.com/defcronyke/signal-gen-cjds66/badges/master/coverage.svg)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/jobs) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/signal-gen-cjds66-lib/LICENSE.md)  
  
https://gitlab.com/defcronyke/signal-gen-cjds66  
  
Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>  
  
MIT License  
  
By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.  
  
Purpose:  
-------  
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
  
Basic Usage Code Example:  
------------------------  
**Getting Started:**  
  * This library is published as a Rust crate on crates.io, so 
  that you can use it in your own projects easily:  
  [https://crates.io/crates/signal-gen-cjds66-lib](https://crates.io/crates/signal-gen-cjds66-lib)

  * The crate's auto-generated documentation can be viewed on docs.rs here:  
  [https://docs.rs/signal-gen-cjds66-lib](https://docs.rs/signal-gen-cjds66-lib)

  * You can also download the latest development version docs here if you
  want the docs for the master branch (development version) of the crate.
  This version also includes the docs for the associated command line program:  
  [signal-gen-cjds66 docs (Development Version)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/jobs/artifacts/master/download?job=docs)

  * To use this crate, put the following lines in your Rust project's
  `Cargo.toml` file (but substitute the newest version number 
  listed at the crates.io link above so you're using the 
  latest version):
  ```toml
  [dependencies]
  signal-gen-cjds66-lib = { version = "0.1" }
  ```

  * **IMPORTANT:** Note that this crate uses three-part version numbers, 
  such as `0.1.7`, but it's better to leave off the third part of the 
  version number in your `Cargo.toml` file, because that way you can more 
  easily and automatically update to the latest `0.1.x` versions, without 
  having to update that third number by hand. It would be annoying 
  otherwise, since that third number may need to change frequently.  
  
**Code Example - Print the device's model number and serial number:**  
`examples/basic-usage.rs`  
```rust
/*! A basic usage example for the library.  
  
Control one signal generator device on 
Linux or Windows. Print the device's 
model number and serial number.
*/

extern crate signal_gen_cjds66_lib;
extern crate clap;

use signal_gen_cjds66_lib::command::*;
use signal_gen_cjds66_lib::error;
use signal_gen_cjds66_lib::error::From;
use signal_gen_cjds66_lib::serial::*;

use clap::ErrorKind;

/// The main entrypoint.
fn main() {
	/* Call the main logic function, and save the
	result, which will either be Ok(0) on success,
	or some type of error.
	*/
	let res = real_main();

	/* Exit the program, returning an appropriate
	exit code to the parent shell or execution 
	environment. This is important so whoever
	ran this program (a bash or batch script maybe)
	can check if anything went wrong while the 
	program was running, and react accordingly.
	*/
	std::process::exit(
		/* If there was an error, get the proper
		exit code for it, to return on exit.
		*/
		error::handle_exit(res)
			.map_or_else(
				|e| e.code,	// Get the numeric error exit code (non-zero).
				
				|code| code,	// Or, get the success exit code (0).
			)
	);
}

/** The main logic area. Rust is a bit weird so it's
better in this case to have this function separate
from main().
*/
fn real_main() -> Result<i32, error::Error> {
	// An error variable, to specify if an error happened.
	let mut err: Option<error::Error> = None;

	/* Choose the serial device location depending 
	on which operating system you're using.  

	HINT: You can put more than one device location 
	in each of these arrays if you'd like to control 
	multiple devices in sequence.  

	NOTE: The values listed below are the default values
	used by the first connected serial device on my 
	Linux and Windows systems. If you get a connection
	error when you run this code, make sure your signal
	generator is plugged into your computer and powered
	on.  

	If you still have a connection error, you'll probably 
	need to find the correct path for your system and 
	change the values below so they	are pointing at the 
	correct device.
	*/
	let devices = 
		if cfg!(unix) {	// If running on Linux (or UNIX).
			vec!["/dev/ttyUSB0".to_string()]
			
		} else if cfg!(windows) {	// If running on Windows.
			vec!["COM3".to_string()]
			
		} else {	// If unsure of the operating system.
			vec!["/dev/ttyUSB0".to_string()]
		};

	/* Iterate over each device path you configured above,
	and perform operations on each device sequentially.
	This example is only connecting to one device, but you
	can add more above if you want.
	*/ 
	for device in &devices {
		/* A new error varialbe (an alias really), borrowing a mutable 
		reference to our error variable from above, and storing it in
		an immutable binding.
		*/
		let err = &mut err;

		/* Establish a serial connection to the signal generator device,
		with all the correct communication options configured. The
		second parameter to the `SerialPortType::new()` function should 
		always be false unless we are making a fake connection for 
		testing purposes, which is currently only useful for the crate's 
		automated test suite.
		*/
		println!("\nOpening communication link with device: {}\n", device);

		let opened = SerialPortType::new(device, false).map_or_else(
			// If opening the device failed, return an error.
			|e| {
				Err(error::Error::with_description(&format!("(device: {}): {}: make sure the device is connected and turned on, or try specifying a different device path with -d /path/to/device", device, e), clap::ErrorKind::Io))
			},

			/* If the device was opened successfully, go on to run some
			commands which operate on the device.
			*/
			|mut port| {
				// A new error variable, just for this inner scope.
				let mut err: Option<error::Error> = None;

				/* Get the model and serial numbers from the device
				(verbose version). The second parameter on the 
				`get_model_and_serial()` function enables 
				"verbose output mode" when you pass in a number 
				higher than 0.  
				  
				Note that this function, and all the other ones 
				which communicate with the signal generator, are 
				defined in the `command` module, and are imported 
				into the global scope in this file by this line 
				near the top of the file:  
				`use signal_gen_cjds66_lib::command::*;`
				*/
				println!("\nGetting the device's model number and serial number, with verbose output mode enabled...\n");

				get_model_and_serial(&mut port, 1)
					.map_err(
						|e| {
							// If there was a problem, report the error.
							err = Some(error::Error::from_clap_error(e));
							println!("{}", err.as_ref().unwrap());
						}
					)
					.unwrap();

				/* Get model and serial number from the device
				(non verbose version). Notice the second 
				parameter to the `get_model_and_serial()` 
				function below is 0 this time, which turns off
				"verbose output mode".  
				  
				NOTE: Every function in the `command` module 
				accepts the same type of "verbose" parameter,
				although it's not always the second parameter,
				rather, it's usually the last one.
				*/
				println!("\nGetting the device's model number and serial number, with verbose output mode disabled...\n");

				get_model_and_serial(&mut port, 0)
					.map_err(
						|e| {
							// If there was a problem, report the error.
							err = Some(error::Error::from_clap_error(e));
							println!("{}", err.as_ref().unwrap());
						}
					)
					.unwrap();

				println!("");	// Line break for nicer output.

				/* Return Ok(0) on success, or an error if there were
				any.
				*/
				err.map_or_else(|| { Ok(0) }, |v| { Err(v) })
			},
		);

		/* If we can't connect to a certain device, skip it and continue 
		with any remaining devices we haven't tried yet.
		*/
		if opened.is_err() {
			*err = opened.map_or_else(
				|e| {
					// If there was a problem, report the error.
					if e.kind == ErrorKind::Io {
						println!("{}", e);
					}

					Some(e)
				},

				|_val| None,
			);

			continue;
		}
	}

	/* If there was a problem during any of the above operations, 
	return the error to the parent function.
	*/
	if err.is_some() {
		Err(err.unwrap())

	} else {	// Otherwise, return Ok(0) to indicate success.
		Ok(0)
	}
}
```  
  
  
**NOTE:** The above example can be found in the file [signal-gen-cjds66-lib/examples/basic-usage.rs](https://gitlab.com/defcronyke/signal-gen-cjds66/-/blob/master/signal-gen-cjds66-lib/examples/basic-usage.rs),
and you can try running it if you want, by issuing the following command from within this 
[signal-gen-cjds66-lib/](https://gitlab.com/defcronyke/signal-gen-cjds66/-/tree/master/signal-gen-cjds66-lib) directory:  
```shell
cargo run --release --example basic-usage
```  
  
