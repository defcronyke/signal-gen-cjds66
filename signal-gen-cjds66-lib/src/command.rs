extern crate serial;

use std::io;
use std::thread;
use std::time::{Duration};
use crate::protocol::*;

use std::io::prelude::*;
use serial::prelude::*;
use std::str;

pub fn get_machine_model(port: &mut Box<dyn SerialPort>) -> io::Result<()> {
    println!("\nRequesting machine model:\n{}", GET_MACHINE_MODEL);

    let inbuf: Vec<u8> = GET_MACHINE_MODEL.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..GET_MACHINE_MODEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    println!("Response:");
    println!("{}", str::from_utf8(&outbuf).unwrap());

    thread::sleep(Duration::from_millis(50));

    Ok(())
}

pub fn get_machine_number(port: &mut Box<dyn SerialPort>) -> io::Result<()> {
    println!("\nRequesting machine number:\n{}", GET_MACHINE_NUMBER);

    let inbuf: Vec<u8> = GET_MACHINE_NUMBER.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..GET_MACHINE_NUMBER_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    println!("Response:");
    println!("{}", str::from_utf8(&outbuf).unwrap());

    thread::sleep(Duration::from_millis(50));

    Ok(())
}
