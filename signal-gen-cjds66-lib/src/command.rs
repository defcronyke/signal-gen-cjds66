extern crate serial;

use crate::protocol::*;

use std::io;
use std::io::{Error, ErrorKind};
use std::thread;
use std::time::{Duration};

use std::io::prelude::*;
use serial::prelude::*;
use std::str;

pub fn read_machine_model(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    println!("\nRequesting machine model number:\n{}", READ_MACHINE_MODEL);

    let inbuf: Vec<u8> = READ_MACHINE_MODEL.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..READ_MACHINE_MODEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn read_machine_number(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    println!("\nRequesting machine serial number:\n{}", READ_MACHINE_NUMBER);

    let inbuf: Vec<u8> = READ_MACHINE_NUMBER.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..READ_MACHINE_NUMBER_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
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

pub fn match_set_channel_output_arg(mut port: &mut Box<dyn SerialPort>, sco: &str) -> io::Result<String> {
    let res: io::Result<String>;
    
    match sco {
        "1,1" | "11" | "on,on" | "1" | "on" => {
            res = set_channel_output(&mut port, true, true);
        },
        
        "0,0" | "00" | "off,off" | "0" | "off" => {
            res = set_channel_output(&mut port, false, false);
        },

        "1,0" | "10" | "on,off" => {
            res = set_channel_output(&mut port, true, false);
        },

        "0,1" | "01" | "off,on" => {
            res = set_channel_output(&mut port, false, true);
        },

        _ => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"-o\" argument: {}", sco)));
        },
    }

    res
}


pub fn set_waveform_preset(port: &mut Box<dyn SerialPort>, chan: u64, preset: u64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_WAVEFORM_PRESET_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_WAVEFORM_PRESET_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if preset > 16 {
        return Err(Error::new(ErrorKind::Other, "Unsupported waveform preset number. Must be 0-16."));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        preset,
        COMMAND_END,
    );
    
    println!("\nSetting waveform preset: ch{}={}:\n{}", chan, preset, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_WAVEFORM_PRESET_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

// NOTE: See here for valid waveform preset names you 
// can use with -w and -x args if you'd rather use 
// names instead of numbers.
//
// Waveform names that are accepted:
// 0:  sine || sin
// 1:  square || sq
// 2:  pulse || pul
// 3:  triangle || tri
// 4:  partialsine || partial-sine || parsine || par-sine || parsin || par-sin || psine || p-sine || psin || p-sin
// 5:  cmos || cm
// 6:  dc
// 7:  halfwave || half-wave || hw || h-w
// 8:  fullwave || full-wave || fw || f-w
// 9:  pos-ladder || posladder || pos-lad || poslad || positive-ladder || positiveladder || pl
// 10: neg-ladder || negladder || neg-lad || neglad || negative-ladder || negativeladder || nl
// 11: noise || nois || noi || no || n
// 12: exp-rise || exprise || e-r || er || e-rise || erise || e-ris || eris
// 13: exp-decay || expdecay || e-d || ed || e-decay || edecay || e-dec || edec
// 14: multi-tone || multitone || m-t || mt || m-tone || mtone
// 15: sinc || sc
// 16: lorenz || loren || lor || lz
pub fn match_set_waveform_preset_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, preset: &str) -> io::Result<String> {
    let res: io::Result<String>;
    
    match preset.parse::<u64>() {
        Ok(preset) => {
            match preset {
                0..=16 => {
                    res = set_waveform_preset(&mut port, chan, preset);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set waveform\" argument (must be 0-16): {}", preset)));
                },
            }
        },

        Err(_e) => {
            match preset {
                "sine" | "sin" => {
                    res = set_waveform_preset(&mut port, chan, 0);
                },

                "square" | "sq" => {
                    res = set_waveform_preset(&mut port, chan, 1);
                },

                "pulse" | "pul" => {
                    res = set_waveform_preset(&mut port, chan, 2);
                },

                "triangle" | "tri" => {
                    res = set_waveform_preset(&mut port, chan, 3);
                },

                "partialsine" | "partial-sine" | "parsine" | "par-sine" | "parsin" | "par-sin" | "psine" | "p-sine" | "psin" | "p-sin" => {
                    res = set_waveform_preset(&mut port, chan, 4);
                },

                "cmos" | "cm" => {
                    res = set_waveform_preset(&mut port, chan, 5);
                },

                "dc" => {
                    res = set_waveform_preset(&mut port, chan, 6);
                },

                "halfwave" | "half-wave" | "hw" | "h-w" => {
                    res = set_waveform_preset(&mut port, chan, 7);
                },

                "fullwave" | "full-wave" | "fw" | "f-w" => {
                    res = set_waveform_preset(&mut port, chan, 8);
                },

                "pos-ladder" | "posladder" | "pos-lad" | "poslad" | "positive-ladder" | "positiveladder" | "pl" => {
                    res = set_waveform_preset(&mut port, chan, 9);
                },

                "neg-ladder" | "negladder" | "neg-lad" | "neglad" | "negative-ladder" | "negativeladder" | "nl" => {
                    res = set_waveform_preset(&mut port, chan, 10);
                },

                "noise" | "nois" | "noi" | "no" | "n" => {
                    res = set_waveform_preset(&mut port, chan, 11);
                },

                "exp-rise" | "exprise" | "e-r" | "er" | "e-rise" | "erise" | "e-ris" | "eris" => {
                    res = set_waveform_preset(&mut port, chan, 12);
                },

                "exp-decay" | "expdecay" | "e-d" | "ed" | "e-decay" | "edecay" | "e-dec" | "edec" => {
                    res = set_waveform_preset(&mut port, chan, 13);
                },

                "multi-tone" | "multitone" | "m-t" | "mt" | "m-tone" | "mtone" => {
                    res = set_waveform_preset(&mut port, chan, 14);
                },

                "sinc" | "sc" => {
                    res = set_waveform_preset(&mut port, chan, 15);
                },

                "lorenz" | "loren" | "lor" | "lz" => {
                    res = set_waveform_preset(&mut port, chan, 16);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set waveform\" argument (must be 0-16): {}", preset)));
                },
            }
        },
    }

    res
}


