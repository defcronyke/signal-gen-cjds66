extern crate serial;

use crate::protocol::*;

use std::io::prelude::*;
use serial::prelude::*;
use std::str;

use clap::{Error, ErrorKind};

pub fn read_machine_model(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    if verbose > 0 {
        println!("\nRequesting machine model number:\n{}", READ_MACHINE_MODEL);
    }

    let inbuf: Vec<u8> = READ_MACHINE_MODEL.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..READ_MACHINE_MODEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    } else {
        let res2: Vec<&str> = res.split("=").collect();

        if res2.len() < 2 {
            return Err(Error::with_description(&format!("unexpected response from device: missing separator ({}): {}", COMMAND_SEPARATOR, res), ErrorKind::ValueValidation));
        }

        if res2[1].len() < 4 {
            return Err(Error::with_description(&format!("unexpected response from device: missing terminator ({}): {}", COMMAND_END, res), ErrorKind::ValueValidation));
        }

        let res3: &str = &res2[1][0..res2[1].len()-3];

        println!("model:\t{}", res3);
    }

    Ok(res.to_string())
}

pub fn read_machine_number(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    if verbose > 0 {
        println!("\nRequesting machine serial number:\n{}", READ_MACHINE_NUMBER);
    }

    let inbuf: Vec<u8> = READ_MACHINE_NUMBER.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..READ_MACHINE_NUMBER_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    } else {
        let res2: Vec<&str> = res.split("=").collect();

        if res2.len() < 2 {
            return Err(Error::with_description(&format!("unexpected response from device: missing separator ({}): {}", COMMAND_SEPARATOR, res), ErrorKind::ValueValidation));
        }

        if res2[1].len() < 4 {
            return Err(Error::with_description(&format!("unexpected response from device: missing terminator ({}): {}", COMMAND_END, res), ErrorKind::ValueValidation));
        }

        let res3: &str = &res2[1][0..res2[1].len()-3];

        println!("serial:\t{}", res3);
    }

    Ok(res.to_string())
}


