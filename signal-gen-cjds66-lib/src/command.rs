extern crate serial;

use std::io;
use std::io::{Error, ErrorKind};
use std::thread;
use std::time::{Duration};
// use crate::protocol;
// #[macro_use]
use crate::protocol::*;

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

    if amount < 1.0 || amount > 8000000000.00 {
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
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 80000000.00 => {
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

    if amount < 1.0 || amount > 8000000000.00 {
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
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 80000000.00 => {
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

    if amount < 1.0 || amount > 8000000000.00 {
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
    let res: io::Result<String>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 60000000.00 => {
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
