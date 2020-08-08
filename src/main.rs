extern crate signal_gen_cjds66_lib;
extern crate clap;

use signal_gen_cjds66_lib::serial::open;
use signal_gen_cjds66_lib::command::*;

use clap::{
    Arg, 
    App, 
    values_t,
};

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}

fn real_main() -> i32 {
    let mut app = App::new("signal-gen-cjds66")
        .version("0.0.1\n")
        .author("Jeremy Carter <jeremy@jeremycarter.ca>\n\n")
        .about("An unofficial program to control the CJDS66 60MHz DDS Signal Generator/Counter (hardware by Koolertron).\n\nSee: https://www.koolertron.com/koolertron-upgraded-60mhz-dds-signal-generator-counterhigh-precision-dualchannel-arbitrary-waveform-function-generator-frequency-meter-200msas-60mhz-p-867.html")
        .arg(
            Arg::with_name("devices")
                .short("d")
                .long("device")
                .help("The device(s) to communicate with.")
                .takes_value(true)
                .value_name("PATH")
                .multiple(true)
        )
        .arg(
            Arg::with_name("model")
                .short("m")
                .long("model")
                .help("Get the device's model number.")
        )
        .arg(
            Arg::with_name("serial")
                .short("s")
                .long("serial")
                .help("Get the device's serial number.")
        )
        .arg(
            Arg::with_name("set channel output")
                .short("o")
                .long("set-channel-output")
                .help("Set the output state to on or off for channels 1 and 2.\nFor example, ch1 on, ch 2 off: -o 1,0\n")
                .takes_value(true)
                .value_name("CH1_ON,CH2_ON")
        )
        .arg(
            Arg::with_name("set waveform channel1")
                .short("w")
                .long("w1")
                .help("Set the waveform preset for channel 1. The value must be a number 0-16.\nFor example, sine wave: -w 0\n")
                .takes_value(true)
                .value_name("CH1 PRESET")
        )
        .arg(
            Arg::with_name("set waveform channel2")
                .short("x")
                .long("w2")
                .help("Set the waveform preset for channel 2. The value must be a number 0-16.\nFor example, sine wave: -x 0\n")
                .takes_value(true)
                .value_name("CH2 PRESET")
        )
        .arg(
            Arg::with_name("set arbitrary waveform channel1")
                .short("a")
                .long("a1")
                .help("Set the arbitrary waveform preset for channel 1. The value must be a number 1-60.\nFor example: -a 1\n")
                .takes_value(true)
                .value_name("CH1 ARB PRESET")
        )
        .arg(
            Arg::with_name("set arbitrary waveform channel2")
                .short("b")
                .long("a2")
                .help("Set the arbitrary waveform preset for channel 2. The value must be a number 1-60.\nFor example: -b 1\n")
                .takes_value(true)
                .value_name("CH2 ARB PRESET")
        )
        .arg(
            Arg::with_name("set frequency in uHz channel1")
                .short("u")
                .long("u1")
                .help("Set the waveform frequency for channel 1 in uHz. The value must be a number 0.01-80000000.0.\nFor example: -u 0.01\n")
                .takes_value(true)
                .value_name("CH1 FREQ uHz")
        )
        .arg(
            Arg::with_name("set frequency in uHz channel2")
                .short("v")
                .long("u2")
                .help("Set the waveform frequency for channel 2 in uHz. The value must be a number 0.01-80000000.0.\nFor example: -v 0.01\n")
                .takes_value(true)
                .value_name("CH2 FREQ uHz")
        )
        .arg(
            Arg::with_name("set frequency in mHz channel1")
                .short("i")
                .long("i1")
                .help("Set the waveform frequency for channel 1 in mHz. The value must be a number 0.01-80000000.0.\nFor example: -i 0.01\n")
                .takes_value(true)
                .value_name("CH1 FREQ mHz")
        )
        .arg(
            Arg::with_name("set frequency in mHz channel2")
                .short("j")
                .long("i2")
                .help("Set the waveform frequency for channel 2 in mHz. The value must be a number 0.01-80000000.0.\nFor example: -j 0.01\n")
                .takes_value(true)
                .value_name("CH2 FREQ mHz")
        )
        .arg(
            Arg::with_name("set frequency in Hz channel1")
                .short("e")
                .long("e1")
                .help("Set the waveform frequency for channel 1 in Hz. The value must be a number 0.01-60000000.0.\nFor example: -e 0.01\n")
                .takes_value(true)
                .value_name("CH1 FREQ Hz")
        )
        .arg(
            Arg::with_name("set frequency in Hz channel2")
                .short("f")
                .long("e2")
                .help("Set the waveform frequency for channel 2 in Hz. The value must be a number 0.01-60000000.0.\nFor example: -f 0.01\n")
                .takes_value(true)
                .value_name("CH2 FREQ Hz")
        )
        .arg(
            Arg::with_name("set frequency in kHz channel1")
                .short("k")
                .long("k1")
                .help("Set the waveform frequency for channel 1 in kHz. The value must be a number 0.00001-60000.0.\nFor example: -k 0.01\n")
                .takes_value(true)
                .value_name("CH1 FREQ kHz")
        )
        .arg(
            Arg::with_name("set frequency in kHz channel2")
                .short("l")
                .long("k2")
                .help("Set the waveform frequency for channel 2 in kHz. The value must be a number 0.00001-60000.0.\nFor example: -l 0.01\n")
                .takes_value(true)
                .value_name("CH2 FREQ kHz")
        );

    println!("");
    app.print_long_help().unwrap();

    let matches = app.clone().get_matches();
        
    let devices = values_t!(matches.values_of("devices"), String)
        .unwrap_or_else(|_e| {
            if cfg!(unix) {
                vec!("/dev/ttyUSB0".to_string())
            } else if cfg!(windows) {
                vec!("COM3".to_string())
            } else {
                vec!("/dev/ttyUSB0".to_string())
            }
        });

    println!("\n\nDevice(s) selected: {:?}\n", devices);

    // Iterate over each device and perform commands on each.
    for device in &devices {
        println!("\n\nOpening communication link with device: {}\n", device);

        // Open the device.
        match open(device) {
            // If device was opened successfully.
            Ok(mut port) => {
                // If model number is requested.
                if matches.is_present("model") {
                    read_machine_model(&mut port).unwrap();
                }

                // If serial number is requested.
                if matches.is_present("serial") {
                    read_machine_number(&mut port).unwrap();
                }

                // If set waveform for channel1 is requested.
                if matches.is_present("set waveform channel1") {
                    let preset = matches.value_of("set waveform channel1").unwrap_or_default();
                    
                    match match_set_waveform_preset_arg(&mut port, 1, preset) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set waveform for channel2 is requested.
                if matches.is_present("set waveform channel2") {
                    let preset = matches.value_of("set waveform channel2").unwrap_or_default();
                    
                    match match_set_waveform_preset_arg(&mut port, 2, preset) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set arbitrary waveform for channel1 is requested.
                if matches.is_present("set arbitrary waveform channel1") {
                    let preset = matches.value_of("set arbitrary waveform channel1").unwrap_or_default();
                    
                    match match_set_waveform_arbitrary_arg(&mut port, 1, preset) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set arbitrary waveform for channel2 is requested.
                if matches.is_present("set arbitrary waveform channel2") {
                    let preset = matches.value_of("set arbitrary waveform channel2").unwrap_or_default();
                    
                    match match_set_waveform_arbitrary_arg(&mut port, 2, preset) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel1 in uHz is requested.
                if matches.is_present("set frequency in uHz channel1") {
                    let amount = matches.value_of("set frequency in uHz channel1").unwrap_or_default();
                    
                    match match_set_frequency_microherz_arg(&mut port, 1, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel2 in uHz is requested.
                if matches.is_present("set frequency in uHz channel2") {
                    let amount = matches.value_of("set frequency in uHz channel2").unwrap_or_default();
                    
                    match match_set_frequency_microherz_arg(&mut port, 2, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel1 in mHz is requested.
                if matches.is_present("set frequency in mHz channel1") {
                    let amount = matches.value_of("set frequency in mHz channel1").unwrap_or_default();
                    
                    match match_set_frequency_milliherz_arg(&mut port, 1, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel2 in mHz is requested.
                if matches.is_present("set frequency in mHz channel2") {
                    let amount = matches.value_of("set frequency in mHz channel2").unwrap_or_default();
                    
                    match match_set_frequency_milliherz_arg(&mut port, 2, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel1 in Hz is requested.
                if matches.is_present("set frequency in Hz channel1") {
                    let amount = matches.value_of("set frequency in Hz channel1").unwrap_or_default();
                    
                    match match_set_frequency_hertz_arg(&mut port, 1, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel2 in Hz is requested.
                if matches.is_present("set frequency in Hz channel2") {
                    let amount = matches.value_of("set frequency in Hz channel2").unwrap_or_default();
                    
                    match match_set_frequency_hertz_arg(&mut port, 2, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel1 in kHz is requested.
                if matches.is_present("set frequency in kHz channel1") {
                    let amount = matches.value_of("set frequency in kHz channel1").unwrap_or_default();
                    
                    match match_set_frequency_kilohertz_arg(&mut port, 1, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel2 in kHz is requested.
                if matches.is_present("set frequency in kHz channel2") {
                    let amount = matches.value_of("set frequency in kHz channel2").unwrap_or_default();
                    
                    match match_set_frequency_kilohertz_arg(&mut port, 2, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set channel output is requested.
                if matches.is_present("set channel output") {
                    let sco = matches.value_of("set channel output").unwrap_or_default();
                    
                    match match_set_channel_output_arg(&mut port, sco) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }
            },

            Err(e) => {
                println!("\nError: {}\n", e);
                continue;
            }
        }
    }

    println!("");

    0
}