pub fn set_waveform_arbitrary(port: &mut Box<dyn SerialPort>, chan: u64, preset: u64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_WAVEFORM_PRESET_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_WAVEFORM_PRESET_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if preset < 1 || preset > 60 {
        return Err(Error::new(ErrorKind::Other, "Unsupported waveform preset number. Must be 1-60."));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        preset + 100,
        COMMAND_END,
    );
    
    println!("\nSetting waveform preset: ch{}={}:\n{}", chan, preset, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_WAVEFORM_PRESET_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_waveform_arbitrary_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, preset: &str) -> io::Result<String> {
    let res: io::Result<String>;
    
    match preset.parse::<u64>() {
        Ok(preset) => {
            match preset {
                1..=60 => {
                    res = set_waveform_arbitrary(&mut port, chan, preset);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set arbitrary waveform\" argument (must be 1-60): {}", preset)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set arbitrary waveform\" argument (must be 1-60): {}: {}", preset, e)));
        },
    }

    res
}


pub fn set_frequency_microhertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if amount < 1.0 || amount > 8000000000.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported amount of uHz. Must be 0.01-80000000.0."));
    }

    command = format!("{}{}{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_ARG_SEPARATOR,
        WRITE_FREQUENCY_COMMAND_UNIT_MICROHERTZ,
        COMMAND_END,
    );
    
    println!("\nSetting frequency in uHz: ch{}={}:\n{}", chan, amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_frequency_microherz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str) -> io::Result<String> {    
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency uHz\" argument (must be 0.01-80000000.0): {}: too many decimal places (2 max)", amount)));
    }

    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 80000000.0 => {
                    res = set_frequency_microhertz(&mut port, chan, amount * 100.0);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency uHz\" argument (must be 0.01-80000000.0): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency uHz\" argument (must be 0.01-80000000.0): {}: {}", amount, e)));
        },
    }

    res
}


