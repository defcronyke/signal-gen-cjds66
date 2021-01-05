/* Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>

By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.
*/

/*! Unit Tests */

#[cfg(test)]
mod test {
	use super::super::serial;
	use self::serial::*;

	use super::super::command;
	use self::command::*;
	
	#[test]
	pub fn get_model_ok() {
		let device: &str = "";
		let mock = true;
		let verbose = 1;

		let mut port = SerialPortType::new(device, mock).unwrap();

		get_model(&mut port, verbose).unwrap();
	}

	#[test]
	pub fn get_serial_ok() {
		let device: &str = "";
		let mock = true;
		let verbose = 1;

		let mut port = SerialPortType::new(device, mock).unwrap();

		get_serial(&mut port, verbose).unwrap();
	}

	#[test]
	pub fn get_model_and_serial_ok() {
		let device: &str = "";
		let mock = true;
		let verbose = 1;

		let mut port = SerialPortType::new(device, mock).unwrap();

		get_model_and_serial(&mut port, verbose).unwrap();
	}

	#[test]
	pub fn set_channel_output_ok() {
		let device: &str = "";
		let mock = true;
		let verbose = 1;

		let mut port = SerialPortType::new(device, mock).unwrap();

		let zero = "0";
		let one = "1";

		let zero_zero = "00";
		let zero_one = "01";
		let one_one = "11";
		let one_zero = "10";

		let zero_c_zero = "0,0";
		let zero_c_one = "0,1";
		let one_c_one = "1,1";
		let one_c_zero = "1,0";

		let off = "off";
		let on = "on";

		let off_off = "off,off";
		let off_on = "off,on";
		let on_on = "on,on";
		let on_off = "on,off";

		set_channel_output(&mut port, zero, verbose).unwrap();
		set_channel_output(&mut port, one, verbose).unwrap();
		set_channel_output(&mut port, zero_zero, verbose).unwrap();
		set_channel_output(&mut port, zero_one, verbose).unwrap();
		set_channel_output(&mut port, one_one, verbose).unwrap();
		set_channel_output(&mut port, one_zero, verbose).unwrap();
		set_channel_output(&mut port, zero_c_zero, verbose).unwrap();
		set_channel_output(&mut port, zero_c_one, verbose).unwrap();
		set_channel_output(&mut port, one_c_one, verbose).unwrap();
		set_channel_output(&mut port, one_c_zero, verbose).unwrap();
		set_channel_output(&mut port, off, verbose).unwrap();
		set_channel_output(&mut port, on, verbose).unwrap();
		set_channel_output(&mut port, off_off, verbose).unwrap();
		set_channel_output(&mut port, off_on, verbose).unwrap();
		set_channel_output(&mut port, on_on, verbose).unwrap();
		set_channel_output(&mut port, on_off, verbose).unwrap();
	}

	#[test]
	pub fn set_channel_output_err() {
		let device: &str = "";
		let mock = true;
		let verbose = 1;

		let mut port = SerialPortType::new(device, mock).unwrap();

		let too_many_digits = "012";
		let too_many_digits_comma = "01,2";
		let invalid_digit = "2";
		let invalid_digits = "02";
		let invalid_decimal = "1.0";
		let invalid_decimals_comma = "1.0,0";
		let invalid_digit_comma = "0,2";
		let too_many_words = "off,on,on";
		let invalid_word = "fonff";
		let invalid_words = "off,fonff";
		let mixed_digit_word = "1,off";
		
		set_channel_output(&mut port, too_many_digits, verbose).unwrap_err();
		set_channel_output(&mut port, too_many_digits_comma, verbose).unwrap_err();
		set_channel_output(&mut port, invalid_digit, verbose).unwrap_err();
		set_channel_output(&mut port, invalid_digits, verbose).unwrap_err();
		set_channel_output(&mut port, invalid_decimal, verbose).unwrap_err();
		set_channel_output(&mut port, invalid_decimals_comma, verbose).unwrap_err();
		set_channel_output(&mut port, invalid_digit_comma, verbose).unwrap_err();
		set_channel_output(&mut port, too_many_words, verbose).unwrap_err();
		set_channel_output(&mut port, invalid_word, verbose).unwrap_err();
		set_channel_output(&mut port, invalid_words, verbose).unwrap_err();
		set_channel_output(&mut port, mixed_digit_word, verbose).unwrap_err();
	}
}
