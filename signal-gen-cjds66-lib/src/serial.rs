extern crate serial;

use crate::protocol::*;
use std::io;
use std::time::Duration;

use serial::prelude::*;

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