pub fn set_frequency_millihertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if amount < 1.0 || amount > 8000000000.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported amount of mHz. Must be 0.01-80000000.0."));
    }

    command = format!("{}{}{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_ARG_SEPARATOR,
        WRITE_FREQUENCY_COMMAND_UNIT_MILLIHERTZ,
        COMMAND_END,
    );
    
    println!("\nSetting frequency in mHz: ch{}={}:\n{}", chan, amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_frequency_milliherz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str) -> io::Result<String> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-80000000.0): {}: too many decimal places (2 max)", amount)));
    }
    
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 80000000.0 => {
                    res = set_frequency_millihertz(&mut port, chan, amount * 100.0);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-80000000.0): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-80000000.0): {}: {}", amount, e)));
        },
    }

    res
}


pub fn set_frequency_hertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if amount < 1.0 || amount > 6000000000.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported amount of Hz. Must be 0.01-60000000.0."));
    }

    command = format!("{}{}{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_ARG_SEPARATOR,
        WRITE_FREQUENCY_COMMAND_UNIT_HERTZ,
        COMMAND_END,
    );
    
    println!("\nSetting frequency in Hz: ch{}={}:\n{}", chan, amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_frequency_hertz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str) -> io::Result<String> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency Hz\" argument (must be 0.01-60000000.0): {}: too many decimal places (2 max)", amount)));
    }
    
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 60000000.0 => {
                    res = set_frequency_hertz(&mut port, chan, amount * 100.0);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-60000000.0): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-60000000.0): {}: {}", amount, e)));
        },
    }

    res
}


pub fn set_frequency_kilohertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if amount < 1.0 || amount > 6000000000.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported amount of kHz. Must be 0.00001-60000.0."));
    }

    command = format!("{}{}{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_ARG_SEPARATOR,
        WRITE_FREQUENCY_COMMAND_UNIT_KILOHERTZ,
        COMMAND_END,
    );
    
    println!("\nSetting frequency in kHz: ch{}={}:\n{}", chan, amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_frequency_kilohertz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str) -> io::Result<String> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 5 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency kHz\" argument (must be 0.00001-60000.0): {}: too many decimal places (5 max)", amount)));
    }
    
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.00001 && amount <= 60000.0 => {
                    let amount_rounded = ((amount * 100000.0 * 100000.0).round() / 100000.0).round();
                    
                    res = set_frequency_kilohertz(&mut port, chan, amount_rounded);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency kHz\" argument (must be 0.00001-60000.0): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency kHz\" argument (must be 0.00001-60000.0): {}: {}", amount, e)));
        },
    }

    res
}


pub fn set_frequency_megahertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if amount < 1.0 || amount > 6000000000.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported amount of MHz. Must be 0.00000001-60.0."));
    }

    command = format!("{}{}{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_ARG_SEPARATOR,
        WRITE_FREQUENCY_COMMAND_UNIT_MEGAHERTZ,
        COMMAND_END,
    );
    
    println!("\nSetting frequency in MHz: ch{}={}:\n{}", chan, amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_frequency_megahertz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str) -> io::Result<String> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 8 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency MHz\" argument (must be 0.00000001-60.0): {}: too many decimal places (8 max)", amount)));
    }
    
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.00000001 && amount <= 60.0 => {
                    let amount_rounded = ((amount * 100000000.0 * 10000000.0).round() / 10000000.0).round();
                    
                    res = set_frequency_megahertz(&mut port, chan, amount_rounded);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency MHz\" argument (must be 0.00000001-60.0): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set frequency MHz\" argument (must be 0.00000001-60.0): {}: {}", amount, e)));
        },
    }

    res
}


