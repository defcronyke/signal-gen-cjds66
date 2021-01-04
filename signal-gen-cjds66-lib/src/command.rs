/* Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>

By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.
*/

/*! Functions which communicate with the device over its USB-serial 
interface, and expose nearly all of its remotely-accessible features. 
This contains the support code and logic for almost everything the 
device can do.  
  
Each function accepts a "verbose" integer parameter, which can be
set to a number above 0 to enable a different and more verbose style 
of output which is useful for debugging, or to help understand what 
is happening behind-the-scenes when a function is run. Currently 
any number above 0 produces the exact same verbose output, so you
can just use 1 for "verbose" output, and 0 for "not verbose" output.
*/

extern crate byteorder;
extern crate serial;

use crate::protocol::*;
use crate::util::*;

use serial::prelude::*;
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::str;

use byteorder::{ByteOrder, LittleEndian};
use clap::{Error, ErrorKind};

/** Get the model number of the device. */
pub fn get_model(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	if verbose > 0 {
		println!("\nRequesting machine model number:\n{}", GET_MODEL);
	}

	let inbuf: Vec<u8> = GET_MODEL.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_MODEL_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		let res2: Vec<&str> = res.split("=").collect();

		if res2.len() < 2 {
			return Err(Error::with_description(
				&format!(
					"unexpected response from device: missing separator ({}): {}",
					COMMAND_SEPARATOR, res
				),
				ErrorKind::ValueValidation,
			));
		}

		if res2[1].len() < 4 {
			return Err(Error::with_description(
				&format!(
					"unexpected response from device: missing terminator ({}): {}",
					COMMAND_END, res
				),
				ErrorKind::ValueValidation,
			));
		}

		let res3: &str = &res2[1][0..res2[1].len() - 3];

		println!("model:\t{}", res3);
	}

	Ok(res.to_string())
}

/** Get the serial number of the device. */
pub fn get_serial(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	if verbose > 0 {
		println!(
			"\nRequesting machine serial number:\n{}",
			GET_SERIAL
		);
	}

	let inbuf: Vec<u8> = GET_SERIAL.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_SERIAL_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		let res2: Vec<&str> = res.split("=").collect();

		if res2.len() < 2 {
			return Err(Error::with_description(
				&format!(
					"unexpected response from device: missing separator ({}): {}",
					COMMAND_SEPARATOR, res
				),
				ErrorKind::ValueValidation,
			));
		}

		if res2[1].len() < 4 {
			return Err(Error::with_description(
				&format!(
					"unexpected response from device: missing terminator ({}): {}",
					COMMAND_END, res
				),
				ErrorKind::ValueValidation,
			));
		}

		let res3: &str = &res2[1][0..res2[1].len() - 3];

		println!("serial:\t{}", res3);
	}

	Ok(res.to_string())
}

/** Get the model number and the serial number of the device. */
pub fn get_model_and_serial(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	if verbose > 0 {
		println!("\nRequesting machine model and serial number:\n{}", GET_MODEL_AND_NUMBER);
	}

	let inbuf: Vec<u8> = GET_MODEL_AND_NUMBER.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_MODEL_AND_NUMBER_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		let res2_parts: Vec<&str> = res.split("=").collect();

		if res2_parts.len() < 3 {
			return Err(Error::with_description(
				&format!(
					"unexpected response from device: missing separator ({}): {}",
					COMMAND_SEPARATOR, res
				),
				ErrorKind::ValueValidation,
			));
		}

		let res_model_parts: Vec<&str> = res2_parts[1].split(".").collect();

		if res_model_parts.len() < 2 {
			return Err(Error::with_description(
				&format!(
					"unexpected response from device: missing separator ({}): {}",
					".", res
				),
				ErrorKind::ValueValidation,
			));
		}

		let res_model = res_model_parts[0];

		let res_number_parts: Vec<&str> = res2_parts[2].split(".").collect();

		if res_number_parts.len() < 2 {
			return Err(Error::with_description(
				&format!(
					"unexpected response from device: missing separator ({}): {}",
					".", res
				),
				ErrorKind::ValueValidation,
			));
		}

		let res_number = res_number_parts[0];

		println!("model:\t{}", res_model);
		println!("serial:\t{}", res_number);
	}

	Ok(res.to_string())
}

/** Set the device's output state (on or off) for channels 1 and 2.  
  
"sco" parameter:  
```ignore
	channel 1 on, channel 2 on:  
		"1,1" | "11" | "on,on" | "1" | "on"  
  
	channel 1 off, channel 2 off:  
		"0,0" | "00" | "off,off" | "0" | "off"  
  
	channel 1 on, channel 2 off:  
		"1,0" | "10" | "on,off"  
  
	channel 1 off, channel 2 on:  
		"0,1" | "01" | "off,on"  
```
*/
pub fn set_channel_output(
	mut port: &mut Box<dyn SerialPort>,
	sco: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let res: Result<String, clap::Error>;

	match sco {
		"1,1" | "11" | "on,on" | "1" | "on" => {
			res = set_channel_output_inner(&mut port, true, true, verbose);
		}

		"0,0" | "00" | "off,off" | "0" | "off" => {
			res = set_channel_output_inner(&mut port, false, false, verbose);
		}

		"1,0" | "10" | "on,off" => {
			res = set_channel_output_inner(&mut port, true, false, verbose);
		}

		"0,1" | "01" | "off,on" => {
			res = set_channel_output_inner(&mut port, false, true, verbose);
		}

		_ => {
			res = Err(Error::with_description(
				&format!("unsupported value passed to \"-o\" argument: {}", sco),
				ErrorKind::InvalidValue,
			));
		}
	}

	res
}