pub fn set_channel_output(port: &mut Box<dyn SerialPort>, ch1: bool, ch2: bool, verbose: u64) -> Result<String, clap::Error> {
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
        return Err(Error::with_description("unsupported input condition", ErrorKind::ArgumentConflict));
    }
    
    if verbose > 0 {
        println!("\nSetting channel output: ch1={} and ch2={}:\n{}", ch1, ch2, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_CHANNEL_OUTPUT_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_channel_output_arg(mut port: &mut Box<dyn SerialPort>, sco: &str, verbose: u64) -> Result<String, clap::Error> {
    let res: Result<String, clap::Error>;
    
    match sco {
        "1,1" | "11" | "on,on" | "1" | "on" => {
            res = set_channel_output(&mut port, true, true, verbose);
        },
        
        "0,0" | "00" | "off,off" | "0" | "off" => {
            res = set_channel_output(&mut port, false, false, verbose);
        },

        "1,0" | "10" | "on,off" => {
            res = set_channel_output(&mut port, true, false, verbose);
        },

        "0,1" | "01" | "off,on" => {
            res = set_channel_output(&mut port, false, true, verbose);
        },

        _ => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"-o\" argument: {}", sco), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_waveform_preset(port: &mut Box<dyn SerialPort>, chan: u64, preset: u64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_WAVEFORM_PRESET_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_WAVEFORM_PRESET_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if preset > 16 {
        return Err(Error::with_description("Unsupported waveform preset number. Must be 0-16.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        preset,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting waveform preset: ch{}={}:\n{}", chan, preset, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_WAVEFORM_PRESET_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

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
pub fn match_set_waveform_preset_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, preset: &str, verbose: u64) -> Result<String, clap::Error> {
    let res: Result<String, clap::Error>;
    
    match preset.parse::<u64>() {
        Ok(preset) => {
            match preset {
                0..=16 => {
                    res = set_waveform_preset(&mut port, chan, preset, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set waveform\" argument (must be 0-16): {}", preset), ErrorKind::InvalidValue));
                },
            }
        },

        Err(_e) => {
            match preset {
                "sine" | "sin" => {
                    res = set_waveform_preset(&mut port, chan, 0, verbose);
                },

                "square" | "sq" => {
                    res = set_waveform_preset(&mut port, chan, 1, verbose);
                },

                "pulse" | "pul" => {
                    res = set_waveform_preset(&mut port, chan, 2, verbose);
                },

                "triangle" | "tri" => {
                    res = set_waveform_preset(&mut port, chan, 3, verbose);
                },

                "partialsine" | "partial-sine" | "parsine" | "par-sine" | "parsin" | "par-sin" | "psine" | "p-sine" | "psin" | "p-sin" => {
                    res = set_waveform_preset(&mut port, chan, 4, verbose);
                },

                "cmos" | "cm" => {
                    res = set_waveform_preset(&mut port, chan, 5, verbose);
                },

                "dc" => {
                    res = set_waveform_preset(&mut port, chan, 6, verbose);
                },

                "halfwave" | "half-wave" | "hw" | "h-w" => {
                    res = set_waveform_preset(&mut port, chan, 7, verbose);
                },

                "fullwave" | "full-wave" | "fw" | "f-w" => {
                    res = set_waveform_preset(&mut port, chan, 8, verbose);
                },

                "pos-ladder" | "posladder" | "pos-lad" | "poslad" | "positive-ladder" | "positiveladder" | "pl" => {
                    res = set_waveform_preset(&mut port, chan, 9, verbose);
                },

                "neg-ladder" | "negladder" | "neg-lad" | "neglad" | "negative-ladder" | "negativeladder" | "nl" => {
                    res = set_waveform_preset(&mut port, chan, 10, verbose);
                },

                "noise" | "nois" | "noi" | "no" | "n" => {
                    res = set_waveform_preset(&mut port, chan, 11, verbose);
                },

                "exp-rise" | "exprise" | "e-r" | "er" | "e-rise" | "erise" | "e-ris" | "eris" => {
                    res = set_waveform_preset(&mut port, chan, 12, verbose);
                },

                "exp-decay" | "expdecay" | "e-d" | "ed" | "e-decay" | "edecay" | "e-dec" | "edec" => {
                    res = set_waveform_preset(&mut port, chan, 13, verbose);
                },

                "multi-tone" | "multitone" | "m-t" | "mt" | "m-tone" | "mtone" => {
                    res = set_waveform_preset(&mut port, chan, 14, verbose);
                },

                "sinc" | "sc" => {
                    res = set_waveform_preset(&mut port, chan, 15, verbose);
                },

                "lorenz" | "loren" | "lor" | "lz" => {
                    res = set_waveform_preset(&mut port, chan, 16, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set waveform\" argument (must be 0-16): {}", preset), ErrorKind::InvalidValue));
                },
            }
        },
    }

    res
}


pub fn set_waveform_arbitrary(port: &mut Box<dyn SerialPort>, chan: u64, preset: u64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_WAVEFORM_PRESET_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_WAVEFORM_PRESET_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if preset < 1 || preset > 60 {
        return Err(Error::with_description("Unsupported waveform preset number. Must be 1-60.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        preset + 100,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting waveform preset: ch{}={}:\n{}", chan, preset, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_WAVEFORM_PRESET_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_waveform_arbitrary_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, preset: &str, verbose: u64) -> Result<String, clap::Error> {
    let res: Result<String, clap::Error>;
    
    match preset.parse::<u64>() {
        Ok(preset) => {
            match preset {
                1..=60 => {
                    res = set_waveform_arbitrary(&mut port, chan, preset, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set arbitrary waveform\" argument (must be 1-60): {}", preset), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set arbitrary waveform\" argument (must be 1-60): {}: {}", preset, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_frequency_microhertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if amount < 1.0 || amount > 8000000000.0 {
        return Err(Error::with_description("Unsupported amount of uHz. Must be 0.01-80000000.0.", ErrorKind::InvalidValue));
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
    
    if verbose > 0 {
        println!("\nSetting frequency in uHz: ch{}={}:\n{}", chan, amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_frequency_microherz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str, verbose: u64) -> Result<String, clap::Error> {    
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set frequency uHz\" argument (must be 0.01-80000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
    }

    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 80000000.0 => {
                    res = set_frequency_microhertz(&mut port, chan, amount * 100.0, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency uHz\" argument (must be 0.01-80000000.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency uHz\" argument (must be 0.01-80000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_frequency_millihertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if amount < 1.0 || amount > 8000000000.0 {
        return Err(Error::with_description("Unsupported amount of mHz. Must be 0.01-80000000.0.", ErrorKind::InvalidValue));
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
    
    if verbose > 0 {
        println!("\nSetting frequency in mHz: ch{}={}:\n{}", chan, amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_frequency_milliherz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-80000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 80000000.0 => {
                    res = set_frequency_millihertz(&mut port, chan, amount * 100.0, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-80000000.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-80000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_frequency_hertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if amount < 1.0 || amount > 6000000000.0 {
        return Err(Error::with_description("Unsupported amount of Hz. Must be 0.01-60000000.0.", ErrorKind::InvalidValue));
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
    
    if verbose > 0 {
        println!("\nSetting frequency in Hz: ch{}={}:\n{}", chan, amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_frequency_hertz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set frequency Hz\" argument (must be 0.01-60000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 60000000.0 => {
                    res = set_frequency_hertz(&mut port, chan, amount * 100.0, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-60000000.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency mHz\" argument (must be 0.01-60000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_frequency_kilohertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if amount < 1.0 || amount > 6000000000.0 {
        return Err(Error::with_description("Unsupported amount of kHz. Must be 0.00001-60000.0.", ErrorKind::InvalidValue));
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
    
    if verbose > 0 {
        println!("\nSetting frequency in kHz: ch{}={}:\n{}", chan, amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_frequency_kilohertz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 5 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set frequency kHz\" argument (must be 0.00001-60000.0): {}: too many decimal places (5 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.00001 && amount <= 60000.0 => {
                    let amount_rounded = ((amount * 100000.0 * 100000.0).round() / 100000.0).round();
                    
                    res = set_frequency_kilohertz(&mut port, chan, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency kHz\" argument (must be 0.00001-60000.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency kHz\" argument (must be 0.00001-60000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_frequency_megahertz(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_FREQUENCY_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if amount < 1.0 || amount > 6000000000.0 {
        return Err(Error::with_description("Unsupported amount of MHz. Must be 0.00000001-60.0.", ErrorKind::InvalidValue));
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
    
    if verbose > 0 {
        println!("\nSetting frequency in MHz: ch{}={}:\n{}", chan, amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_frequency_megahertz_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 8 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set frequency MHz\" argument (must be 0.00000001-60.0): {}: too many decimal places (8 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.00000001 && amount <= 60.0 => {
                    let amount_rounded = ((amount * 100000000.0 * 10000000.0).round() / 10000000.0).round();
                    
                    res = set_frequency_megahertz(&mut port, chan, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency MHz\" argument (must be 0.00000001-60.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set frequency MHz\" argument (must be 0.00000001-60.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_amplitude(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_AMPLITUDE_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_AMPLITUDE_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if amount < 0.0 || amount > 20000.0 {
        return Err(Error::with_description("Unsupported amount of volts. Must be 0.000-20.0.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting amplitude in volts: ch{}={}:\n{}", chan, amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_AMPLITUDE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_amplitude_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 3 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set amplitude volts\" argument (must be 0.000-20.0): {}: too many decimal places (3 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.0 && amount <= 20.0 => {
                    let amount_rounded = ((amount * 1000.0 * 1000.0).round() / 1000.0).round();
                    
                    res = set_amplitude(&mut port, chan, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set amplitude volts\" argument (must be 0.000-20.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set amplitude volts\" argument (must be 0.000-20.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_duty_cycle(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_DUTY_CYCLE_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_DUTY_CYCLE_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if amount < 0.0 || amount > 999.0 {
        return Err(Error::with_description("Unsupported duty cycle. Must be 0.0-99.9.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting duty cycle percent: ch{}={}:\n{}", chan, amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_DUTY_CYCLE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_duty_cycle_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 1 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set duty cycle\" argument (must be 0.0-99.9): {}: too many decimal places (1 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.0 && amount <= 99.9 => {
                    let amount_rounded = ((amount * 10.0 * 10.0).round() / 10.0).round();
                    
                    res = set_duty_cycle(&mut port, chan, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set duty cycle\" argument (must be 0.0-99.9): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set duty cycle\" argument (must be 0.0-99.9): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_voltage_offset(port: &mut Box<dyn SerialPort>, chan: u64, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;
    let chan_out: &str;

    if chan == 1 {
        chan_out = WRITE_VOLTAGE_OFFSET_COMMAND_CH1;
    } else if chan == 2 {
        chan_out = WRITE_VOLTAGE_OFFSET_COMMAND_CH2;
    } else {
        return Err(Error::with_description("Unsupported channel number. Must be 1 or 2.", ErrorKind::InvalidValue));
    }

    if amount < 1.0 || amount > 1999.0 {
        return Err(Error::with_description("Unsupported voltage offset. Must be -9.99-9.99.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        chan_out,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting voltage offset: ch{}={}:\n{}", chan, amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_VOLTAGE_OFFSET_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_voltage_offset_arg(mut port: &mut Box<dyn SerialPort>, chan: u64, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set voltage offset\" argument (must be -9.99-9.99): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= -9.99 && amount <= 9.99 => {
                    let amount_rounded = (((1000.0 + amount * 100.0) * 100.0).round() / 100.0).round();
                    
                    res = set_voltage_offset(&mut port, chan, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set voltage offset\" argument (must be -9.99-9.99): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set voltage offset\" argument (must be -9.99-9.99): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_phase(port: &mut Box<dyn SerialPort>, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;

    if amount < 0.0 || amount > 3600.0 {
        return Err(Error::with_description("Unsupported phase. Must be 0.0-360.0.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_PHASE_COMMAND,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting phase: {}:\n{}", amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_PHASE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_phase_arg(mut port: &mut Box<dyn SerialPort>, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 1 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set phase\" argument (must be 0.0-360.0): {}: too many decimal places (1 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.0 && amount <= 360.0 => {
                    let amount_rounded = ((amount * 10.0 * 10.0).round() / 10.0).round();
                    
                    res = set_phase(&mut port, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set phase\" argument (must be 0.0-360.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set phase\" argument (must be 0.0-360.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_tracking(port: &mut Box<dyn SerialPort>, track: TrackingArg, verbose: u64) -> Result<String, clap::Error> {
    let command: String;

    if track > TrackingArg::all() {
        return Err(Error::with_description(&format!("Unsupported tracking argument. Must be a number 0-{}.\n\n{}", TrackingArg::all().to_str_val(), TRACKING_FEATURES), ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_TRACKING_COMMAND,
        COMMAND_SEPARATOR,
        track,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting tracking: {}:\n{}", track.to_names(), command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_TRACKING_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_tracking_arg(mut port: &mut Box<dyn SerialPort>, track: &str, verbose: u64) -> Result<String, clap::Error> {
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
            },

            None => {
                let track_vec: Vec<&str> = track.split(",").collect();

                if track_vec.len() > max_len {
                    return Err(Error::with_description(&format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}: too many digits (5 max)\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES), ErrorKind::InvalidValue));
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
                        return Err(Error::with_description(&format!("unsupported value passed to \"set tracking\" argument (must be a set of zeros and ones in the range 0-{}): {}\n\n{}", TrackingArg::all().to_str_val(), track, TRACKING_FEATURES), ErrorKind::InvalidValue));
                    }
                }
                
                break;
            },
        }

    }

    match track_bits {
        track_bits if track_bits <= TrackingArg::all() => {
            res = set_tracking(&mut port, track_bits, verbose);
        },

        _ => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set tracking\" argument (must be 0-{}): {}\n\n{}", TrackingArg::all().to_str_val(), track_bits, TRACKING_FEATURES), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_switch_function_panel_main(port: &mut Box<dyn SerialPort>, chan: u64, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str;

    if chan < 1 || chan > 2 {
        return Err(Error::with_description("Unsupported channel. Must be 1 or 2.", ErrorKind::InvalidValue));
    } else if chan == 1 {
        command = WRITE_SWITCH_FUNCTION_PANEL_MAIN_CH1;
    } else {    // if chan == 2
        command = WRITE_SWITCH_FUNCTION_PANEL_MAIN_CH2;
    }
    
    if verbose > 0 {
        println!("\nSwitching to function panel main ch{} mode:\n{}", chan, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_switch_function_panel_sys(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_SYS;
    
    if verbose > 0 {
        println!("\nSwitching function panel to system settings mode:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_switch_function_panel_measurement(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_MEASUREMENT;
    
    if verbose > 0 {
        println!("\nSwitching function panel to measurement mode:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

// Measurement starting - counting, sweep, frequency, pulse, burst stopping.
pub fn set_measurement_starting(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_EXTENDED_FUNCTION_MEASUREMENT_STARTING;
    
    if verbose > 0 {
        println!("\nMeasurement starting - counting, sweep, frequency, pulse, burst stopping:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_switch_function_panel_counting(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_COUNTING;
    
    if verbose > 0 {
        println!("\nSwitching function panel to counting mode:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

// Counting starting.
pub fn set_counting_starting(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_EXTENDED_FUNCTION_COUNTING_STARTING;
    
    if verbose > 0 {
        println!("\nCounting starting:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_switch_function_panel_sweep(port: &mut Box<dyn SerialPort>, chan: u64, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str;

    if chan < 1 || chan > 2 {
        return Err(Error::with_description("Unsupported channel. Must be 1 or 2.", ErrorKind::InvalidValue));
    } else if chan == 1 {
        command = WRITE_SWITCH_FUNCTION_PANEL_SWEEP_CH1;
    } else {    // if chan == 2
        command = WRITE_SWITCH_FUNCTION_PANEL_SWEEP_CH2;
    }
    
    if verbose > 0 {
        println!("\nSwitching to function panel sweep ch{} mode:\n{}", chan, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

// Sweep starting.
pub fn set_sweep_starting(port: &mut Box<dyn SerialPort>, chan: u64, verbose: u64) -> Result<String, clap::Error> {
    set_switch_function_panel_sweep(port, chan, verbose)?;
    
    let command: &'static str = WRITE_EXTENDED_FUNCTION_SWEEP_STARTING;
    
    if verbose > 0 {
        println!("\nSweep starting:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_switch_function_panel_pulse(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_PULSE;
    
    if verbose > 0 {
        println!("\nSwitching function panel to pulse mode:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

// Pulse starting.
pub fn set_pulse_starting(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_EXTENDED_FUNCTION_PULSE_STARTING;
    
    if verbose > 0 {
        println!("\nPulse starting:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_switch_function_panel_bursting(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_SWITCH_FUNCTION_PANEL_BURST;
    
    if verbose > 0 {
        println!("\nSwitching function panel to bursting mode:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWITCH_FUNCTION_PANEL_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

// Bursting starting.
pub fn set_bursting_starting(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_EXTENDED_FUNCTION_BURSTING_STARTING;
    
    if verbose > 0 {
        println!("\nBursting starting:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_EXTENDED_FUNCTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

// set measurement coupling to AC.
pub fn set_measurement_coupling_ac(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_MEASUREMENT_COUPLING_AC;
    
    if verbose > 0 {
        println!("\nSetting measurement coupling to AC:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_COUPLING_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

// set measurement coupling to DC.
pub fn set_measurement_coupling_dc(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_MEASUREMENT_COUPLING_DC;
    
    if verbose > 0 {
        println!("\nSetting measurement coupling to DC:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_COUPLING_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_measurement_gate_time(port: &mut Box<dyn SerialPort>, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;

    if amount < 1.0 || amount > 1000.0 {
        return Err(Error::with_description("Unsupported measurement gate time. Must be 0.01-10.0.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_MEASUREMENT_GATE_TIME_COMMAND,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting measurement gate time: {}:\n{}", amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_GATE_TIME_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_measurement_gate_time_arg(mut port: &mut Box<dyn SerialPort>, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set measurement gate time\" argument (must be 0.01-10.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 10.0 => {
                    let amount_rounded = ((amount * 100.0 * 100.0).round() / 100.0).round();
                    
                    res = set_measurement_gate_time(&mut port, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set measurement gate time\" argument (must be 0.01-10.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set measurement gate time\" argument (must be 0.01-10.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


// set measurement mode to count frequency.
pub fn set_measurement_mode_count_frequency(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_MEASUREMENT_MODE_COUNT_FREQUENCY;
    
    if verbose > 0 {
        println!("\nSetting measurement mode to count frequency:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

// set measurement mode to counting period.
pub fn set_measurement_mode_counting_period(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_MEASUREMENT_MODE_COUNTING_PERIOD;
    
    if verbose > 0 {
        println!("\nSetting measurement mode to counting period:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


// set measurement count clear.
pub fn set_measurement_count_clear(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command: &'static str = WRITE_MEASUREMENT_COUNT_CLEAR;
    
    if verbose > 0 {
        println!("\nSetting measurement count clear:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_MEASUREMENT_COUNT_CLEAR_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_burst_pulse_number(port: &mut Box<dyn SerialPort>, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;

    if amount < 1.0 || amount > 1048575.0 {
        return Err(Error::with_description("Unsupported burst pulse number. Must be 1-1048575.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_BURST_PULSE_NUMBER_COMMAND,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting burst pulse number: {}:\n{}", amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_BURST_PULSE_NUMBER_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_burst_pulse_number_arg(mut port: &mut Box<dyn SerialPort>, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1] != "0" {
        return Err(Error::with_description(&format!("unsupported value passed to \"set burst pulse number\" argument (must be 1-1048575): {}: too many decimal places (0 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 1.0 && amount <= 1048575.0 => {
                    res = set_burst_pulse_number(&mut port, amount, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set burst pulse number\" argument (must be 1-1048575): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set burst pulse number\" argument (must be 1-1048575): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_burst_pulse_once(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_BURST_PULSE_ONCE;
    
    if verbose > 0 {
        println!("\nBurst pulse once:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_BURST_PULSE_ONCE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_burst_mode_manual_trigger(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_BURST_MODE_MANUAL_TRIGGER;
    
    if verbose > 0 {
        println!("\nSetting burst mode to manual trigger:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_BURST_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn set_burst_mode_ch2_burst(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_BURST_MODE_CH2_BURST;
    
    if verbose > 0 {
        println!("\nSetting burst mode to CH2 burst:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_BURST_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn set_burst_mode_external_burst_ac(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_BURST_MODE_EXTERNAL_BURST_AC;
    
    if verbose > 0 {
        println!("\nSetting burst mode to external burst AC:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_BURST_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn set_burst_mode_external_burst_dc(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_BURST_MODE_EXTERNAL_BURST_DC;
    
    if verbose > 0 {
        println!("\nSetting burst mode to external burst DC:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_BURST_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_sweep_starting_frequency(port: &mut Box<dyn SerialPort>, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;

    if amount < 1.0 || amount > 6000000000.0 {
        return Err(Error::with_description("Unsupported sweep starting frequency. Must be 0.01-60000000.0.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_SWEEP_STARTING_FREQUENCY_COMMAND,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting sweep starting frequency: {}:\n{}", amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWEEP_STARTING_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_sweep_starting_frequency_arg(mut port: &mut Box<dyn SerialPort>, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set sweep starting frequency\" argument (must be 0.01-60000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 60000000.0 => {
                    let amount_rounded = ((amount * 100.0 * 10.0).round() / 10.0).round();
                    
                    res = set_sweep_starting_frequency(&mut port, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep starting frequency\" argument (must be 0.01-60000000.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep starting frequency\" argument (must be 0.01-60000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_sweep_termination_frequency(port: &mut Box<dyn SerialPort>, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;

    if amount < 1.0 || amount > 6000000000.0 {
        return Err(Error::with_description("Unsupported sweep termination frequency. Must be 0.01-60000000.0.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_SWEEP_TERMINATION_FREQUENCY_COMMAND,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting sweep termination frequency: {}:\n{}", amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWEEP_TERMINATION_FREQUENCY_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_sweep_termination_frequency_arg(mut port: &mut Box<dyn SerialPort>, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 2 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set sweep termination frequency\" argument (must be 0.01-60000000.0): {}: too many decimal places (2 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.01 && amount <= 60000000.0 => {
                    let amount_rounded = ((amount * 100.0 * 10.0).round() / 10.0).round();
                    
                    res = set_sweep_termination_frequency(&mut port, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep termination frequency\" argument (must be 0.01-60000000.0): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep termination frequency\" argument (must be 0.01-60000000.0): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_sweep_time(port: &mut Box<dyn SerialPort>, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;

    if amount < 1.0 || amount > 6000000000.0 {
        return Err(Error::with_description("Unsupported sweep time. Must be 0.1-999.9.", ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_SWEEP_TIME_COMMAND,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting sweep time: {}:\n{}", amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWEEP_TIME_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_sweep_time_arg(mut port: &mut Box<dyn SerialPort>, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 && amount_parts[1].len() > 1 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set sweep time\" argument (must be 0.1-999.9): {}: too many decimal places (1 max)", amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= 0.1 && amount <= 999.9 => {
                    let amount_rounded = (amount * 10.0).round();
                    
                    res = set_sweep_time(&mut port, amount_rounded, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep time\" argument (must be 0.1-999.9): {}", amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set sweep time\" argument (must be 0.1-999.9): {}: {}", amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


pub fn set_sweep_direction_normal(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_SWEEP_DIRECTION_NORMAL;
    
    if verbose > 0 {
        println!("\nSetting sweep direction to normal:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWEEP_DIRECTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn set_sweep_direction_reverse(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_SWEEP_DIRECTION_REVERSE;
    
    if verbose > 0 {
        println!("\nSetting sweep direction to reverse:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWEEP_DIRECTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn set_sweep_direction_round_trip(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_SWEEP_DIRECTION_ROUND_TRIP;
    
    if verbose > 0 {
        println!("\nSetting sweep direction to round trip:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWEEP_DIRECTION_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


pub fn set_sweep_mode_linear(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_SWEEP_MODE_LINEAR;
    
    if verbose > 0 {
        println!("\nSetting sweep mode to linear:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWEEP_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn set_sweep_mode_logarithm(port: &mut Box<dyn SerialPort>, verbose: u64) -> Result<String, clap::Error> {
    let command = WRITE_SWEEP_MODE_LOGARITHM;
    
    if verbose > 0 {
        println!("\nSetting sweep mode to logarithm:\n{}", command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_SWEEP_MODE_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}


// Set the modulation pulse width. It is in nanosecond units unless 
// the microseconds parameter is true.
// 
// IMPORTANT NOTE: There seems to be no option on the device's physical 
// controls to switch between nanosecond and microsecond units, but if 
// you specify a value in microseconds, the device will switch to 
// microsecond mode, and all future values to this command entered through
// the physical device interface will be interpreted as microsecond units 
// until you turn off the device, or set a nanosecond value using this 
// serial-interface-only command. If you save the device state while in 
// microseconds mode, this could be a problem, because then you need to 
// use this serial program to get back to the default nanoseconds mode.
pub fn set_pulse_width(port: &mut Box<dyn SerialPort>, amount: f64, microseconds: bool, verbose: u64) -> Result<String, clap::Error> {
    let units: &'static str;
    let arg_min: f64;
    let arg_max: f64;
    let command: String;
    
    if microseconds {
        units = "us";
        arg_min = WRITE_PULSE_WIDTH_ARG_MICROSECONDS_MIN;
        arg_max = WRITE_PULSE_WIDTH_ARG_MICROSECONDS_MAX;

        command = format!("{}{}{}{}{}{}{}{}",
            COMMAND_BEGIN,
            COMMAND_WRITE,
            WRITE_PULSE_WIDTH_COMMAND,
            COMMAND_SEPARATOR,
            amount,
            COMMAND_ARG_SEPARATOR,
            WRITE_PULSE_WIDTH_ARG_MICROSECONDS,
            COMMAND_END,
        );

    } else {
        units = "ns";
        arg_min = WRITE_PULSE_WIDTH_ARG_NANOSECONDS_MIN;
        arg_max = WRITE_PULSE_WIDTH_ARG_NANOSECONDS_MAX;

        command = format!("{}{}{}{}{}{}{}{}",
            COMMAND_BEGIN,
            COMMAND_WRITE,
            WRITE_PULSE_WIDTH_COMMAND,
            COMMAND_SEPARATOR,
            amount,
            COMMAND_ARG_SEPARATOR,
            WRITE_PULSE_WIDTH_ARG_NANOSECONDS,
            COMMAND_END,
        );
    }

    if amount < arg_min || amount > arg_max {
        return Err(Error::with_description(&format!("Unsupported pulse width ({}). Must be {}-{}.", units, arg_min, arg_max), ErrorKind::InvalidValue));
    }
    
    if verbose > 0 {
        println!("\nSetting pulse width: {} {}:\n{}", amount, units, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_PULSE_WIDTH_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_pulse_width_arg(mut port: &mut Box<dyn SerialPort>, amount: &str, microseconds: bool, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    let units: &'static str;
    let arg_min: f64;
    let arg_max: f64;

    if microseconds {
        units = "us";
        arg_min = WRITE_PULSE_WIDTH_ARG_MICROSECONDS_MIN;
        arg_max = WRITE_PULSE_WIDTH_ARG_MICROSECONDS_MAX;
    } else {
        units = "ns";
        arg_min = WRITE_PULSE_WIDTH_ARG_NANOSECONDS_MIN;
        arg_max = WRITE_PULSE_WIDTH_ARG_NANOSECONDS_MAX;
    }
    
    if amount_parts.len() > 1 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set pulse width ({})\" argument (must be {}-{}): {}: too many decimal places (0 max)", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= arg_min && amount <= arg_max => {
                    if units == "ns" && amount as i64 % 5 != 0 {
                        return Err(Error::with_description(&format!("unsupported value passed to \"set pulse width ({})\" argument (must be {}-{}): {}: if nanoseconds, it must be a multiple of 5", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
                    }

                    res = set_pulse_width(&mut port, amount, microseconds, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse width ({})\" argument (must be {}-{}): {}", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse width ({})\" argument (must be {}-{}): {}: {}", units, arg_min, arg_max, amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


// Set the modulation pulse period. It is in nanosecond units unless 
// the microseconds parameter is true.
// 
// IMPORTANT NOTE: There seems to be no option on the device's physical 
// controls to switch between nanosecond and microsecond units, but if 
// you specify a value in microseconds, the device will switch to 
// microsecond mode, and all future values to this command entered through
// the physical device interface will be interpreted as microsecond units 
// until you turn off the device, or set a nanosecond value using this 
// serial-interface-only command. If you save the device state while in 
// microseconds mode, this could be a problem, because then you need to 
// use this serial program to get back to the default nanoseconds mode.
pub fn set_pulse_period(port: &mut Box<dyn SerialPort>, amount: f64, microseconds: bool, verbose: u64) -> Result<String, clap::Error> {
    let units: &'static str;
    let arg_min: f64;
    let arg_max: f64;
    let command: String;
    
    if microseconds {
        units = "us";
        arg_min = WRITE_PULSE_PERIOD_ARG_MICROSECONDS_MIN;
        arg_max = WRITE_PULSE_PERIOD_ARG_MICROSECONDS_MAX;

        command = format!("{}{}{}{}{}{}{}{}",
            COMMAND_BEGIN,
            COMMAND_WRITE,
            WRITE_PULSE_PERIOD_COMMAND,
            COMMAND_SEPARATOR,
            amount,
            COMMAND_ARG_SEPARATOR,
            WRITE_PULSE_PERIOD_ARG_MICROSECONDS,
            COMMAND_END,
        );

    } else {
        units = "ns";
        arg_min = WRITE_PULSE_PERIOD_ARG_NANOSECONDS_MIN;
        arg_max = WRITE_PULSE_PERIOD_ARG_NANOSECONDS_MAX;

        command = format!("{}{}{}{}{}{}{}{}",
            COMMAND_BEGIN,
            COMMAND_WRITE,
            WRITE_PULSE_PERIOD_COMMAND,
            COMMAND_SEPARATOR,
            amount,
            COMMAND_ARG_SEPARATOR,
            WRITE_PULSE_PERIOD_ARG_NANOSECONDS,
            COMMAND_END,
        );
    }

    if amount < arg_min || amount > arg_max {
        return Err(Error::with_description(&format!("Unsupported pulse period ({}). Must be {}-{}.", units, arg_min, arg_max), ErrorKind::InvalidValue));
    }
    
    if verbose > 0 {
        println!("\nSetting pulse period: {} {}:\n{}", amount, units, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_PULSE_PERIOD_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_pulse_period_arg(mut port: &mut Box<dyn SerialPort>, amount: &str, microseconds: bool, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    let units: &'static str;
    let arg_min: f64;
    let arg_max: f64;

    if microseconds {
        units = "us";
        arg_min = WRITE_PULSE_PERIOD_ARG_MICROSECONDS_MIN;
        arg_max = WRITE_PULSE_PERIOD_ARG_MICROSECONDS_MAX;
    } else {
        units = "ns";
        arg_min = WRITE_PULSE_PERIOD_ARG_NANOSECONDS_MIN;
        arg_max = WRITE_PULSE_PERIOD_ARG_NANOSECONDS_MAX;
    }
    
    if amount_parts.len() > 1 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set pulse period ({})\" argument (must be {}-{}): {}: too many decimal places (0 max)", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= arg_min && amount <= arg_max => {
                    if units == "ns" && amount as i64 % 5 != 0 {
                        return Err(Error::with_description(&format!("unsupported value passed to \"set pulse period ({})\" argument (must be {}-{}): {}: if nanoseconds, it must be a multiple of 5", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
                    }

                    res = set_pulse_period(&mut port, amount, microseconds, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse period ({})\" argument (must be {}-{}): {}", units, arg_min, arg_max, amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse period ({})\" argument (must be {}-{}): {}: {}", units, arg_min, arg_max, amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}


// Set the pulse offset in percent.
pub fn set_pulse_offset(port: &mut Box<dyn SerialPort>, amount: f64, verbose: u64) -> Result<String, clap::Error> {
    let command: String;

    if amount < WRITE_PULSE_OFFSET_ARG_PERCENT_MIN || amount > WRITE_PULSE_OFFSET_ARG_PERCENT_MAX {
        return Err(Error::with_description(&format!("Unsupported pulse offset. Must be {}-{}.", WRITE_PULSE_OFFSET_ARG_PERCENT_MIN, WRITE_PULSE_OFFSET_ARG_PERCENT_MAX), ErrorKind::InvalidValue));
    }

    command = format!("{}{}{}{}{}{}",
        COMMAND_BEGIN,
        COMMAND_WRITE,
        WRITE_PULSE_OFFSET_COMMAND,
        COMMAND_SEPARATOR,
        amount,
        COMMAND_END,
    );
    
    if verbose > 0 {
        println!("\nSetting pulse offset: {}:\n{}", amount, command);
    }

    let inbuf: Vec<u8> = command.as_bytes().to_vec();
    let mut outbuf: Vec<u8> = (0..WRITE_PULSE_OFFSET_RES_LEN).collect();

    port.write(&inbuf[..])?;
    port.read(&mut outbuf[..])?;

    let res = str::from_utf8(&outbuf).unwrap();

    if verbose > 0 {
        println!("Response:");
        println!("{}", res);
    }

    Ok(res.to_string())
}

pub fn match_set_pulse_offset_arg(mut port: &mut Box<dyn SerialPort>, amount: &str, verbose: u64) -> Result<String, clap::Error> {
    let amount_parts: Vec<&str> = amount.split(".").collect();
    
    if amount_parts.len() > 1 {
        return Err(Error::with_description(&format!("unsupported value passed to \"set pulse offset\" argument (must be {}-{}): {}: too many decimal places (0 max)", WRITE_PULSE_OFFSET_ARG_PERCENT_MIN, WRITE_PULSE_OFFSET_ARG_PERCENT_MAX, amount), ErrorKind::InvalidValue));
    }
    
    let res: Result<String, clap::Error>;
    
    match amount.parse::<f64>() {
        Ok(amount) => {
            match amount {
                _y if amount >= WRITE_PULSE_OFFSET_ARG_PERCENT_MIN && amount <= WRITE_PULSE_OFFSET_ARG_PERCENT_MAX => {
                    res = set_pulse_offset(&mut port, amount, verbose);
                },

                _ => {
                    res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse offset\" argument (must be {}-{}): {}", WRITE_PULSE_OFFSET_ARG_PERCENT_MIN, WRITE_PULSE_OFFSET_ARG_PERCENT_MAX, amount), ErrorKind::InvalidValue));
                },
            }
        },

        Err(e) => {
            res = Err(Error::with_description(&format!("unsupported value passed to \"set pulse offset\" argument (must be {}-{}): {}: {}", WRITE_PULSE_OFFSET_ARG_PERCENT_MIN, WRITE_PULSE_OFFSET_ARG_PERCENT_MAX, amount, e), ErrorKind::InvalidValue));
        },
    }

    res
}