pub fn set_amplitude(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_AMPLITUDE_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_AMPLITUDE_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if amount < 0.0 || amount > 20000.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported amount of volts. Must be 0.000-20.0."));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    println!("\nSetting amplitude in volts: ch{}={}:\n{}", chan, amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_AMPLITUDE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_amplitude_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str) -> io::Result<String> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 3 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set amplitude volts\" argument (must be 0.000-20.0): {}: too many decimal places (3 max)", amount)));
    }
    
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.0 && amount <= 20.0 => {
                    let amount_rounded = ((amount * 1000.0 * 1000.0).round() / 1000.0).round();
                    
                    res = set_amplitude(&mut port, chan, amount_rounded);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set amplitude volts\" argument (must be 0.000-20.0): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set amplitude volts\" argument (must be 0.000-20.0): {}: {}", amount, e)));
        },
    }

    res
}


pub fn set_duty_cycle(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_DUTY_CYCLE_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_DUTY_CYCLE_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if amount < 0.0 || amount > 999.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported duty cycle. Must be 0.0-99.9."));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    println!("\nSetting duty cycle percent: ch{}={}:\n{}", chan, amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_DUTY_CYCLE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_duty_cycle_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str) -> io::Result<String> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 1 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set duty cycle\" argument (must be 0.0-99.9): {}: too many decimal places (1 max)", amount)));
    }
    
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.0 && amount <= 99.9 => {
                    let amount_rounded = ((amount * 10.0 * 10.0).round() / 10.0).round();
                    
                    res = set_duty_cycle(&mut port, chan, amount_rounded);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set duty cycle\" argument (must be 0.0-99.9): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set duty cycle\" argument (must be 0.0-99.9): {}: {}", amount, e)));
        },
    }

    res
}


pub fn set_voltage_offset(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64) -> io::Result<String> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_VOLTAGE_OFFSET_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_VOLTAGE_OFFSET_COMMAND_CH2;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel number. Must be 1 or 2."));
    }

    if amount < 1.0 || amount > 1999.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported voltage offset. Must be -9.99-9.99."));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    println!("\nSetting voltage offset: ch{}={}:\n{}", chan, amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_VOLTAGE_OFFSET_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_voltage_offset_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str) -> io::Result<String> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set voltage offset\" argument (must be -9.99-9.99): {}: too many decimal places (2 max)", amount)));
    }
    
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= -9.99 && amount <= 9.99 => {
                    let amount_rounded = (((1000.0 + amount * 100.0) * 100.0).round() / 100.0).round();
                    
                    res = set_voltage_offset(&mut port, chan, amount_rounded);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set voltage offset\" argument (must be -9.99-9.99): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set voltage offset\" argument (must be -9.99-9.99): {}: {}", amount, e)));
        },
    }

    res
}


pub fn set_phase(port: &mut Box<dyn SerialPort>, amount: f64) -> io::Result<String> {
    let command: String;

    if amount < 0.0 || amount > 3600.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported phase. Must be 0.0-360.0."));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_PHASE_COMMAND,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    println!("\nSetting phase: {}:\n{}", amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_PHASE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_phase_arg(mut port: &mut Box<dyn SerialPort>, amount: &str) -> io::Result<String> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 1 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set phase\" argument (must be 0.0-360.0): {}: too many decimal places (1 max)", amount)));
    }
    
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.0 && amount <= 360.0 => {
                    let amount_rounded = ((amount * 10.0 * 10.0).round() / 10.0).round();
                    
                    res = set_phase(&mut port, amount_rounded);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set phase\" argument (must be 0.0-360.0): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set phase\" argument (must be 0.0-360.0): {}: {}", amount, e)));
        },
    }

    res
}