fn set_channel_output_inner(
	port: &mut Box<dyn SerialPort>,
	ch1: bool,
	ch2: bool,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &str;

	// Supported states.
	if ch1 && ch2 {
		// Both on.
		command = SET_CHANNEL_OUTPUT_BOTH_ON;
	} else if !ch1 && !ch2 {
		// Both off.
		command = SET_CHANNEL_OUTPUT_BOTH_OFF;
	} else if ch1 && !ch2 {
		// ch1 on, ch2 off.
		command = SET_CHANNEL_OUTPUT_CH1_ON_CH2_OFF;
	} else if !ch1 && ch2 {
		// ch1 off, ch2 on.
		command = SET_CHANNEL_OUTPUT_CH1_OFF_CH2_ON;
	} else {
		return Err(Error::with_description(
			"unsupported input condition",
			ErrorKind::ArgumentConflict,
		));
	}

	if verbose > 0 {
		println!(
			"\nSetting channel output: ch1={} and ch2={}:\n{}",
			ch1, ch2, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_CHANNEL_OUTPUT_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Get the device's channel output state (on or off) for channel 1 and channel 2.  
  
Return Value (Ok Result):  
```ignore
channel 1 off, channel 2 on:
"0,1"

channel 1 on, channel 2 on:
"1,1"
```
*/
pub fn get_channel_output(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = GET_CHANNEL_OUTPUT;

	if verbose > 0 {
		println!("\nGetting channel output:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_CHANNEL_OUTPUT_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3 = res3_parts[0];

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res3.to_string())
}

/** Set the device to use a certain named or numbered waveform preset 
for a specific channel's output. You can choose one of these presets 
either by name or by the number listed next to the name below.  
  
"preset" parameter:
```ignore
Waveform names that are accepted, along with their corresponding numbers:
"0":  "sine" | "sin"
"1":  "square" | "sq"
"2":  "pulse" | "pul"
"3":  "triangle" | "tri"
"4":  "partialsine" | "partial-sine" | "parsine" | "par-sine" | "parsin" | "par-sin" | "psine" | "p-sine" | "psin" | "p-sin"
"5":  "cmos" | "cm"
"6":  "dc"
"7":  "halfwave" | "half-wave" | "hw" | "h-w"
"8":  "fullwave" | "full-wave" | "fw" | "f-w"
"9":  "pos-ladder" | "posladder" | "pos-lad" | "poslad" | "positive-ladder" | "positiveladder" | "pl"
"10": "neg-ladder" | "negladder" | "neg-lad" | "neglad" | "negative-ladder" | "negativeladder" | "nl"
"11": "noise" | "nois" | "noi" | "no" | "n"
"12": "exp-rise" | "exprise" | "e-r" | "er" | "e-rise" | "erise" | "e-ris" | "eris"
"13": "exp-decay" | "expdecay" | "e-d" | "ed" | "e-decay" | "edecay" | "e-dec" | "edec"
"14": "multi-tone" | "multitone" | "m-t" | "mt" | "m-tone" | "mtone"
"15": "sinc" | "sc"
"16": "lorenz" | "loren" | "lor" | "lz"
```
*/
pub fn set_waveform_preset(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	preset: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let res: Result<String, clap::Error>;

	match preset.parse::<u64>() {
		Ok(preset) => {
			match preset {
				0..=16 => {
					res = set_waveform_preset_inner(&mut port, chan, preset, verbose);
				}

				_ => {
					res = Err(Error::with_description(&format!("unsupported value passed to \"set waveform\" argument (must be 0-16): {}", preset), ErrorKind::InvalidValue));
				}
			}
		}

		Err(_e) => {
			match preset {
				"sine" | "sin" => {
					res = set_waveform_preset_inner(&mut port, chan, 0, verbose);
				}

				"square" | "sq" => {
					res = set_waveform_preset_inner(&mut port, chan, 1, verbose);
				}

				"pulse" | "pul" => {
					res = set_waveform_preset_inner(&mut port, chan, 2, verbose);
				}

				"triangle" | "tri" => {
					res = set_waveform_preset_inner(&mut port, chan, 3, verbose);
				}

				"partialsine" | "partial-sine" | "parsine" | "par-sine" | "parsin" | "par-sin"
				| "psine" | "p-sine" | "psin" | "p-sin" => {
					res = set_waveform_preset_inner(&mut port, chan, 4, verbose);
				}

				"cmos" | "cm" => {
					res = set_waveform_preset_inner(&mut port, chan, 5, verbose);
				}

				"dc" => {
					res = set_waveform_preset_inner(&mut port, chan, 6, verbose);
				}

				"halfwave" | "half-wave" | "hw" | "h-w" => {
					res = set_waveform_preset_inner(&mut port, chan, 7, verbose);
				}

				"fullwave" | "full-wave" | "fw" | "f-w" => {
					res = set_waveform_preset_inner(&mut port, chan, 8, verbose);
				}

				"pos-ladder" | "posladder" | "pos-lad" | "poslad" | "positive-ladder"
				| "positiveladder" | "pl" => {
					res = set_waveform_preset_inner(&mut port, chan, 9, verbose);
				}

				"neg-ladder" | "negladder" | "neg-lad" | "neglad" | "negative-ladder"
				| "negativeladder" | "nl" => {
					res = set_waveform_preset_inner(&mut port, chan, 10, verbose);
				}

				"noise" | "nois" | "noi" | "no" | "n" => {
					res = set_waveform_preset_inner(&mut port, chan, 11, verbose);
				}

				"exp-rise" | "exprise" | "e-r" | "er" | "e-rise" | "erise" | "e-ris" | "eris" => {
					res = set_waveform_preset_inner(&mut port, chan, 12, verbose);
				}

				"exp-decay" | "expdecay" | "e-d" | "ed" | "e-decay" | "edecay" | "e-dec"
				| "edec" => {
					res = set_waveform_preset_inner(&mut port, chan, 13, verbose);
				}

				"multi-tone" | "multitone" | "m-t" | "mt" | "m-tone" | "mtone" => {
					res = set_waveform_preset_inner(&mut port, chan, 14, verbose);
				}

				"sinc" | "sc" => {
					res = set_waveform_preset_inner(&mut port, chan, 15, verbose);
				}

				"lorenz" | "loren" | "lor" | "lz" => {
					res = set_waveform_preset_inner(&mut port, chan, 16, verbose);
				}

				_ => {
					res = Err(Error::with_description(&format!("unsupported value passed to \"set waveform\" argument (must be 0-16): {}", preset), ErrorKind::InvalidValue));
				}
			}
		}
	}

	res
}

fn set_waveform_preset_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	preset: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_WAVEFORM_PRESET_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_WAVEFORM_PRESET_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if preset > 16 {
		return Err(Error::with_description(
			"Unsupported waveform preset number. Must be 0-16.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN, COMMAND_SET, chan_out, COMMAND_SEPARATOR, preset, COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting waveform preset: ch{}={}:\n{}",
			chan, preset, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_WAVEFORM_PRESET_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Get the current output waveform preset for a 
specific channel.  
  
Return Value (Ok Result):  
```ignore
Waveform preset number to name mappings:
"0":  sine
"1":  square
"2":  pulse
"3":  triangle
"4":  partial-sine
"5":  cmos
"6":  dc
"7":  half-wave
"8":  full-wave
"9":  pos-ladder
"10": neg-ladder
"11": noise
"12": exp-rise
"13": exp-decay
"14": multi-tone
"15": sinc
"16": lorenz

Arbitrary waveform presets:
"101" - "160": arbitrary wave 1 - arbitrary wave 60
```
*/
pub fn get_waveform_preset(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = GET_WAVEFORM_PRESET_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = GET_WAVEFORM_PRESET_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_GET,
		chan_out,
		COMMAND_SEPARATOR,
		GET_WAVEFORM_PRESET_ARG,
		COMMAND_END,
	);

	if verbose > 0 {
		println!("\nGetting waveform preset: ch{}:\n{}", chan, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_WAVEFORM_PRESET_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3 = res3_parts[0];

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res3.to_string())
}

/** Set the device to use a user-defined arbitrary waveform 
preset by number, for a specific channel's output.  
  
"preset" parameter:
```ignore
Arbitrary waveform preset 1 (a.k.a. preset 101):
"1"

Arbitrary waveform preset 60 (a.k.a. preset 160):
"60"
```
*/
pub fn set_waveform_preset_arbitrary(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	preset: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let res: Result<String, clap::Error>;

	match preset.parse::<u64>() {
		Ok(preset) => match preset {
			1..=60 => {
				res = set_waveform_preset_arbitrary_inner(&mut port, chan, preset, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set arbitrary waveform\" argument (must be 1-60): {}", preset), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set arbitrary waveform\" argument (must be 1-60): {}: {}", preset, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_waveform_preset_arbitrary_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	preset: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_WAVEFORM_PRESET_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_WAVEFORM_PRESET_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if preset < 1 || preset > 60 {
		return Err(Error::with_description(
			"Unsupported waveform preset number. Must be 1-60.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		chan_out,
		COMMAND_SEPARATOR,
		preset + 100,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting waveform preset: ch{}={}:\n{}",
			chan, preset, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_WAVEFORM_PRESET_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's output frequency for a particular channel,
in microhertz (µHz).  
  
"amount" parameter:
```ignore
"0.01" - "80000000.0"
```
*/
pub fn set_frequency_microhertz(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set frequency uHz\" argument (must be 0.01-80000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.01 && amount <= 80000000.0 => {
				res = set_frequency_microhertz_inner(&mut port, chan, amount * 100.0, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency uHz\" argument (must be 0.01-80000000.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency uHz\" argument (must be 0.01-80000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_frequency_microhertz_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_FREQUENCY_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_FREQUENCY_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if amount < 1.0 || amount > 8000000000.0 {
		return Err(Error::with_description(
			"Unsupported amount of uHz. Must be 0.01-80000000.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		chan_out,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_ARG_SEPARATOR,
		SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting frequency in uHz: ch{}={}:\n{}",
			chan, amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_FREQUENCY_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's output frequency for a particular channel,
in millihertz (mHz).  
  
"amount" parameter:
```ignore
"0.01" - "80000000.0"
```
*/
pub fn set_frequency_millihertz(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-80000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.01 && amount <= 80000000.0 => {
				res = set_frequency_millihertz_inner(&mut port, chan, amount * 100.0, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-80000000.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-80000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_frequency_millihertz_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_FREQUENCY_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_FREQUENCY_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if amount < 1.0 || amount > 8000000000.0 {
		return Err(Error::with_description(
			"Unsupported amount of mHz. Must be 0.01-80000000.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		chan_out,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_ARG_SEPARATOR,
		SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting frequency in mHz: ch{}={}:\n{}",
			chan, amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_FREQUENCY_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's output frequency for a particular channel,
in hertz (Hz).  
  
"amount" parameter:
```ignore
"0.01" - "60000000.0"
```
*/
pub fn set_frequency_hertz(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set frequency Hz\" argument (must be 0.01-60000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.01 && amount <= 60000000.0 => {
				res = set_frequency_hertz_inner(&mut port, chan, amount * 100.0, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-60000000.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-60000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_frequency_hertz_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_FREQUENCY_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_FREQUENCY_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if amount < 1.0 || amount > 6000000000.0 {
		return Err(Error::with_description(
			"Unsupported amount of Hz. Must be 0.01-60000000.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		chan_out,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_ARG_SEPARATOR,
		SET_FREQUENCY_COMMAND_UNIT_HERTZ,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting frequency in Hz: ch{}={}:\n{}",
			chan, amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_FREQUENCY_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's output frequency for a particular channel,
in kilohertz (kHz).  
  
"amount" parameter:
```ignore
"0.00001" - "60000.0"
```
*/
pub fn set_frequency_kilohertz(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 5 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set frequency kHz\" argument (must be 0.00001-60000.0): {}: too many decimal places (5 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.00001 && amount <= 60000.0 => {
				let amount_rounded = ((amount * 100000.0 * 100000.0).round() / 100000.0).round();

				res = set_frequency_kilohertz_inner(&mut port, chan, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency kHz\" argument (must be 0.00001-60000.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency kHz\" argument (must be 0.00001-60000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_frequency_kilohertz_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_FREQUENCY_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_FREQUENCY_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if amount < 1.0 || amount > 6000000000.0 {
		return Err(Error::with_description(
			"Unsupported amount of kHz. Must be 0.00001-60000.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		chan_out,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_ARG_SEPARATOR,
		SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting frequency in kHz: ch{}={}:\n{}",
			chan, amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_FREQUENCY_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's output frequency for a particular channel,
in megahertz (MHz).  
  
"amount" parameter:
```ignore
"0.00000001" - "60.0"
```
*/
pub fn set_frequency_megahertz(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 8 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set frequency MHz\" argument (must be 0.00000001-60.0): {}: too many decimal places (8 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.00000001 && amount <= 60.0 => {
				let amount_rounded =
					((amount * 100000000.0 * 10000000.0).round() / 10000000.0).round();

				res = set_frequency_megahertz_inner(&mut port, chan, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency MHz\" argument (must be 0.00000001-60.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency MHz\" argument (must be 0.00000001-60.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_frequency_megahertz_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_FREQUENCY_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_FREQUENCY_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if amount < 1.0 || amount > 6000000000.0 {
		return Err(Error::with_description(
			"Unsupported amount of MHz. Must be 0.00000001-60.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		chan_out,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_ARG_SEPARATOR,
		SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting frequency in MHz: ch{}={}:\n{}",
			chan, amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_FREQUENCY_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Get the device's output frequency for a particular channel,
in hertz (Hz).  
  
Return Value (Ok Result):
```ignore
10,000 Hz:
"10000"
```
*/
pub fn get_frequency_hertz(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = GET_FREQUENCY_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = GET_FREQUENCY_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN, COMMAND_GET, chan_out, COMMAND_SEPARATOR, GET_FREQUENCY_ARG, COMMAND_END,
	);

	if verbose > 0 {
		println!("\nGetting frequency in Hz: ch{}:\n{}", chan, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_FREQUENCY_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3 = res3_parts[0];

	let res4_parts: Vec<&str> = res3.split(",").collect();

	if res4_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing comma (,): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res4_str = res4_parts[0];

	let res4 = res4_str.parse::<f64>().unwrap() / 100.0;

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res4);
	}

	Ok(res4.to_string())
}

/** Set the device's output signal amplitude in volts, for a 
particular channel.  
  
"amount" parameter:
```ignore
"0.000" - "20.0"
```
*/
pub fn set_amplitude(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 3 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set amplitude volts\" argument (must be 0.000-20.0): {}: too many decimal places (3 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.0 && amount <= 20.0 => {
				let amount_rounded = ((amount * 1000.0 * 1000.0).round() / 1000.0).round();

				res = set_amplitude_inner(&mut port, chan, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set amplitude volts\" argument (must be 0.000-20.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set amplitude volts\" argument (must be 0.000-20.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_amplitude_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_AMPLITUDE_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_AMPLITUDE_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if amount < 0.0 || amount > 20000.0 {
		return Err(Error::with_description(
			"Unsupported amount of volts. Must be 0.000-20.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN, COMMAND_SET, chan_out, COMMAND_SEPARATOR, amount, COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting amplitude in volts: ch{}={}:\n{}",
			chan, amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_AMPLITUDE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Get the device's output signal amplitude in volts for 
a particular channel.  
  
Return Value (Ok Result):
```ignore
5 volts:
"5"
```
*/
pub fn get_amplitude(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = GET_AMPLITUDE_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = GET_AMPLITUDE_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN, COMMAND_GET, chan_out, COMMAND_SEPARATOR, GET_AMPLITUDE_ARG, COMMAND_END,
	);

	if verbose > 0 {
		println!("\nGetting amplitude in volts: ch{}:\n{}", chan, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_AMPLITUDE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3_str = res3_parts[0];

	let res3 = res3_str.parse::<f64>().unwrap() / 1000.0;

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res3.to_string())
}

/** Set the device's duty cycle in percent, for a 
particular channel.  
  
"amount" parameter:
```ignore
"0.0" - "99.9"
```
*/
pub fn set_duty_cycle(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set duty cycle\" argument (must be 0.0-99.9): {}: too many decimal places (1 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.0 && amount <= 99.9 => {
				let amount_rounded = ((amount * 10.0 * 10.0).round() / 10.0).round();

				res = set_duty_cycle_inner(&mut port, chan, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set duty cycle\" argument (must be 0.0-99.9): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set duty cycle\" argument (must be 0.0-99.9): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_duty_cycle_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_DUTY_CYCLE_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_DUTY_CYCLE_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if amount < 0.0 || amount > 999.0 {
		return Err(Error::with_description(
			"Unsupported duty cycle. Must be 0.0-99.9.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN, COMMAND_SET, chan_out, COMMAND_SEPARATOR, amount, COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting duty cycle percent: ch{}={}:\n{}",
			chan, amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_DUTY_CYCLE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Get the device's duty cycle in percent, for a 
particular channel.  
  
Return Value (Ok Result):
```ignore
1%:
"1"
```
*/
pub fn get_duty_cycle(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = GET_DUTY_CYCLE_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = GET_DUTY_CYCLE_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN, COMMAND_GET, chan_out, COMMAND_SEPARATOR, GET_DUTY_CYCLE_ARG, COMMAND_END,
	);

	if verbose > 0 {
		println!("\nGetting duty cycle percent: ch{}:\n{}", chan, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_DUTY_CYCLE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3_str = res3_parts[0];

	let res3 = res3_str.parse::<f64>().unwrap() / 10.0;

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Set the device's voltage offset in volts, for a 
particular channel.  
  
"amount" parameter:
```ignore
"-9.99" - "9.99"
```
*/
pub fn set_voltage_offset(
	mut port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set voltage offset\" argument (must be -9.99-9.99): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= -9.99 && amount <= 9.99 => {
				let amount_rounded = (((1000.0 + amount * 100.0) * 100.0).round() / 100.0).round();

				res = set_voltage_offset_inner(&mut port, chan, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set voltage offset\" argument (must be -9.99-9.99): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set voltage offset\" argument (must be -9.99-9.99): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_voltage_offset_inner(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = SET_VOLTAGE_OFFSET_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = SET_VOLTAGE_OFFSET_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	if amount < 1.0 || amount > 1999.0 {
		return Err(Error::with_description(
			"Unsupported voltage offset. Must be -9.99-9.99.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN, COMMAND_SET, chan_out, COMMAND_SEPARATOR, amount, COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting voltage offset: ch{}={}:\n{}",
			chan, amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_VOLTAGE_OFFSET_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Get the device's voltage offset in volts, for a 
particular channel.  
  
Return Value (Ok Result):
```ignore
5 volts:
"5"
```
*/
pub fn get_voltage_offset(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;
	let chan_out: &str;

	if chan == 1 {
		chan_out = GET_VOLTAGE_OFFSET_COMMAND_CH1;
	} else if chan == 2 {
		chan_out = GET_VOLTAGE_OFFSET_COMMAND_CH2;
	} else {
		return Err(Error::with_description(
			"Unsupported channel number. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_GET,
		chan_out,
		COMMAND_SEPARATOR,
		GET_VOLTAGE_OFFSET_ARG,
		COMMAND_END,
	);

	if verbose > 0 {
		println!("\nGetting voltage offset: ch{}:\n{}", chan, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_VOLTAGE_OFFSET_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3_str = res3_parts[0];

	let res3 = (res3_str.parse::<f64>().unwrap() - 1000.0) / 100.0;

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Set the device's phase in degrees (°).  
  
"amount" parameter:
```ignore
"0.0" - "360.0"
```
*/
pub fn set_phase(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set phase\" argument (must be 0.0-360.0): {}: too many decimal places (1 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.0 && amount <= 360.0 => {
				let amount_rounded = ((amount * 10.0 * 10.0).round() / 10.0).round();

				res = set_phase_inner(&mut port, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set phase\" argument (must be 0.0-360.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set phase\" argument (must be 0.0-360.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_phase_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < 0.0 || amount > 3600.0 {
		return Err(Error::with_description(
			"Unsupported phase. Must be 0.0-360.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN, COMMAND_SET, SET_PHASE_COMMAND, COMMAND_SEPARATOR, amount, COMMAND_END,
	);

	if verbose > 0 {
		println!("\nSetting phase: {}:\n{}", amount, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_PHASE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Get the device's phase in degrees (°).  
  
Return Value (Ok Result):
```ignore
180 degrees:
"180"
```
*/
pub fn get_phase(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
	let command: String;

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_GET,
		GET_PHASE_COMMAND,
		COMMAND_SEPARATOR,
		GET_PHASE_ARG,
		COMMAND_END,
	);

	if verbose > 0 {
		println!("\nGetting phase:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_PHASE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3_str = res3_parts[0];

	let res3 = res3_str.parse::<f64>().unwrap() / 10.0;

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Set the device's tracking mode.  
  
"track" parameter:
```ignore
The value must be either a set of comma-separated setting names 
(see below), or a set of zeros and ones in the range of "0" - "11111", 
each bit corresponding to a feature you want to toggle tracking on/off 
for ("1" being on and "0" being off). For example: track frequency and 
amplitude: "101"

The bit position meanings are as follows:
0: "frequency" | "freq" | "fq" | "fr" | "f"
1: "waveform" | "wave" | "wav" | "wv" | "w"
2: "amplitude" | "ampli" | "amp" | "am" | "a"
3: "dutycycle" | "duty" | "dc" | "du" | "d"
4: "offset" | "off" | "os" | "ot" | "o"

turn off tracking: "0" | "none" | "null" | "non" | "nil" | "no" | "n"

You can also use any of the names above separated by commas to turn on
the tracking by feature name.
Ex:
frequency and amplitude sync: "freq,amp"

Or you can use the single character versions with no commas in between.
Ex:
frequency and amplitude sync: "fa"

Turn tracking off like this: "n"

Note that a value of zero (or no value) in the bit position will turn off 
tracking for the corresponding feature, so to turn tracking off for all 
features, you can do: "0"

You can also separate the values with commas if you prefer: "1,0,1"
```
*/
pub fn set_tracking(
	mut port: &mut Box<dyn SerialPort>,
	track: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let max_len = 5;

	let track_stripped = track.replace(',', "");

	let res: Result<String, clap::Error>;

	let mut track_bits = TrackingArg::from_bits(0).unwrap();

	for (i, c) in track_stripped.chars().enumerate() {
		match c.to_digit(10) {
			Some(c_num) => {
				if track_stripped.len() > max_len {
					return Err(Error::with_description(&format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}: too many digits (5 max)\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES), ErrorKind::InvalidValue));
				}

				if c_num > 1 {
					return Err(Error::with_description(&format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES), ErrorKind::InvalidValue));
				}

				if c_num == 1 {
					track_bits = TrackingArg::from_bits(track_bits.bits() | (1 << i)).unwrap();
				}
			}

			None => {
				let track_vec: Vec<&str> = track.split(",").collect();

				if track_vec.len() > max_len {
					return Err(Error::with_description(&format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}: too many digits (5 max)\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES), ErrorKind::InvalidValue));
				}

				for (i2, word) in track_vec.into_iter().enumerate() {
					let mut res = TrackingArg::NONE;
					let mut res_bits = res.bits();

					let mut return_err = false;

					track_bits |= TrackingArg::from_bits(
						*TRACKING_ARG_MAP.get(word.trim()).unwrap_or_else(|| {
							for (i3, c) in word.chars().enumerate() {
								let key: String = c.to_string();
								let val = TRACKING_ARG_MAP.get(&key[..]);

								match val {
									Some(val) => {
										res |= TrackingArg::from_bits(*val)
											.unwrap_or(TrackingArg::NONE);
									}

									None => {
										println!("Error: feature name argument at position (word: {}, char: {}) not recognized.", i2, i3);

										res |= TrackingArg::NONE;
										return_err = true;

										return &res_bits;
									}
								}
							}

							res_bits = res.bits();
							&res_bits
						}),
					)
					.unwrap();

					if return_err {
						return Err(Error::with_description(&format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES), ErrorKind::InvalidValue));
					}
				}

				break;
			}
		}
	}

	match track_bits {
		track_bits if track_bits <= TrackingArg::all() => {
			res = set_tracking_inner(&mut port, track_bits, verbose);
		}

		_ => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set tracking\" argument (must be 0-{}): {}\n\n{}", TrackingArg::all().to_str_val(), track_bits, TRACKING_FEATURES), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_tracking_inner(
	port: &mut Box<dyn SerialPort>,
	track: TrackingArg,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if track > TrackingArg::all() {
		return Err(Error::with_description(
			&format!(
				"Unsupported tracking argument. Must be a number 0-{}.\n\n{}",
				TrackingArg::all().to_str_val(),
				TRACKING_FEATURES
			),
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN, COMMAND_SET, SET_TRACKING_COMMAND, COMMAND_SEPARATOR, track, COMMAND_END,
	);

	if verbose > 0 {
		println!("\nSetting tracking: {}:\n{}", track.to_names(), command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_TRACKING_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Switch the device's display panel to the main screen,
with either channel 1 or channel 2 showing at the top.

"chan" parameter:
```ignore
Channel 1 at the top:
1

Channel 2 at the top:
2
```
*/
pub fn switch_function_panel_main(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str;

	if chan < 1 || chan > 2 {
		return Err(Error::with_description(
			"Unsupported channel. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	} else if chan == 1 {
		command = SWITCH_FUNCTION_PANEL_MAIN_CH1;
	} else {
		// if chan == 2
		command = SWITCH_FUNCTION_PANEL_MAIN_CH2;
	}

	if verbose > 0 {
		println!(
			"\nSwitching to function panel main ch{} mode:\n{}",
			chan, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SWITCH_FUNCTION_PANEL_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Switch the device's display panel to the system (SYS)
screen. */
pub fn switch_function_panel_sys(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SWITCH_FUNCTION_PANEL_SYS;

	if verbose > 0 {
		println!(
			"\nSwitching function panel to system settings mode:\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SWITCH_FUNCTION_PANEL_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Switch the device's display panel to the measure 
subsection on the measure mode (MEAS) screen. */
pub fn switch_function_panel_measurement(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SWITCH_FUNCTION_PANEL_MEASUREMENT;

	if verbose > 0 {
		println!(
			"\nSwitching function panel to measurement mode:\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SWITCH_FUNCTION_PANEL_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Start taking a measurement from the device's input (Ext. IN) 
channel.  
  
Also, stop the following functions if any are currently in 
progress:  
```ignore
counting
sweep
frequency
pulse
burst
```
*/
pub fn start_measuring(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = START_MEASURING;

	if verbose > 0 {
		println!(
			"\nMeasurement starting - counting, sweep, frequency, pulse, burst stopping:\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_EXTENDED_FUNCTION_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Switch the device's display panel to the counting
subsection on the measure mode (MEAS) screen. */
pub fn switch_function_panel_counting(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SWITCH_FUNCTION_PANEL_COUNTING;

	if verbose > 0 {
		println!("\nSwitching function panel to counting mode:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SWITCH_FUNCTION_PANEL_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Start counting, using the device's input (Ext. IN) channel. */
pub fn start_counting(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = START_COUNTING;

	if verbose > 0 {
		println!("\nCounting starting:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_EXTENDED_FUNCTION_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Switch the device's display panel to the sweep frequency
(channel 1 or channel 2) subsection of the modulation mode 
screen.  
  
"chan" parameter:
```ignore
Sweep Frequency (CH1):
1

Sweep Frequency (CH2):
2
```
*/
pub fn switch_function_panel_sweep(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str;

	if chan < 1 || chan > 2 {
		return Err(Error::with_description(
			"Unsupported channel. Must be 1 or 2.",
			ErrorKind::InvalidValue,
		));
	} else if chan == 1 {
		command = SWITCH_FUNCTION_PANEL_SWEEP_CH1;
	} else {
		// if chan == 2
		command = SWITCH_FUNCTION_PANEL_SWEEP_CH2;
	}

	if verbose > 0 {
		println!(
			"\nSwitching to function panel sweep ch{} mode:\n{}",
			chan, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SWITCH_FUNCTION_PANEL_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Start frequency sweeping on a given output channel.  
  
"chan" parameter:
```ignore
Sweep Frequency (CH1):
1

Sweep Frequency (CH2):
2
```
*/
pub fn start_sweeping(
	port: &mut Box<dyn SerialPort>,
	chan: u64,
	verbose: u64,
) -> Result<String, clap::Error> {
	switch_function_panel_sweep(port, chan, verbose)?;

	let command: &'static str = START_SWEEPING;

	if verbose > 0 {
		println!("\nSweep starting:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_EXTENDED_FUNCTION_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Switch the device's display panel to the pulse generator
subsection of the modulation mode screen. */
pub fn switch_function_panel_pulse(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SWITCH_FUNCTION_PANEL_PULSE;

	if verbose > 0 {
		println!("\nSwitching function panel to pulse mode:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SWITCH_FUNCTION_PANEL_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Start the device's pulsing output on channel 1. */
pub fn start_pulsing(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = START_PULSING;

	if verbose > 0 {
		println!("\nPulse starting:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_EXTENDED_FUNCTION_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Switch the device's display panel to the burst subsection
of the modulation mode screen. */
pub fn switch_function_panel_bursting(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SWITCH_FUNCTION_PANEL_BURST;

	if verbose > 0 {
		println!("\nSwitching function panel to bursting mode:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SWITCH_FUNCTION_PANEL_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Start the device's bursting output on channel 1. */
pub fn start_bursting(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = START_BURSTING;

	if verbose > 0 {
		println!("\nBursting starting:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_EXTENDED_FUNCTION_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's measurement mode measure coupling to 
AC (Ext. IN).
*/
pub fn set_measurement_coupling_ac(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SET_MEASUREMENT_COUPLING_AC;

	if verbose > 0 {
		println!("\nSetting measurement coupling to AC:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_MEASUREMENT_COUPLING_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's measurement mode measure coupling to 
DC (Ext. IN).
*/
pub fn set_measurement_coupling_dc(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SET_MEASUREMENT_COUPLING_DC;

	if verbose > 0 {
		println!("\nSetting measurement coupling to DC:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_MEASUREMENT_COUPLING_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's measurement mode measure gate time 
in seconds.  
  
"amount" parameter:
```ignore
"0.01" - "10.0"
```
*/
pub fn set_measurement_gate_time(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set measurement gate time\" argument (must be 0.01-10.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.01 && amount <= 10.0 => {
				let amount_rounded = ((amount * 100.0 * 100.0).round() / 100.0).round();

				res = set_measurement_gate_time_inner(&mut port, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set measurement gate time\" argument (must be 0.01-10.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set measurement gate time\" argument (must be 0.01-10.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_measurement_gate_time_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < 1.0 || amount > 1000.0 {
		return Err(Error::with_description(
			"Unsupported measurement gate time. Must be 0.01-10.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		SET_MEASUREMENT_GATE_TIME_COMMAND,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_END,
	);

	if verbose > 0 {
		println!("\nSetting measurement gate time: {}:\n{}", amount, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_MEASUREMENT_GATE_TIME_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's measurement mode to measure count 
frequency in hertz (Hz). */
pub fn set_measurement_mode_count_frequency(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SET_MEASUREMENT_MODE_COUNT_FREQUENCY;

	if verbose > 0 {
		println!(
			"\nSetting measurement mode to count frequency:\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_MEASUREMENT_MODE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the device's measurement mode to measure counting 
period in hertz (Hz). */
pub fn set_measurement_mode_counting_period(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SET_MEASUREMENT_MODE_COUNTING_PERIOD;

	if verbose > 0 {
		println!(
			"\nSetting measurement mode to counting period:\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_MEASUREMENT_MODE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Clear the measure mode counter's count number (Cnt. Num). */
pub fn set_measurement_count_clear(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = SET_MEASUREMENT_COUNT_CLEAR;

	if verbose > 0 {
		println!("\nSetting measurement count clear:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_MEASUREMENT_COUNT_CLEAR_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Get the measure mode counter's count number (Cnt. Num). */
pub fn get_measurement_count(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = GET_MEASUREMENT_COUNT;

	if verbose > 0 {
		println!(
			"\nGetting measurement count:\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_MEASUREMENT_COUNT_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3 = res3_parts[0];

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Get the measure mode's measure frequency in frequency mode 
(hertz). */
pub fn get_measurement_frequency(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = GET_MEASUREMENT_FREQUENCY;

	if verbose > 0 {
		println!(
			"\nGetting measurement frequency value in frequency mode (hertz):\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_MEASUREMENT_FREQUENCY_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3 = res3_parts[0];

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Get the measure mode's measure frequency in period mode 
(hertz). */
pub fn get_measurement_frequency_period(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = GET_MEASUREMENT_FREQUENCY_PERIOD;

	if verbose > 0 {
		println!(
			"\nGetting measurement frequency value in period mode (in hertz):\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_MEASUREMENT_FREQUENCY_PERIOD_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3_str = res3_parts[0];
	
	// POSSIBLE BUG: Not sure if this is the correct denominator.
	let res3 = res3_str.parse::<f64>().unwrap() / 1000.0;
	
	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Get the measure mode's measure pulse width (positive). */
pub fn get_measurement_pulse_width_positive(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = GET_MEASUREMENT_PULSE_WIDTH_POSITIVE;

	if verbose > 0 {
		println!(
			"\nGetting measurement pulse width (positive):\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3_str = res3_parts[0];
	
	// NOTE: Should we convert the response unit?
	let res3 = res3_str.parse::<f64>().unwrap();
	
	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Get the measure mode's measure pulse width (negative). */
pub fn get_measurement_pulse_width_negative(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE;

	if verbose > 0 {
		println!(
			"\nGetting measurement pulse width (negative):\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3_str = res3_parts[0];
	
	// NOTE: Should we convert the response unit?
	let res3 = res3_str.parse::<f64>().unwrap();
	
	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Get the measure mode's measure period. */
pub fn get_measurement_period(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = GET_MEASUREMENT_PERIOD;

	if verbose > 0 {
		println!(
			"\nGetting measurement period:\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_MEASUREMENT_PERIOD_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3_str = res3_parts[0];
	
	// NOTE: Should we convert the response unit?
	let res3 = res3_str.parse::<f64>().unwrap();
	
	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Get the measure mode's measure duty cycle. */
pub fn get_measurement_duty_cycle(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: &'static str = GET_MEASUREMENT_DUTY_CYCLE;

	if verbose > 0 {
		println!(
			"\nGetting measurement duty cycle:\n{}",
			command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_MEASUREMENT_DUTY_CYCLE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	let res2_parts: Vec<&str> = res.split("=").collect();

	if res2_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing equals (=): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res2 = res2_parts[1];

	let res3_parts: Vec<&str> = res2.split(".").collect();

	if res3_parts.len() < 2 {
		return Err(Error::with_description(
			&format!(
				"unexpected response from device: missing period (.): {}",
				res
			),
			ErrorKind::Io,
		));
	}

	let res3_str = res3_parts[0];
	
	// NOTE: Should we convert the response unit?
	let res3 = res3_str.parse::<f64>().unwrap();
	
	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	} else {
		println!("{}", res3);
	}

	Ok(res.to_string())
}

/** Set the number of burst pulses to perform when bursting.  
  
"amount" parameter:
```ignore
5 burst pulses:
"5"
```
*/
pub fn set_burst_pulse_number(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1] != "0" {
		return Err(Error::with_description(&format!("unsupported value passed to \"set burst pulse number\" argument (must be 1-1048575): {}: too many decimal places (0 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 1.0 && amount <= 1048575.0 => {
				res = set_burst_pulse_number_inner(&mut port, amount, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set burst pulse number\" argument (must be 1-1048575): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set burst pulse number\" argument (must be 1-1048575): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_burst_pulse_number_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < 1.0 || amount > 1048575.0 {
		return Err(Error::with_description(
			"Unsupported burst pulse number. Must be 1-1048575.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		SET_BURST_PULSE_NUMBER_COMMAND,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_END,
	);

	if verbose > 0 {
		println!("\nSetting burst pulse number: {}:\n{}", amount, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_BURST_PULSE_NUMBER_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Perform a single burst pulse immediately. */
pub fn start_burst_pulse_once(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = START_BURST_PULSE_ONCE;

	if verbose > 0 {
		println!("\nBurst pulse once:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..START_BURST_PULSE_ONCE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode's Burst (CH1) mode to manual trigger
(Manual Trig.).
*/
pub fn set_burst_mode_manual_trigger(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = SET_BURST_MODE_MANUAL_TRIGGER;

	if verbose > 0 {
		println!("\nSetting burst mode to manual trigger:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_BURST_MODE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode's Burst (CH1) mode to channel 2 trigger
(CH2 Trig.).
*/
pub fn set_burst_mode_ch2_burst(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = SET_BURST_MODE_CH2_BURST;

	if verbose > 0 {
		println!("\nSetting burst mode to CH2 burst:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_BURST_MODE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode's Burst (CH1) mode to external 
trigger AC (Ext. Trig (AC)).
*/
pub fn set_burst_mode_external_burst_ac(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = SET_BURST_MODE_EXTERNAL_BURST_AC;

	if verbose > 0 {
		println!("\nSetting burst mode to external burst AC:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_BURST_MODE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode's Burst (CH1) mode to external 
trigger DC (Ext. Trig (DC)).
*/
pub fn set_burst_mode_external_burst_dc(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = SET_BURST_MODE_EXTERNAL_BURST_DC;

	if verbose > 0 {
		println!("\nSetting burst mode to external burst DC:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_BURST_MODE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode sweep frequency's starting
frequency in hertz (Hz).  
  
"amount" parameter:
```ignore
1,000 hertz:
"1000"
```
*/
pub fn set_sweep_starting_frequency(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set sweep starting frequency\" argument (must be 0.01-60000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.01 && amount <= 60000000.0 => {
				let amount_rounded = ((amount * 100.0 * 10.0).round() / 10.0).round();

				res = set_sweep_starting_frequency_inner(&mut port, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep starting frequency\" argument (must be 0.01-60000000.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep starting frequency\" argument (must be 0.01-60000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_sweep_starting_frequency_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < 1.0 || amount > 6000000000.0 {
		return Err(Error::with_description(
			"Unsupported sweep starting frequency. Must be 0.01-60000000.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		SET_SWEEP_STARTING_FREQUENCY_COMMAND,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting sweep starting frequency: {}:\n{}",
			amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_SWEEP_STARTING_FREQUENCY_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode sweep frequency's end
frequency in hertz (Hz).  
  
"amount" parameter:
```ignore
2,000 hertz:
"2000"
```
*/
pub fn set_sweep_end_frequency(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set sweep termination frequency\" argument (must be 0.01-60000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.01 && amount <= 60000000.0 => {
				let amount_rounded = ((amount * 100.0 * 10.0).round() / 10.0).round();

				res = set_sweep_end_frequency_inner(&mut port, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep termination frequency\" argument (must be 0.01-60000000.0): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep termination frequency\" argument (must be 0.01-60000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_sweep_end_frequency_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < 1.0 || amount > 6000000000.0 {
		return Err(Error::with_description(
			"Unsupported sweep termination frequency. Must be 0.01-60000000.0.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		SET_SWEEP_END_FREQUENCY_COMMAND,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nSetting sweep termination frequency: {}:\n{}",
			amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_SWEEP_END_FREQUENCY_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode sweep frequency's sweep time
in seconds.  
  
"amount" parameter:
```ignore
10 seconds:
"10"
```
*/
pub fn set_sweep_time(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set sweep time\" argument (must be 0.1-999.9): {}: too many decimal places (1 max)", amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= 0.1 && amount <= 999.9 => {
				let amount_rounded = (amount * 10.0).round();

				res = set_sweep_time_inner(&mut port, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep time\" argument (must be 0.1-999.9): {}", amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep time\" argument (must be 0.1-999.9): {}: {}", amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_sweep_time_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < 1.0 || amount > 6000000000.0 {
		return Err(Error::with_description(
			"Unsupported sweep time. Must be 0.1-999.9.",
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		SET_SWEEP_TIME_COMMAND,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_END,
	);

	if verbose > 0 {
		println!("\nSetting sweep time: {}:\n{}", amount, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_SWEEP_TIME_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode sweep frequency's sweep direction to normal:
`Rise`
*/
pub fn set_sweep_direction_rise(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = SET_SWEEP_DIRECTION_RISE;

	if verbose > 0 {
		println!("\nSetting sweep direction to rise:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_SWEEP_DIRECTION_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode sweep frequency's sweep direction to reverse:
`Fall`
*/
pub fn set_sweep_direction_fall(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = SET_SWEEP_DIRECTION_FALL;

	if verbose > 0 {
		println!("\nSetting sweep direction to fall:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_SWEEP_DIRECTION_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode sweep frequency's sweep direction to round-trip:
`Rise & Fall`
*/
pub fn set_sweep_direction_rise_fall(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = SET_SWEEP_DIRECTION_RISE_FALL;

	if verbose > 0 {
		println!("\nSetting sweep direction to rise and fall:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_SWEEP_DIRECTION_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode sweep frequency's mode to linear. */
pub fn set_sweep_mode_linear(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = SET_SWEEP_MODE_LINEAR;

	if verbose > 0 {
		println!("\nSetting sweep mode to linear:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_SWEEP_MODE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode sweep frequency's mode to logarithm. */
pub fn set_sweep_mode_logarithm(
	port: &mut Box<dyn SerialPort>,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command = SET_SWEEP_MODE_LOGARITHM;

	if verbose > 0 {
		println!("\nSetting sweep mode to logarithm:\n{}", command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_SWEEP_MODE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode pulse generator's pulse width.  
  
It is in nanosecond units unless, the microseconds parameter is true.  
  
IMPORTANT NOTE: There seems to be no option on the device's physical
controls to switch between nanosecond and microsecond units, but if
you specify a value in microseconds, the device will switch to
microsecond mode, and all future values to this command entered through
the physical device interface will be interpreted as microsecond units
until you turn off the device, or set a nanosecond value using this
serial-interface-only command. If you save the device state while in
microseconds mode, this could be a problem, because then you need to
use this serial program to get back to the default nanoseconds mode.  
  
"amount" parameter when "microseconds" parameter is false:
```ignore
1,000 nanoseconds:
"1000"
```
  
"amount" parameter when "microseconds" parameter is true:
```ignore
2,000 microseconds:
"2000"
```
*/
pub fn set_pulse_width(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	microseconds: bool,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();
	let units: &'static str;
	let arg_min: f64;
	let arg_max: f64;

	if microseconds {
		units = "us";
		arg_min = SET_PULSE_WIDTH_ARG_MICROSECONDS_MIN;
		arg_max = SET_PULSE_WIDTH_ARG_MICROSECONDS_MAX;
	} else {
		units = "ns";
		arg_min = SET_PULSE_WIDTH_ARG_NANOSECONDS_MIN;
		arg_max = SET_PULSE_WIDTH_ARG_NANOSECONDS_MAX;
	}

	if amount_parts.len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set pulse width ({})\" argument (must be {}-{}): {}: too many decimal places (0 max)", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= arg_min && amount <= arg_max => {
				if units == "ns" && amount as i64 % 5 != 0 {
					return Err(Error::with_description(&format!("unsupported value passed to \"set pulse width ({})\" argument (must be {}-{}): {}: if nanoseconds, it must be a multiple of 5", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
				}

				res = set_pulse_width_inner(&mut port, amount, microseconds, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse width ({})\" argument (must be {}-{}): {}", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse width ({})\" argument (must be {}-{}): {}: {}", units, arg_min, arg_max, amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_pulse_width_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	microseconds: bool,
	verbose: u64,
) -> Result<String, clap::Error> {
	let units: &'static str;
	let arg_min: f64;
	let arg_max: f64;
	let command: String;

	if microseconds {
		units = "us";
		arg_min = SET_PULSE_WIDTH_ARG_MICROSECONDS_MIN;
		arg_max = SET_PULSE_WIDTH_ARG_MICROSECONDS_MAX;

		command = format!(
			"{}{}{}{}{}{}{}{}",
			COMMAND_BEGIN,
			COMMAND_SET,
			SET_PULSE_WIDTH_COMMAND,
			COMMAND_SEPARATOR,
			amount,
			COMMAND_ARG_SEPARATOR,
			SET_PULSE_WIDTH_ARG_MICROSECONDS,
			COMMAND_END,
		);
	} else {
		units = "ns";
		arg_min = SET_PULSE_WIDTH_ARG_NANOSECONDS_MIN;
		arg_max = SET_PULSE_WIDTH_ARG_NANOSECONDS_MAX;

		command = format!(
			"{}{}{}{}{}{}{}{}",
			COMMAND_BEGIN,
			COMMAND_SET,
			SET_PULSE_WIDTH_COMMAND,
			COMMAND_SEPARATOR,
			amount,
			COMMAND_ARG_SEPARATOR,
			SET_PULSE_WIDTH_ARG_NANOSECONDS,
			COMMAND_END,
		);
	}

	if amount < arg_min || amount > arg_max {
		return Err(Error::with_description(
			&format!(
				"Unsupported pulse width ({}). Must be {}-{}.",
				units, arg_min, arg_max
			),
			ErrorKind::InvalidValue,
		));
	}

	if verbose > 0 {
		println!("\nSetting pulse width: {} {}:\n{}", amount, units, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_PULSE_WIDTH_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode pulse generator's pulse period.  
  
It is in nanosecond units unless the microseconds parameter is true.  
  
IMPORTANT NOTE: There seems to be no option on the device's physical
controls to switch between nanosecond and microsecond units, but if
you specify a value in microseconds, the device will switch to
microsecond mode, and all future values to this command entered through
the physical device interface will be interpreted as microsecond units
until you turn off the device, or set a nanosecond value using this
serial-interface-only command. If you save the device state while in
microseconds mode, this could be a problem, because then you need to
use this serial program to get back to the default nanoseconds mode.  
  
"amount" parameter when "microseconds" parameter is false:
```ignore
1,000 nanoseconds:
"1000"
```
  
"amount" parameter when "microseconds" parameter is true:
```ignore
2,000 microseconds:
"2000"
```
*/
pub fn set_pulse_period(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	microseconds: bool,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();
	let units: &'static str;
	let arg_min: f64;
	let arg_max: f64;

	if microseconds {
		units = "us";
		arg_min = SET_PULSE_PERIOD_ARG_MICROSECONDS_MIN;
		arg_max = SET_PULSE_PERIOD_ARG_MICROSECONDS_MAX;
	} else {
		units = "ns";
		arg_min = SET_PULSE_PERIOD_ARG_NANOSECONDS_MIN;
		arg_max = SET_PULSE_PERIOD_ARG_NANOSECONDS_MAX;
	}

	if amount_parts.len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set pulse period ({})\" argument (must be {}-{}): {}: too many decimal places (0 max)", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= arg_min && amount <= arg_max => {
				if units == "ns" && amount as i64 % 5 != 0 {
					return Err(Error::with_description(&format!("unsupported value passed to \"set pulse period ({})\" argument (must be {}-{}): {}: if nanoseconds, it must be a multiple of 5", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
				}

				res = set_pulse_period_inner(&mut port, amount, microseconds, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse period ({})\" argument (must be {}-{}): {}", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse period ({})\" argument (must be {}-{}): {}: {}", units, arg_min, arg_max, amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_pulse_period_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	microseconds: bool,
	verbose: u64,
) -> Result<String, clap::Error> {
	let units: &'static str;
	let arg_min: f64;
	let arg_max: f64;
	let command: String;

	if microseconds {
		units = "us";
		arg_min = SET_PULSE_PERIOD_ARG_MICROSECONDS_MIN;
		arg_max = SET_PULSE_PERIOD_ARG_MICROSECONDS_MAX;

		command = format!(
			"{}{}{}{}{}{}{}{}",
			COMMAND_BEGIN,
			COMMAND_SET,
			SET_PULSE_PERIOD_COMMAND,
			COMMAND_SEPARATOR,
			amount,
			COMMAND_ARG_SEPARATOR,
			SET_PULSE_PERIOD_ARG_MICROSECONDS,
			COMMAND_END,
		);
	} else {
		units = "ns";
		arg_min = SET_PULSE_PERIOD_ARG_NANOSECONDS_MIN;
		arg_max = SET_PULSE_PERIOD_ARG_NANOSECONDS_MAX;

		command = format!(
			"{}{}{}{}{}{}{}{}",
			COMMAND_BEGIN,
			COMMAND_SET,
			SET_PULSE_PERIOD_COMMAND,
			COMMAND_SEPARATOR,
			amount,
			COMMAND_ARG_SEPARATOR,
			SET_PULSE_PERIOD_ARG_NANOSECONDS,
			COMMAND_END,
		);
	}

	if amount < arg_min || amount > arg_max {
		return Err(Error::with_description(
			&format!(
				"Unsupported pulse period ({}). Must be {}-{}.",
				units, arg_min, arg_max
			),
			ErrorKind::InvalidValue,
		));
	}

	if verbose > 0 {
		println!("\nSetting pulse period: {} {}:\n{}", amount, units, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_PULSE_PERIOD_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode pulse generator's pulse offset in percent (%).  
  
"amount" parameter:
```ignore
50 percent:
"50"
```
*/
pub fn set_pulse_offset(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set pulse offset\" argument (must be {}-{}): {}: too many decimal places (0 max)", SET_PULSE_OFFSET_ARG_PERCENT_MIN, SET_PULSE_OFFSET_ARG_PERCENT_MAX, amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= SET_PULSE_OFFSET_ARG_PERCENT_MIN
				&& amount <= SET_PULSE_OFFSET_ARG_PERCENT_MAX =>
			{
				res = set_pulse_offset_inner(&mut port, amount, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse offset\" argument (must be {}-{}): {}", SET_PULSE_OFFSET_ARG_PERCENT_MIN, SET_PULSE_OFFSET_ARG_PERCENT_MAX, amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse offset\" argument (must be {}-{}): {}: {}", SET_PULSE_OFFSET_ARG_PERCENT_MIN, SET_PULSE_OFFSET_ARG_PERCENT_MAX, amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_pulse_offset_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < SET_PULSE_OFFSET_ARG_PERCENT_MIN || amount > SET_PULSE_OFFSET_ARG_PERCENT_MAX {
		return Err(Error::with_description(
			&format!(
				"Unsupported pulse offset. Must be {}-{}.",
				SET_PULSE_OFFSET_ARG_PERCENT_MIN, SET_PULSE_OFFSET_ARG_PERCENT_MAX
			),
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		SET_PULSE_OFFSET_COMMAND,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_END,
	);

	if verbose > 0 {
		println!("\nSetting pulse offset: {}:\n{}", amount, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_PULSE_OFFSET_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Set the modulation mode pulse generator's pulse amplitude in volts (V).  
  
"amount" parameter:
```ignore
5 volts:
"5"
```
*/
pub fn set_pulse_amplitude(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
		return Err(Error::with_description(&format!("unsupported value passed to \"set pulse amplitude\" argument (must be {}-{}): {}: too many decimal places (2 max)", SET_PULSE_AMPLITUDE_ARG_VOLTS_MIN, SET_PULSE_AMPLITUDE_ARG_VOLTS_MAX, amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= SET_PULSE_AMPLITUDE_ARG_VOLTS_MIN
				&& amount <= SET_PULSE_AMPLITUDE_ARG_VOLTS_MAX =>
			{
				let amount_rounded = ((amount * 100.0 * 100.0).round() / 100.0).round();

				res = set_pulse_amplitude_inner(&mut port, amount_rounded, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse amplitude\" argument (must be {}-{}): {}", SET_PULSE_AMPLITUDE_ARG_VOLTS_MIN, SET_PULSE_AMPLITUDE_ARG_VOLTS_MAX, amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse amplitude\" argument (must be {}-{}): {}: {}", SET_PULSE_AMPLITUDE_ARG_VOLTS_MIN, SET_PULSE_AMPLITUDE_ARG_VOLTS_MAX, amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_pulse_amplitude_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < SET_PULSE_AMPLITUDE_ARG_VOLTS_MIN * 100.0
		|| amount > SET_PULSE_AMPLITUDE_ARG_VOLTS_MAX * 100.0
	{
		return Err(Error::with_description(
			&format!(
				"Unsupported pulse amplitude. Must be {}-{}.",
				SET_PULSE_AMPLITUDE_ARG_VOLTS_MIN, SET_PULSE_AMPLITUDE_ARG_VOLTS_MAX
			),
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		SET_PULSE_AMPLITUDE_COMMAND,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_END,
	);

	if verbose > 0 {
		println!("\nSetting pulse amplitude: {}:\n{}", amount, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_PULSE_AMPLITUDE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Save all current values on the device as a numbered preset.  
  
"amount" parameter:
```ignore
save as preset 0:
"0"
```
*/
pub fn save_preset(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"save preset\" argument (must be {}-{}): {}: too many decimal places (0 max)", SAVE_PRESET_ARG_NUM_MIN, SAVE_PRESET_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => {
			match amount {
				_y if amount >= SAVE_PRESET_ARG_NUM_MIN
					&& amount <= SAVE_PRESET_ARG_NUM_MAX =>
				{
					res = save_preset_inner(&mut port, amount, verbose);
				}

				_ => {
					res = Err(Error::with_description(&format!("unsupported value passed to \"save preset\" argument (must be {}-{}): {}", SAVE_PRESET_ARG_NUM_MIN, SAVE_PRESET_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
				}
			}
		}

		Err(e) => {
			res = Err(Error::with_description(
				&format!(
					"unsupported value passed to \"save preset\" argument (must be {}-{}): {}: {}",
					SAVE_PRESET_ARG_NUM_MIN, SAVE_PRESET_ARG_NUM_MAX, amount, e
				),
				ErrorKind::InvalidValue,
			));
		}
	}

	res
}

fn save_preset_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < SAVE_PRESET_ARG_NUM_MIN || amount > SAVE_PRESET_ARG_NUM_MAX {
		return Err(Error::with_description(
			&format!(
				"Unsupported preset number. Must be {}-{}.",
				SAVE_PRESET_ARG_NUM_MIN, SAVE_PRESET_ARG_NUM_MAX
			),
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		SAVE_PRESET_COMMAND,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_END,
	);

	if verbose > 0 {
		println!("\nSaving values as preset number: {}:\n{}", amount, command);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SAVE_PRESET_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Load all values for the device from a numbered preset.  
  
"amount" parameter:
```ignore
save as preset 0:
"0"
```
*/
pub fn load_preset(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"recall preset\" argument (must be {}-{}): {}: too many decimal places (0 max)", LOAD_PRESET_ARG_NUM_MIN, LOAD_PRESET_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= LOAD_PRESET_ARG_NUM_MIN
				&& amount <= LOAD_PRESET_ARG_NUM_MAX =>
			{
				res = load_preset_inner(&mut port, amount, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"recall preset\" argument (must be {}-{}): {}", LOAD_PRESET_ARG_NUM_MIN, LOAD_PRESET_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"recall preset\" argument (must be {}-{}): {}: {}", LOAD_PRESET_ARG_NUM_MIN, LOAD_PRESET_ARG_NUM_MAX, amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn load_preset_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < LOAD_PRESET_ARG_NUM_MIN || amount > LOAD_PRESET_ARG_NUM_MAX {
		return Err(Error::with_description(
			&format!(
				"Unsupported preset number. Must be {}-{}.",
				LOAD_PRESET_ARG_NUM_MIN, LOAD_PRESET_ARG_NUM_MAX
			),
			ErrorKind::InvalidValue,
		));
	}

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		COMMAND_SET,
		LOAD_PRESET_COMMAND,
		COMMAND_SEPARATOR,
		amount,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nRecalling values from preset number: {}:\n{}",
			amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..LOAD_PRESET_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Convert a WaveCAD (.wav) file to the device's arbitrary
waveform text (.txt) file format.  
  
"path" parameter:
```ignore
some-wav-file-to-convert.wav:
"some-wav-file-to-convert.wav"
```
*/
pub fn wav_to_txt(path: &str, verbose: u64) -> Result<String, clap::Error> {
	let mut res: Result<String, clap::Error>;

	if path == "" {
		res = Err(Error::with_description(
			&format!(
				"unsupported path passed as \"wav_to_txt\" argument (must not be blank): {}",
				path
			),
			ErrorKind::InvalidValue,
		));
		return res;
	}

	let new_path: &str = &change_file_extension(path, ".txt");

	match fs::File::open(path) {
		Ok(mut file) => {
			let mut buf = [0u8; 4096];
			res = file.read(&mut buf).map_or_else(
				|e| {
					Err(Error::with_description(
						&format!("failed reading file: {}: {}", path, e),
						ErrorKind::Io,
					))
				},
				|_res| Ok("".to_string()),
			);

			if res.is_err() {
				return res;
			}

			let mut out = String::new();

			let mut outbuf = [0i16; 2048];

			LittleEndian::read_i16_into(&buf, &mut outbuf);

			let clamp_min = 0;
			let clamp_max = 4095;

			let mut clamp_min_count = 0u64;
			let mut clamp_max_count = 0u64;

			let mut clamp_min_adjustment_total = 0i64;
			let mut clamp_max_adjustment_total = 0i64;

			for (i, val) in outbuf.iter().enumerate() {
				let mut new_val = *val as i64 + 2048;

				if new_val < clamp_min {
					clamp_min_count += 1;
					let clamp_min_adjustment = clamp_min - new_val;
					clamp_min_adjustment_total += clamp_min_adjustment;

					println!("warning: wave data below min supported value, clamping to {}:\tline: {}:\tvalue lost: {}:\tadjustment: +{}", clamp_min, i + 1, new_val, clamp_min_adjustment);

					new_val = clamp_min;
				} else if new_val > clamp_max {
					clamp_max_count += 1;
					let clamp_max_adjustment = clamp_max - new_val;
					clamp_max_adjustment_total += clamp_max_adjustment;

					println!("warning: wave data above max supported value, clamping to {}:\tline: {}:\tvalue lost: {}:\tadjustment: {}", clamp_max, i + 1, new_val, clamp_max_adjustment);

					new_val = clamp_max;
				}

				out += &new_val.to_string();

				if i < buf.len() - 1 {
					out += "\n";
				}
			}

			if clamp_min_count > 0 || clamp_max_count > 0 {
				println!("");
			}

			if clamp_min_count > 0 {
				println!("warning: wave data was min clamped to {}:\ttimes min clamped: {}:\tsummed min clamp adjustments: +{}", clamp_min, clamp_min_count, clamp_min_adjustment_total);
			}

			if clamp_max_count > 0 {
				println!("warning: wave data was max clamped to {}:\ttimes max clamped: {}:\tsummed max clamp adjustments: {}", clamp_max, clamp_max_count, clamp_max_adjustment_total);
			}

			if clamp_min_count > 0 || clamp_max_count > 0 {
				println!("warning: some of the wave data was clamped:\ttotal times clamped: {}:\ttotal summed clamp adjustments: {}\n", clamp_min_count + clamp_max_count, clamp_min_adjustment_total - clamp_max_adjustment_total);
			}

			match fs::File::create(&format!("{}", new_path)) {
				Ok(mut outfile) => {
					res = outfile.write_all(out.as_bytes()).map_or_else(
						|e| {
							Err(Error::with_description(
								&format!("failed writing to file: {}: {}", new_path, e),
								ErrorKind::Io,
							))
						},
						|_res| Ok(fs::read_to_string(&format!("{}", new_path)).unwrap()),
					);

					if res.is_err() {
						return res;
					}

					if verbose > 0 {
						println!(
							"\nWaveCAD file converted to text and saved: {} -> {}",
							path, new_path
						);
					}
				}

				Err(e) => {
					res = Err(Error::with_description(
						&format!("failed creating file: {}: {}", new_path, e),
						ErrorKind::Io,
					));
					return res;
				}
			}
		}

		Err(e) => {
			res = Err(Error::with_description(
				&format!("failed opening file: {}: {}", path, e),
				ErrorKind::Io,
			));
			return res;
		}
	}

	res
}

/** Convert the device's arbitrary waveform text (.txt) file format
to a WaveCAD (.wav) file.  
  
Outputs the resulting binary file to stdout if the "output_binary" 
parameter is set to true.  
  
"path" parameter:
```ignore
some-txt-file-to-convert.txt:
"some-txt-file-to-convert.txt"
```
*/
pub fn txt_to_wav(path: &str, output_binary: bool, verbose: u64) -> Result<String, clap::Error> {
	let mut res: Result<String, clap::Error>;

	if path == "" {
		return Err(Error::with_description(
			&format!(
				"unsupported path passed as \"txt_to_wav\" argument (must not be blank): {}",
				path
			),
			ErrorKind::InvalidValue,
		));
	}

	let new_path: &str = &change_file_extension(path, ".wav");

	match fs::File::open(path) {
		Ok(mut file) => {
			// Buffer to hold the input file contents.
			let mut stringbuf = String::new();
			res = file.read_to_string(&mut stringbuf).map_or_else(
				|e| {
					Err(Error::with_description(
						&format!("failed reading file: {}: {}", path, e),
						ErrorKind::Io,
					))
				},
				|_res| Ok("".to_string()),
			);

			if res.is_err() {
				return res;
			}

			// Split the input file buffer on newlines.
			let stringbuf: Vec<&str> = stringbuf.split("\n").collect();

			// Convert the input file to WaveCAD format.
			let buf: Vec<i16> = stringbuf
				.iter()
				.map(|val| {
					val.chars()
						.map(|c| {
							// Convert number characters to integers.
							c.to_digit(10).unwrap() as i16
						})
						.fold(
							// Join individual digits into full numbers.
							0,
							|acc, elem| acc * 10 + elem,
						)
				})
				.map(
					// Adjust the numbers so they're in the correct
					// range for the WaveCAD format.
					|n| n - 2048,
				)
				.collect();

			// Remove trailing 0 because of newline.
			let buf = &buf[..buf.len() - 1];

			// Buffer to hold the final WaveCAD format bytes.
			let mut outbuf = [0u8; 4096];

			// Write to the WaveCAD buffer.
			LittleEndian::write_i16_into(&buf, &mut outbuf);

			// Save the new WaveCAD file.
			match fs::File::create(&format!("{}", new_path)) {
				Ok(mut outfile) => {
					res = outfile.write_all(&outbuf).map_or_else(
						|e| {
							Err(Error::with_description(&format!("failed writing to file: {}: {}", new_path, e), ErrorKind::Io))
						},
						|_res| {
							if output_binary {
								if verbose > 0 {
									return Err(Error::with_description(&format!("failed outputting binary to stdout: you can't do this when the verbosity level is greater than 0"), ErrorKind::InvalidValue));
								}

								let mut out = std::io::stdout();
								out.write_all(&outbuf).unwrap();
								out.flush().unwrap();
							}

							Ok("".to_string())
						}
					);

					if res.is_err() {
						return res;
					}

					if verbose > 0 {
						println!(
							"\nText file converted to WaveCAD and saved: {} -> {}",
							path, new_path
						);
					}
				}

				Err(e) => {
					res = Err(Error::with_description(
						&format!("failed creating file: {}: {}", new_path, e),
						ErrorKind::Io,
					));
					return res;
				}
			}
		}

		Err(e) => {
			res = Err(Error::with_description(
				&format!("failed opening file: {}: {}", path, e),
				ErrorKind::Io,
			));
			return res;
		}
	}

	res
}

/** Write a user-defined arbitrary waveform to the device.  
  
Use the helper function 
[set_arbitrary_wave_stdin](fn.set_arbitrary_wave_stdin.html) 
instead to accept the text input for the "data" parameter 
from stdin.  
  
Specify which arbitrary wave preset to save it in, as the
"amount" parameter.  
  
"data" parameter, some user-defined wave. Must be 2048 
lines of ASCII whole numbers, each in the range of 0 - 4095, 
followed by one extra blank line. For example:
```ignore
"2456"
"3016"
"4054"
"4001"
...
"3244"
"2010"
"1012"
  
```
*/
pub fn set_arbitrary_wave(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	data: &[String],
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < SET_ARBITRARY_WAVE_ARG_NUM_MIN || amount > SET_ARBITRARY_WAVE_ARG_NUM_MAX {
		return Err(Error::with_description(
			&format!(
				"Unsupported slot number. Must be {}-{}.",
				SET_ARBITRARY_WAVE_ARG_NUM_MIN, SET_ARBITRARY_WAVE_ARG_NUM_MAX
			),
			ErrorKind::InvalidValue,
		));
	}

	if verbose > 0 {
		println!("\nInput arbitrary waveform data (one integer from 0-4095 per line, and 2048 lines total):\n");
	}

	let amount_str = format!("{:02}", amount);

	let mut arg = String::new();

	let mut i = 0;
	for line in data {
		let line_len = line.chars().collect::<Vec<char>>().len();
		i += 1;

		if line_len < 1 {
			break;
		}

		match line.parse::<u32>() {
			Ok(num) => {
				if num > 4095 {
					return Err(Error::with_description(&format!("Invalid arbitrary wave data. Must be 2048 lines of integers in the range of 0 - 4095: Number out of range: {}: on line: {}", line, i), ErrorKind::InvalidValue));
				}

				arg += &(line.chars().collect::<String>() + ",");
			}

			Err(e) => {
				return Err(Error::with_description(&format!("Invalid arbitrary wave data. Must be 2048 lines of integers in the range of 0 - 4095: Invalid number: {}: on line: {}: {}", line, i, e), ErrorKind::InvalidValue));
			}
		}
	}

	// 2048 for stdin, 2049 for wavecad
	if i != 2048 && i != 2049 {
		return Err(Error::with_description(&format!("Invalid arbitrary wave data. Must be 2048 lines of integers in the range of 0 - 4095: Incorrect number of lines: {}", i), ErrorKind::InvalidValue));
	}

	arg = arg
		.chars()
		.take(arg.chars().collect::<Vec<char>>().len() - 1)
		.collect::<String>()
		.to_string();

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		SET_ARBITRARY_WAVE_COMMAND,
		amount_str,
		COMMAND_SEPARATOR,
		arg,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nWriting arbitrary wave to slot {}:\n\n{}",
			amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..SET_ARBITRARY_WAVE_RES_LEN).collect();

	port.write(&inbuf[..])?;
	port.read(&mut outbuf[..])?;

	let res = str::from_utf8(&outbuf).unwrap();

	if verbose > 0 {
		println!("Response:");
		println!("{}", res);
	}

	Ok(res.to_string())
}

/** Write a user-defined arbitrary waveform to the device from a WaveCAD (.wav) 
file, saving it into one of the device's arbitrary wave preset slots.  
  
"arg" parameter:
```ignore
Save the wav file into preset 1:
"1,some-wav-file-to-upload.wav"
```
*/
pub fn set_arbitrary_wavecad(
	mut port: &mut Box<dyn SerialPort>,
	arg: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let arg_parts: Vec<&str> = arg.split(",").collect();

	if arg_parts.len() < 2 {
		return Err(Error::with_description(&format!("unsupported value passed to \"write arbitrary wavecad\" argument (must be {}-{},<file_path>): slot number and file path must be present and separated with a comma but no space: {}", SET_ARBITRARY_WAVE_ARG_NUM_MIN, SET_ARBITRARY_WAVE_ARG_NUM_MAX, arg), ErrorKind::InvalidValue));
	}

	let amount = arg_parts[0];

	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"write arbitrary wavecad\" argument (must be {}-{}): {}: too many decimal places (0 max)", SET_ARBITRARY_WAVE_ARG_NUM_MIN, SET_ARBITRARY_WAVE_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
	}

	let path = arg_parts[1];

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= SET_ARBITRARY_WAVE_ARG_NUM_MIN
				&& amount <= SET_ARBITRARY_WAVE_ARG_NUM_MAX =>
			{
				let data = wav_to_txt(path, verbose);

				if data.is_err() {
					return data;
				}

				let data: Vec<String> = data
					.unwrap()
					.split("\n")
					.map(|res| res.to_string())
					.collect();

				res = set_arbitrary_wave(&mut port, amount, &data, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"write arbitrary wavecad\" argument (must be {}-{}): {}", SET_ARBITRARY_WAVE_ARG_NUM_MIN, SET_ARBITRARY_WAVE_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"write arbitrary wavecad\" argument (must be {}-{}): {}: {}", SET_ARBITRARY_WAVE_ARG_NUM_MIN, SET_ARBITRARY_WAVE_ARG_NUM_MAX, amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

/** Write a user-defined arbitrary waveform to the device from stdin.  
  
Specify which arbitrary wave preset to save it in, as the
"amount" parameter.  
  
The stdin values define some user-defined wave. Must be 2048 
lines of ASCII whole numbers, each in the range of 0 - 4095, 
followed by one extra blank line. For example:
```ignore
"2456"
"3016"
"4054"
"4001"
...
"3244"
"2010"
"1012"
  
```
*/
pub fn set_arbitrary_wave_stdin(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"write arbitrary wave\" argument (must be {}-{}): {}: too many decimal places (0 max)", SET_ARBITRARY_WAVE_ARG_NUM_MIN, SET_ARBITRARY_WAVE_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= SET_ARBITRARY_WAVE_ARG_NUM_MIN
				&& amount <= SET_ARBITRARY_WAVE_ARG_NUM_MAX =>
			{
				res = set_arbitrary_wave_stdin_inner(&mut port, amount, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"write arbitrary wave\" argument (must be {}-{}): {}", SET_ARBITRARY_WAVE_ARG_NUM_MIN, SET_ARBITRARY_WAVE_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"write arbitrary wave\" argument (must be {}-{}): {}: {}", SET_ARBITRARY_WAVE_ARG_NUM_MIN, SET_ARBITRARY_WAVE_ARG_NUM_MAX, amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn set_arbitrary_wave_stdin_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let data: Vec<String> = io::stdin()
		.lock()
		.lines()
		.collect::<Result<_, _>>()
		.map_or_else(|_e| Vec::new(), |res| res);

	let data_len = data.len();

	if data_len != 2048 {
		return Err(Error::with_description(&format!("Invalid arbitrary wave data from stdin. Must be 2048 lines of integers in the range of 0 - 4095: Incorrect number of lines: {}", data_len), ErrorKind::InvalidValue));
	}

	return set_arbitrary_wave(port, amount, &data[0..2048], verbose);
}

/** Read a user-defined arbitrary waveform from one of the device's 
numbered arbitrary wave preset slots.  
  
"amount" parameter:
```ignore
Get the waveform data which is stored in preset 1:
"1"
```
*/
pub fn get_arbitrary_wave(
	mut port: &mut Box<dyn SerialPort>,
	amount: &str,
	verbose: u64,
) -> Result<String, clap::Error> {
	let amount_parts: Vec<&str> = amount.split(".").collect();

	if amount_parts.len() > 1 {
		return Err(Error::with_description(&format!("unsupported value passed to \"read arbitrary wave\" argument (must be {}-{}): {}: too many decimal places (0 max)", SET_ARBITRARY_WAVE_ARG_NUM_MIN, SET_ARBITRARY_WAVE_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
	}

	let res: Result<String, clap::Error>;

	match amount.parse::<f64>() {
		Ok(amount) => match amount {
			_y if amount >= GET_ARBITRARY_WAVE_ARG_NUM_MIN
				&& amount <= GET_ARBITRARY_WAVE_ARG_NUM_MAX =>
			{
				res = get_arbitrary_wave_inner(&mut port, amount, verbose);
			}

			_ => {
				res = Err(Error::with_description(&format!("unsupported value passed to \"read arbitrary wave\" argument (must be {}-{}): {}", GET_ARBITRARY_WAVE_ARG_NUM_MIN, GET_ARBITRARY_WAVE_ARG_NUM_MAX, amount), ErrorKind::InvalidValue));
			}
		},

		Err(e) => {
			res = Err(Error::with_description(&format!("unsupported value passed to \"read arbitrary wave\" argument (must be {}-{}): {}: {}", GET_ARBITRARY_WAVE_ARG_NUM_MIN, GET_ARBITRARY_WAVE_ARG_NUM_MAX, amount, e), ErrorKind::InvalidValue));
		}
	}

	res
}

fn get_arbitrary_wave_inner(
	port: &mut Box<dyn SerialPort>,
	amount: f64,
	verbose: u64,
) -> Result<String, clap::Error> {
	let command: String;

	if amount < GET_ARBITRARY_WAVE_ARG_NUM_MIN || amount > GET_ARBITRARY_WAVE_ARG_NUM_MAX {
		return Err(Error::with_description(
			&format!(
				"Unsupported slot number. Must be {}-{}.",
				GET_ARBITRARY_WAVE_ARG_NUM_MIN, GET_ARBITRARY_WAVE_ARG_NUM_MAX
			),
			ErrorKind::InvalidValue,
		));
	}

	let amount_str = format!("{:02}", amount);

	command = format!(
		"{}{}{}{}{}{}",
		COMMAND_BEGIN,
		GET_ARBITRARY_WAVE_COMMAND,
		amount_str,
		COMMAND_SEPARATOR,
		GET_ARBITRARY_WAVE_ARG2,
		COMMAND_END,
	);

	if verbose > 0 {
		println!(
			"\nReading arbitrary wave from slot: {}:\n{}",
			amount, command
		);
	}

	let inbuf: Vec<u8> = command.as_bytes().to_vec();
	let mut outbuf: Vec<u8> = (0..GET_ARBITRARY_WAVE_RES_LEN).map(|_val| 0u8).collect();

	port.write(&inbuf[..])?;
	let mut n = 0;
	let mut chunk: &str;

	while n < GET_ARBITRARY_WAVE_RES_LEN as usize {
		match port.read(&mut outbuf[n..]) {
			Ok(val) => {
				// Track buffer position.
				n += val;

				// Count number of commas to detect end of buffer.
				chunk = str::from_utf8(&outbuf[..]).unwrap();
				let num_commas = chunk.matches(',').count();
				if num_commas == 2048 {
					if verbose > 0 {
						println!("\nReached end of buffer: Detected all expected commas\n");
					}

					break;
				}
			}

			Err(e) => {
				return Err(Error::with_description(
					&format!("Reached end of buffer unexpectedly: {}.", e),
					ErrorKind::InvalidValue,
				));
			}
		}
	}

	let res = str::from_utf8(&outbuf[..n]).unwrap();

	let res_parts: Vec<&str> = res.split("=").collect();

	if res_parts.len() < 2 {
		return Err(Error::with_description(
			&format!("Invalid response from device: {}", res),
			ErrorKind::Io,
		));
	}

	let res_data: Vec<&str> = res_parts[1].split(",").collect();
	let res_data = &res_data[..res_data.len() - 1];

	let res_str = res_data.join("\n");

	if verbose > 0 {
		println!("Response size: {} bytes\n", n);
		println!("Response:");
		println!("{}\n", res);
	} else {
		println!("{}", res_str);
	}

	Ok(res_str)
}
