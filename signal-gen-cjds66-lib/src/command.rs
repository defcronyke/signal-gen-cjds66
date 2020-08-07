extern crate serial;

use std::io;
use std::io::{Error, ErrorKind};
use std::thread;
use std::time::{Duration};
use crate::protocol::*;

use std::io::prelude::*;
use serial::prelude::*;
use std::str;

pub fn read_machine_model(port: &mut Box<dyn SerialPort>) -> io::Result<()> {
    println!("\nRequesting machine model number:\n{}", READ_MACHINE_MODEL);

    let inbuf: Vec<u8> = READ_MACHINE_MODEL.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..READ_MACHINE_MODEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    println!("Response:");
    println!("{}", str::from_utf8(&outbuf).unwrap());

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(())
}

pub fn read_machine_number(port: &mut Box<dyn SerialPort>) -> io::Result<()> {
    println!("\nRequesting machine serial number:\n{}", READ_MACHINE_NUMBER);

    let inbuf: Vec<u8> = READ_MACHINE_NUMBER.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..READ_MACHINE_NUMBER_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    println!("Response:");
    println!("{}", str::from_utf8(&outbuf).unwrap());

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(())
}

pub fn set_channel_output(port: &mut Box<dyn SerialPort>, ch1: bool, ch2: bool) -> io::Result<String> {
    let command: &str;
    
    // Supported states.
    if ch1 && ch2 { // Both on.
        command = WRITE_CHANNEL_OUTPUT_BOTH_ON;
    } else if !ch1 && !ch2 {  // Both off.
        command = WRITE_CHANNEL_OUTPUT_BOTH_OFF;
    } else if ch1 && !ch2 {  // ch1 on, ch2 off.
        command = WRITE_CHANNEL_OUTPUT_CH1_ON_CH2_OFF;
    } else if !ch1 && ch2 { // ch1 off, ch2 on.
        command = WRITE_CHANNEL_OUTPUT_CH1_OFF_CH2_ON;
    } else {
        return Err(Error::new(ErrorKind::Other, "unsupported input condition"));
    }
    
    println!("\nSetting channel output: ch1={} and ch2={}:\n{}", ch1, ch2, command);
    // println!("\nSetting channel output: ch1={}, ch2={}\n{}", ch1, ch2, SET_CHANNEL_OUTPUT);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_CHANNEL_OUTPUT_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}