pub fn set_tracking(port: &mut Box<dyn SerialPort>, track: TrackingArg) -> io::Result<String> {
    let command: String;

    if track > TrackingArg::all() {
        return Err(Error::new(ErrorKind::Other, format!("Unsupported tracking argument. Must be a number 0-{}.\n\n{}", TrackingArg::all().to_str_val(), TRACKING_FEATURES)));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_TRACKING_COMMAND,
        COMMAND_SEPARATOR,
        track,
        COMMAND_END,
    );
    
    println!("\nSetting tracking: {}:\n{}", track.to_names(), command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_TRACKING_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_tracking_arg(mut port: &mut Box<dyn SerialPort>, track: &str) -> io::Result<String> {
    let max_len = 5;
    
    let track_stripped = track.replace(',', "");

    let res: io::Result<String>;

    let mut track_bits = TrackingArg::from_bits(0).unwrap();    

    for (i, c) in track_stripped.chars().enumerate() {
        match c.to_digit(10) {
            Some(c_num) => {
                if track_stripped.len() > max_len {
                    return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}: too many digits (5 max)\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES)));
                }

                if c_num > 1 {
                    return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES)));
                }
                
                if c_num == 1 {
                    track_bits = TrackingArg::from_bits(track_bits.bits() | (1 << i)).unwrap();
                }
            },

            None => {
                let track_vec: Vec<&str> = track.split(",").collect();

                if track_vec.len() > max_len {
                    return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}: too many digits (5 max)\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES)));
                }

                for (i2, word) in track_vec.into_iter().enumerate() {
                    let mut res = TrackingArg::NONE;
                    let mut res_bits = res.bits();

                    let mut return_err = false;

                    track_bits |= TrackingArg::from_bits(*TRACKING_ARG_MAP.get(word.trim()).unwrap_or_else(|| {
                        for (i3, c) in word.chars().enumerate() {
                            let key: String = c.to_string();
                            let val = TRACKING_ARG_MAP.get(&key[..]);

                            match val {
                                Some(val) => {
                                    res |= TrackingArg::from_bits(*val).unwrap_or(TrackingArg::NONE);
                                },

                                None => {
                                    println!("Error: feature name argument at position (word: {}, char: {}) not recognized.", i2, i3);

                                    res |= TrackingArg::NONE;
                                    return_err = true;

                                    return &res_bits;
                                },
                            }
                        }

                        res_bits = res.bits();
                        &res_bits

                    })).unwrap();

                    if return_err {
                        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES)));
                    }
                }
                
                break;
            },
        }

    }

    match track_bits {
        track_bits if track_bits <= TrackingArg::all() => {
            res = set_tracking(&mut port, track_bits);
        },

        _ => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set tracking\" argument (must be 0-{}): {}\n\n{}", TrackingArg::all().to_str_val(), track_bits, TRACKING_FEATURES)));
        },
    }

    res
}


