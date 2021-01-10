/* Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>

By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.
*/

/*! Unit Tests */

use super::serial::*;
use super::command::*;
use super::protocol::*;
	
#[test]
pub fn get_model_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;

	for verbose in 0..(verbose_max + 1) {
		get_model(&mut port, verbose).unwrap();
	}
}


#[test]
pub fn get_serial_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;

	for verbose in 0..(verbose_max + 1) {
		get_serial(&mut port, verbose).unwrap();
	}
}


#[test]
pub fn get_model_and_serial_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;

	for verbose in 0..(verbose_max + 1) {
		get_model_and_serial(&mut port, verbose).unwrap();
	}
}


#[test]
pub fn set_channel_output_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;

	let args = [
		"0",
		"1",
		
		"00",
		"01",
		"11",
		"10",
		
		"0,0",
		"0,1",
		"1,1",
		"1,0",
		
		"off",
		"on",

		"off,off",
		"off,on",
		"on,on",
		"on,off",
	];

	for verbose in 0..(verbose_max + 1) {
		for arg in args.iter() {
			set_channel_output(&mut port, arg, verbose).unwrap();
		}
	}
}

#[test]
pub fn set_channel_output_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;

	let args = [
		"012",			// too many digits
		"01,2",			// too many digits comma
		"2",			// invalid digit
		"02",			// invalid digits
		"1.0",			// invalid decimal
		"1.0,0",		// invalid decimals comma
		"0,2",			// invalid digit comma
		"off,on,on",	// too many words
		"fonff",		// invalid word
		"off,fonff",	// invalid words
		"1,off",		// mixed digit word
	];
	
	for verbose in 0..(verbose_max + 1) {
		for arg in args.iter() {
			set_channel_output(&mut port, arg, verbose).unwrap_err();
		}
	}
}


#[test]
pub fn get_channel_output_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;

	for verbose in 0..(verbose_max + 1) {
		get_channel_output(&mut port, verbose).unwrap();
	}
}


#[test]
pub fn set_waveform_preset_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;
	let presets = 16;

	let words = [
		"sine",
		"sin",

		"square",
		"sq",

		"pulse",
		"pul",

		"triangle",
		"tri",

		"partialsine",
		"partial-sine",
		"parsine",
		"par-sine",
		"parsin",
		"par-sin",
		"psine",
		"p-sine",
		"psin",
		"p-sin",

		"cmos",
		"cm",

		"dc",

		"halfwave",
		"half-wave",
		"hw",
		"h-w",

		"fullwave",
		"full-wave",
		"fw",
		"f-w",

		"posladder",
		"pos-ladder",
		"poslad",
		"pos-lad",
		"positiveladder",
		"positive-ladder",
		"pl",

		"negladder",
		"neg-ladder",
		"neglad",
		"neg-lad",
		"negativeladder",
		"negative-ladder",
		"nl",

		"noise",
		"nois",
		"noi",
		"no",
		"n",

		"exprise",
		"exp-rise",
		"er",
		"e-r",
		"erise",
		"e-rise",
		"eris",
		"e-ris",

		"expdecay",
		"exp-decay",
		"ed",
		"e-d",
		"edecay",
		"e-decay",
		"edec",
		"e-dec",

		"multitone",
		"multi-tone",
		"mt",
		"m-t",
		"mtone",
		"m-tone",

		"sinc",
		"sc",

		"lorenz",
		"loren",
		"lor",
		"lz",
	];

	// Test word inputs.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			for word in words.iter() {
				set_waveform_preset(&mut port, chan, word, verbose).unwrap();
			}
		}
	}

	// Test number inputs.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			for preset in 0..(presets + 1) {
				set_waveform_preset(&mut port, chan, &preset.to_string(), verbose).unwrap();
			}
		}
	}
}

#[test]
pub fn set_waveform_preset_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;
	let presets = 16;

	let words = [
		"grampn",	// invalid word
		"",			// empty string
	];

	// Test invalid word inputs.
	for verbose in 0..(verbose_max + 1) {
		for word in words.iter() {
			set_waveform_preset(&mut port, chans, word, verbose).unwrap_err();
		}
	}

	// Test invalid number inputs.
	for verbose in 0..(verbose_max + 1) {
		set_waveform_preset(&mut port, chans, &(presets + 1).to_string(), verbose).unwrap_err();
	}

	// Test invalid channels.
	for verbose in 0..(verbose_max + 1) {
		set_waveform_preset(&mut port, chans + 1, &presets.to_string(), verbose).unwrap_err();
	}
}


#[test]
pub fn get_waveform_preset_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			get_waveform_preset(&mut port, chan, verbose).unwrap();
		}
	}
}

#[test]
pub fn get_waveform_preset_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test invalid channels
	for verbose in 0..(verbose_max + 1) {
		get_waveform_preset(&mut port, chans + 1, verbose).unwrap_err();
	}
}


