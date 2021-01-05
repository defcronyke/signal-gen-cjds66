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

		get_model(&mut port, verbose).unwrap();
	}
}
