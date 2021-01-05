/* Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>

By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.
*/

/*! Some code which initiates the correct type of 
serial connection to properly communicate with the
device.
*/

extern crate serial;

use crate::protocol::*;
use std::io;
use std::time::Duration;

use serial::prelude::*;

/** A type which wraps the serial crate's SerialPort type,
for test mocking convenience.  
  
All of the functions in the `command` module expect a value 
of this type as their first parameter.
*/
pub struct SerialPortType {
	/// The serial port connection.
	pub port: Option<Box<dyn SerialPort>>,

	/** Are we mocking a serial port for testing? If true, the 
	`port` variable	will be `None` and you shouldn't try to 
	use it.
	*/
	pub mock: bool,
}

impl SerialPortType {
	/** The SerialPortType constructor. Pass in the path to the device
	for `arg` and set `mock` to false, to open a serial connection to
	a real device.  
	  
	If you want to instantiate without connecting to a serial device,
	maybe for test mocking convenience for example, pass an empty
	string for `arg` and set `mock` to true.
	 */
	pub fn new(arg: &str, mock: bool) -> io::Result<SerialPortType> {
		if mock {
			Ok(
				Self{
					port: None,
					mock,
				}
			)

		} else {
			let port = open(&arg)?;

			Ok(
				Self{
					port: Some(port),
					mock,
				}
			)
		}
	}
}

/** Open a serial communication link with the device,
and configure it so it can communicate properly.  
  
"arg" parameter (on Linux it defaults to "/dev/ttyUSB0", 
on Windows it defaults to "COM3"):
```ignore
Path to a serial device (Linux):
"/dev/ttyUSB0"

Path to a serial device (Windows):
"COM3"
```
*/
pub fn open(arg: &str) -> io::Result<Box<dyn SerialPort>> {
	let mut port = Box::new(serial::open(&arg)?);

	port.reconfigure(&|settings| {
		settings.set_baud_rate(serial::Baud115200)?;
		settings.set_char_size(serial::Bits8);
		settings.set_parity(serial::ParityNone);
		settings.set_stop_bits(serial::Stop1);
		settings.set_flow_control(serial::FlowNone);
		Ok(())
	})?;

	port.set_timeout(Duration::from_millis(SERIAL_TIMEOUT_MS))?;

	Ok(port)
}