#[test]
pub fn set_waveform_preset_arbitrary_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;
	let presets = 60;

	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			for preset in 1..(presets + 1) {
				set_waveform_preset_arbitrary(&mut port, chan, &preset.to_string(), verbose).unwrap();
			}
		}
	}
}

#[test]
pub fn set_waveform_preset_arbitrary_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;
	let presets = 60;

	// Test invalid preset.
	for verbose in 0..(verbose_max + 1) {
		set_waveform_preset_arbitrary(&mut port, chans, &(presets + 1).to_string(), verbose).unwrap_err();
	}

	// Test invalid channel.
	for verbose in 0..(verbose_max + 1) {
		set_waveform_preset_arbitrary(&mut port, chans + 1, &presets.to_string(), verbose).unwrap_err();
	}
}


#[test]
pub fn set_frequency_microhertz_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test arg min.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_microhertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MIN.to_string(), verbose).unwrap();
		}
	}

	// Test arg max.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_microhertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MAX.to_string(), verbose).unwrap();
		}
	}

	// Test arg middle.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_microhertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MAX / 2.0).to_string(), verbose).unwrap();
		}
	}

	// Test decimal places.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			for decimal in 1..SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MAX_DECIMAL_PLACES {
				set_frequency_microhertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf(decimal as f64)))).to_string(), verbose).unwrap();
			}
		}
	}
}

#[test]
pub fn set_frequency_microhertz_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test greater than arg max.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_microhertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MAX + 1.0).to_string(), verbose).unwrap_err();
	}

	// Test less than arg min.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_microhertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MIN - 1.0).to_string(), verbose).unwrap_err();
	}

	// Test too many decimal places.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_microhertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf((SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MAX_DECIMAL_PLACES + 1) as f64)))).to_string(), verbose).unwrap_err();
	}

	// Test invalid channel.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_microhertz(&mut port, chans + 1, &(SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MIN).to_string(), verbose).unwrap_err();
	}
}


#[test]
pub fn set_frequency_millihertz_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test arg min.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_millihertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MIN.to_string(), verbose).unwrap();
		}
	}

	// Test arg max.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_millihertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MAX.to_string(), verbose).unwrap();
		}
	}

	// Test arg middle.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_millihertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MAX / 2.0).to_string(), verbose).unwrap();
		}
	}

	// Test decimal places.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			for decimal in 1..SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MAX_DECIMAL_PLACES {
				set_frequency_millihertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf(decimal as f64)))).to_string(), verbose).unwrap();
			}
		}
	}
}

#[test]
pub fn set_frequency_millihertz_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test greater than arg max.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_millihertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MAX + 1.0).to_string(), verbose).unwrap_err();
	}

	// Test less than arg min.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_millihertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MIN - 1.0).to_string(), verbose).unwrap_err();
	}

	// Test too many decimal places.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_millihertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf((SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MAX_DECIMAL_PLACES + 1) as f64)))).to_string(), verbose).unwrap_err();
	}

	// Test invalid channel.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_millihertz(&mut port, chans + 1, &(SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ_ARG_MIN).to_string(), verbose).unwrap_err();
	}
}


#[test]
pub fn set_frequency_hertz_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test arg min.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_hertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MIN.to_string(), verbose).unwrap();
		}
	}

	// Test arg max.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_hertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MAX.to_string(), verbose).unwrap();
		}
	}

	// Test arg middle.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_hertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MAX / 2.0).to_string(), verbose).unwrap();
		}
	}

	// Test decimal places.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			for decimal in 1..SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MAX_DECIMAL_PLACES {
				set_frequency_hertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf(decimal as f64)))).to_string(), verbose).unwrap();
			}
		}
	}
}

#[test]
pub fn set_frequency_hertz_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test greater than arg max.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_hertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MAX + 1.0).to_string(), verbose).unwrap_err();
	}

	// Test less than arg min.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_hertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MIN - 1.0).to_string(), verbose).unwrap_err();
	}

	// Test too many decimal places.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_hertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf((SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MAX_DECIMAL_PLACES + 1) as f64)))).to_string(), verbose).unwrap_err();
	}

	// Test invalid channel.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_hertz(&mut port, chans + 1, &(SET_FREQUENCY_COMMAND_UNIT_HERTZ_ARG_MIN).to_string(), verbose).unwrap_err();
	}
}


#[test]
pub fn set_frequency_kilohertz_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test arg min.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_kilohertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MIN.to_string(), verbose).unwrap();
		}
	}

	// Test arg max.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_kilohertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MAX.to_string(), verbose).unwrap();
		}
	}

	// Test arg middle.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_kilohertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MAX / 2.0).to_string(), verbose).unwrap();
		}
	}

	// Test decimal places.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			for decimal in 1..SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MAX_DECIMAL_PLACES {
				set_frequency_kilohertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf(decimal as f64)))).to_string(), verbose).unwrap();
			}
		}
	}
}