pub fn set_switch_function_panel_main(port: &mut Box<dyn SerialPort>, chan: u64) -> io::Result<String> {
    let command: &'static str;

    if chan < 1 || chan > 2 {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel. Must be 1 or 2."));
    } else if chan == 1 {
        command = WRITE_SWITCH_FUNCTION_PANEL_MAIN_CH1;
    } else {    // if chan == 2
        command = WRITE_SWITCH_FUNCTION_PANEL_MAIN_CH2;
    }
    
    println!("\nSwitching to function panel main ch{} mode:\n{}", chan, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}


pub fn set_switch_function_panel_sys(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_SYS;
    
    println!("\nSwitching function panel to system settings mode:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}


pub fn set_switch_function_panel_measurement(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_MEASUREMENT;
    
    println!("\nSwitching function panel to measurement mode:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

// Measurement starting - counting, sweep, frequency, pulse, burst stopping.
pub fn set_measurement_starting(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_EXTENDED_FUNCTION_MEASUREMENT_STARTING;
    
    println!("\nMeasurement starting - counting, sweep, frequency, pulse, burst stopping:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}


pub fn set_switch_function_panel_counting(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_COUNTING;
    
    println!("\nSwitching function panel to counting mode:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

// Counting starting.
pub fn set_counting_starting(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_EXTENDED_FUNCTION_COUNTING_STARTING;
    
    println!("\nCounting starting:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}


pub fn set_switch_function_panel_sweep(port: &mut Box<dyn SerialPort>, chan: u64) -> io::Result<String> {
    let command: &'static str;

    if chan < 1 || chan > 2 {
        return Err(Error::new(ErrorKind::Other, "Unsupported channel. Must be 1 or 2."));
    } else if chan == 1 {
        command = WRITE_SWITCH_FUNCTION_PANEL_SWEEP_CH1;
    } else {    // if chan == 2
        command = WRITE_SWITCH_FUNCTION_PANEL_SWEEP_CH2;
    }
    
    println!("\nSwitching to function panel sweep ch{} mode:\n{}", chan, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

// Sweep starting.
pub fn set_sweep_starting(port: &mut Box<dyn SerialPort>, chan: u64) -> io::Result<String> {
    set_switch_function_panel_sweep(port, chan)?;
    
    let command: &'static str = WRITE_EXTENDED_FUNCTION_SWEEP_STARTING;
    
    println!("\nSweep starting:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}


pub fn set_switch_function_panel_pulse(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_PULSE;
    
    println!("\nSwitching function panel to pulse mode:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

// Pulse starting.
pub fn set_pulse_starting(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_EXTENDED_FUNCTION_PULSE_STARTING;
    
    println!("\nPulse starting:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}


pub fn set_switch_function_panel_bursting(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_BURST;
    
    println!("\nSwitching function panel to bursting mode:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

// Bursting starting.
pub fn set_bursting_starting(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_EXTENDED_FUNCTION_BURSTING_STARTING;
    
    println!("\nBursting starting:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

// set measurement coupling to AC.
pub fn set_measurement_coupling_ac(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_MEASUREMENT_COUPLING_AC;
    
    println!("\nSetting measurement coupling to AC:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_COUPLING_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

// set measurement coupling to DC.
pub fn set_measurement_coupling_dc(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_MEASUREMENT_COUPLING_DC;
    
    println!("\nSetting measurement coupling to DC:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_COUPLING_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}


pub fn set_measurement_gate_time(port: &mut Box<dyn SerialPort>, amount: f64) -> io::Result<String> {
    let command: String;

    if amount < 1.0 || amount > 1000.0 {
        return Err(Error::new(ErrorKind::Other, "Unsupported measurement gate time. Must be 0.01-10.0."));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_MEASUREMENT_GATE_TIME_COMMAND,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    println!("\nSetting measurement gate time: {}:\n{}", amount, command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_GATE_TIME_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

pub fn match_set_measurement_gate_time_arg(mut port: &mut Box<dyn SerialPort>, amount: &str) -> io::Result<String> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set measurement gate time\" argument (must be 0.01-10.0): {}: too many decimal places (2 max)", amount)));
    }
    
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 10.0 => {
                    let amount_rounded = ((amount * 100.0 * 100.0).round() / 100.0).round();
                    
                    res = set_measurement_gate_time(&mut port, amount_rounded);
                },

                _ => {
                    res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set measurement gate time\" argument (must be 0.01-10.0): {}", amount)));
                },
            }
        },

        Err(e) => {
            res = Err(Error::new(ErrorKind::Other, format!("unsupported value passed to \"set measurement gate time\" argument (must be 0.01-10.0): {}: {}", amount, e)));
        },
    }

    res
}


// set measurement mode to count frequency.
pub fn set_measurement_mode_count_frequency(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_MEASUREMENT_MODE_COUNT_FREQUENCY;
    
    println!("\nSetting measurement mode to count frequency:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}

// set measurement mode to counting period.
pub fn set_measurement_mode_counting_period(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_MEASUREMENT_MODE_COUNTING_PERIOD;
    
    println!("\nSetting measurement mode to counting period:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}


// set measurement count clear.
pub fn set_measurement_count_clear(port: &mut Box<dyn SerialPort>) -> io::Result<String> {
    let command: &'static str = WRITE_MEASUREMENT_COUNT_CLEAR;
    
    println!("\nSetting measurement count clear:\n{}", command);

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_COUNT_CLEAR_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    println!("Response:");
    println!("{}", res);

    thread::sleep(Duration::from_millis(COMMAND_DELAY_MS));

    Ok(res.to_string())
}