#[test]
pub fn set_frequency_kilohertz_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test greater than arg max.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_kilohertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MAX + 1.0).to_string(), verbose).unwrap_err();
	}

	// Test less than arg min.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_kilohertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MIN - 1.0).to_string(), verbose).unwrap_err();
	}

	// Test too many decimal places.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_kilohertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf((SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MAX_DECIMAL_PLACES + 1) as f64)))).to_string(), verbose).unwrap_err();
	}

	// Test invalid channel.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_kilohertz(&mut port, chans + 1, &(SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ_ARG_MIN).to_string(), verbose).unwrap_err();
	}
}


#[test]
pub fn set_frequency_megahertz_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test arg min.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_megahertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MIN.to_string(), verbose).unwrap();
		}
	}

	// Test arg max.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_megahertz(&mut port, chan, &SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MAX.to_string(), verbose).unwrap();
		}
	}

	// Test arg middle.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_frequency_megahertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MAX / 2.0).to_string(), verbose).unwrap();
		}
	}

	// Test decimal places.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			for decimal in 1..SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MAX_DECIMAL_PLACES {
				set_frequency_megahertz(&mut port, chan, &(SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf(decimal as f64)))).to_string(), verbose).unwrap();
			}
		}
	}
}

#[test]
pub fn set_frequency_megahertz_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test greater than arg max.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_megahertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MAX + 1.0).to_string(), verbose).unwrap_err();
	}

	// Test less than arg min.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_megahertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MIN - 1.0).to_string(), verbose).unwrap_err();
	}

	// Test too many decimal places.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_megahertz(&mut port, chans, &(SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MIN + (1.0 / ((10.0 as f64).powf((SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MAX_DECIMAL_PLACES + 1) as f64)))).to_string(), verbose).unwrap_err();
	}

	// Test invalid channel.
	for verbose in 0..(verbose_max + 1) {
		set_frequency_megahertz(&mut port, chans + 1, &(SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ_ARG_MIN).to_string(), verbose).unwrap_err();
	}
}


#[test]
pub fn get_frequency_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			get_frequency(&mut port, chan, verbose).unwrap();
		}
	}
}

#[test]
pub fn get_frequency_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test invalid channels
	for verbose in 0..(verbose_max + 1) {
		get_frequency(&mut port, chans + 1, verbose).unwrap_err();
	}
}


#[test]
pub fn get_frequency_hertz_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			get_frequency_hertz(&mut port, chan, verbose).unwrap();
		}
	}
}

#[test]
pub fn get_frequency_hertz_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test invalid channels
	for verbose in 0..(verbose_max + 1) {
		get_frequency_hertz(&mut port, chans + 1, verbose).unwrap_err();
	}
}


#[test]
pub fn set_amplitude_ok() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test arg min.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_amplitude(&mut port, chan, &SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MIN.to_string(), verbose).unwrap();
		}
	}

	// Test arg max.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_amplitude(&mut port, chan, &SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MAX.to_string(), verbose).unwrap();
		}
	}

	// Test arg middle.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			set_amplitude(&mut port, chan, &(SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MAX / 2.0).to_string(), verbose).unwrap();
		}
	}

	// Test decimal places.
	for verbose in 0..(verbose_max + 1) {
		for chan in 1..(chans + 1) {
			for decimal in 1..SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MAX_DECIMAL_PLACES {
				set_amplitude(&mut port, chan, &(SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MIN + (1.0 / ((10.0 as f64).powf(decimal as f64)))).to_string(), verbose).unwrap();
			}
		}
	}
}

#[test]
pub fn set_amplitude_err() {
	let mut port = SerialPortType::new("", true).unwrap();
	let verbose_max = 1;
	let chans = 2;

	// Test greater than arg max.
	for verbose in 0..(verbose_max + 1) {
		set_amplitude(&mut port, chans, &(SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MAX + 1.0).to_string(), verbose).unwrap_err();
	}

	// Test less than arg min.
	for verbose in 0..(verbose_max + 1) {
		set_amplitude(&mut port, chans, &(SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MIN - 1.0).to_string(), verbose).unwrap_err();
	}

	// Test too many decimal places.
	for verbose in 0..(verbose_max + 1) {
		set_amplitude(&mut port, chans, &(SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MIN + (1.0 / ((10.0 as f64).powf((SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MAX_DECIMAL_PLACES + 1) as f64)))).to_string(), verbose).unwrap_err();
	}

	// Test invalid channel.
	for verbose in 0..(verbose_max + 1) {
		set_amplitude(&mut port, chans + 1, &(SET_AMPLITUDE_COMMAND_UNIT_VOLTS_ARG_MIN).to_string(), verbose).unwrap_err();
	}
}
