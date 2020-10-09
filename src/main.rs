extern crate signal_gen_cjds66_lib;
extern crate clap;

use signal_gen_cjds66_lib::serial::open;
use signal_gen_cjds66_lib::command::*;
use signal_gen_cjds66_lib::protocol::*;

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
    let set_waveform_ch1_help = format!("Set the waveform preset for channel 1. The value must be either the name of the waveform preset (see below), or a number 0-16, for example, sine wave: -w 0\n\nAccepted preset names:\n{}\n", WAVEFORM_PRESET_NAMES);
    let set_tracking_help = format!("Set the tracking mode. The value must be either a set of comma-separated setting names (see below), or a set of zeros and ones in the range of 0-{}, each bit corresponding to a feature you want to toggle tracking on/off for (1 being on and 0 being off). For example: track frequency and amplitude: -T 101\n\n{}\n\nNote that a value of zero (or no value) in the bit position will turn off tracking for the corresponding feature, so to turn tracking off for all features, you can do: -T 0\nYou can also separate the values with commas if you prefer: -T 1,0,1", TrackingArg::all().to_str_val(), TRACKING_FEATURES);

    let app = App::new("signal-gen-cjds66")
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
                .long("output")
                .help("Set the output state to on or off for channels 1 and 2. For example, ch1 on, ch 2 off: -o 1,0\n")
                .takes_value(true)
                .value_name("CH1_ON,CH2_ON")
        )
        .arg(
            Arg::with_name("set waveform channel1")
                .short("w")
                .long("wave-preset-ch1")
                .help(&set_waveform_ch1_help)
                .takes_value(true)
                .value_name("CH1 PRESET")
        )
        .arg(
            Arg::with_name("set waveform channel2")
                .short("x")
                .long("wave-preset-ch2")
                .help("Set the waveform preset for channel 2. The value must be either the name of the waveform preset (see channel1 help item for details), or a number 0-16. For example, sine wave: -x 0\n")
                .takes_value(true)
                .value_name("CH2 PRESET")
        )
        .arg(
            Arg::with_name("set arbitrary waveform channel1")
                .short("a")
                .long("wave-arb-ch1")
                .help("Set the arbitrary waveform preset for channel 1. The value must be a number 1-60. For example: -a 1\n")
                .takes_value(true)
                .value_name("CH1 ARB PRESET")
        )
        .arg(
            Arg::with_name("set arbitrary waveform channel2")
                .short("b")
                .long("wave-arb-ch2")
                .help("Set the arbitrary waveform preset for channel 2. The value must be a number 1-60. For example: -b 1\n")
                .takes_value(true)
                .value_name("CH2 ARB PRESET")
        )
        .arg(
            Arg::with_name("set frequency in uHz channel1")
                .short("u")
                .long("freq-micro-ch1")
                .help("Set the waveform frequency for channel 1 in uHz. The value must be a number 0.01-80000000.0. For example: -u 0.01\n")
                .takes_value(true)
                .value_name("CH1 FREQ uHz")
        )
        .arg(
            Arg::with_name("set frequency in uHz channel2")
                .short("v")
                .long("freq-micro-ch2")
                .help("Set the waveform frequency for channel 2 in uHz. The value must be a number 0.01-80000000.0. For example: -v 0.01\n")
                .takes_value(true)
                .value_name("CH2 FREQ uHz")
        )
        .arg(
            Arg::with_name("set frequency in mHz channel1")
                .short("i")
                .long("freq-milli-ch1")
                .help("Set the waveform frequency for channel 1 in mHz. The value must be a number 0.01-80000000.0. For example: -i 0.01\n")
                .takes_value(true)
                .value_name("CH1 FREQ mHz")
        )
        .arg(
            Arg::with_name("set frequency in mHz channel2")
                .short("j")
                .long("freq-milli-ch2")
                .help("Set the waveform frequency for channel 2 in mHz. The value must be a number 0.01-80000000.0. For example: -j 0.01\n")
                .takes_value(true)
                .value_name("CH2 FREQ mHz")
        )
        .arg(
            Arg::with_name("set frequency in Hz channel1")
                .short("e")
                .long("freq-hertz-ch1")
                .help("Set the waveform frequency for channel 1 in Hz. The value must be a number 0.01-60000000.0. For example: -e 0.01\n")
                .takes_value(true)
                .value_name("CH1 FREQ Hz")
        )
        .arg(
            Arg::with_name("set frequency in Hz channel2")
                .short("f")
                .long("freq-hertz-ch2")
                .help("Set the waveform frequency for channel 2 in Hz. The value must be a number 0.01-60000000.0. For example: -f 0.01\n")
                .takes_value(true)
                .value_name("CH2 FREQ Hz")
        )
        .arg(
            Arg::with_name("set frequency in kHz channel1")
                .short("k")
                .long("freq-kilo-ch1")
                .help("Set the waveform frequency for channel 1 in kHz. The value must be a number 0.00001-60000.0. For example: -k 0.00001\n")
                .takes_value(true)
                .value_name("CH1 FREQ kHz")
        )
        .arg(
            Arg::with_name("set frequency in kHz channel2")
                .short("l")
                .long("freq-kilo-ch2")
                .help("Set the waveform frequency for channel 2 in kHz. The value must be a number 0.00001-60000.0. For example: -l 0.00001\n")
                .takes_value(true)
                .value_name("CH2 FREQ kHz")
        )
        .arg(
            Arg::with_name("set frequency in MHz channel1")
                .short("y")
                .long("freq-mega-ch1")
                .help("Set the waveform frequency for channel 1 in MHz. The value must be a number 0.00000001-60.0. For example: -y 0.00000001\n")
                .takes_value(true)
                .value_name("CH1 FREQ MHz")
        )
        .arg(
            Arg::with_name("set frequency in MHz channel2")
                .short("z")
                .long("freq-mega-ch2")
                .help("Set the waveform frequency for channel 2 in MHz. The value must be a number 0.00000001-60.0. For example: -z 0.00000001\n")
                .takes_value(true)
                .value_name("CH2 FREQ MHz")
        )
        .arg(
            Arg::with_name("set amplitude in volts channel1")
                .short("p")
                .long("ampli-ch1")
                .help("Set the signal amplitude for channel 1 in volts. The value must be a number 0.000-20.0. For example: -p 0.001\n")
                .takes_value(true)
                .value_name("CH1 AMPLI V")
        )
        .arg(
            Arg::with_name("set amplitude in volts channel2")
                .short("q")
                .long("ampli-ch2")
                .help("Set the signal amplitude for channel 2 in volts. The value must be a number 0.000-20.0. For example: -q 0.001\n")
                .takes_value(true)
                .value_name("CH2 AMPLI V")
        )
        .arg(
            Arg::with_name("set duty cycle channel1")
                .short("t")
                .long("duty-ch1")
                .help("Set the duty cycle for channel 1 in percent. The value must be a number 0.0-99.9. For example: -t 40.1\n")
                .takes_value(true)
                .value_name("CH1 DUTY CYCLE")
        )
        .arg(
            Arg::with_name("set duty cycle channel2")
                .short("c")
                .long("duty-ch2")
                .help("Set the duty cycle for channel 2 in percent. The value must be a number 0.0-99.9. For example: -c 40.1\n")
                .takes_value(true)
                .value_name("CH2 DUTY CYCLE")
        )
        .arg(
            Arg::with_name("set voltage offset channel1")
                .short("g")
                .long("offset-ch1")
                .help("Set the voltage offset for channel 1 in volts. The value must be a number -9.99-9.99. For example: -g -1.23\n")
                .takes_value(true)
                .value_name("CH1 VOLT OFFSET")
                .allow_hyphen_values(true)
        )
        .arg(
            Arg::with_name("set voltage offset channel2")
                .short("n")
                .long("offset-ch2")
                .help("Set the voltage offset for channel 2 in volts. The value must be a number -9.99-9.99. For example: -n -1.23\n")
                .takes_value(true)
                .value_name("CH2 VOLT OFFSET")
                .allow_hyphen_values(true)
        )
        .arg(
            Arg::with_name("set phase")
                .short("r")
                .long("phase")
                .help("Set the phase in degrees. The value must be a number 0.0-360.0, and 360.0 wraps around to 0.0. For example: -r 180.7\n")
                .takes_value(true)
                .value_name("PHASE DEG")
        )
        .arg(
            Arg::with_name("set tracking")
                .short("T")
                .long("track")
                .help(&set_tracking_help)
                .takes_value(true)
                .value_name("TRACK FEATURES")
        )
        .arg(
            Arg::with_name("set switch main ch1")
                .short("A")
                .long("main1")
                .help("Switch the function panel to main channel 1 mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set switch main ch2")
                .short("B")
                .long("main2")
                .help("Switch the function panel to main channel 2 mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set switch sys")
                .short("Y")
                .long("sys")
                .help("Switch the function panel to system settings (SYS) mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set switch counting")
                .short("C")
                .long("count")
                .help("Switch the function panel to counting mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set counting starting")
                .short("D")
                .long("start-count")
                .help("Set the extended function to start counting.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set switch sweep ch1")
                .short("V")
                .long("sweep1")
                .help("Switch the function panel to sweep channel 1 mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set switch sweep ch2")
                .short("W")
                .long("sweep2")
                .help("Switch the function panel to sweep channel 2 mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set sweep starting ch1")
                .short("S")
                .long("start-sweep1")
                .help("Set the extended function to start sweep on channel 1.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set sweep starting ch2")
                .short("U")
                .long("start-sweep2")
                .help("Set the extended function to start sweep on channel 2.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set switch pulse")
                .short("P")
                .long("pulse")
                .help("Switch the function panel to pulse mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set pulse starting")
                .short("Q")
                .long("start-pulse")
                .help("Set the extended function to start pulse.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set switch bursting")
                .short("R")
                .long("burst")
                .help("Switch the function panel to bursting mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set bursting starting")
                .short("O")
                .long("start-burst")
                .help("Set the extended function to start bursting.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set switch measurement")
                .short("M")
                .long("measure")
                .help("Switch the function panel to measurement mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set measurement starting")
                .short("N")
                .long("start-measure")
                .help("Set the extended function to start measuring, and to stop counting, sweep, pulse, and bursting.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set measurement coupling ac")
                .long("ac")
                .help("Set the measurement mode coupling option to AC.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set measurement coupling dc")
                .long("dc")
                .help("Set the measurement mode coupling option to DC.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set measurement gate time")
                .long("gt")
                .help("Set the measurement gate time in seconds. The value must be a number 0.01-10.0. For example: --gt 0.01\n")
                .takes_value(true)
                .value_name("GATE TIME")
        )
        .arg(
            Arg::with_name("set measurement mode count frequency")
                .long("cf")
                .help("Set the measurement mode to count frequency.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set measurement mode counting period")
                .long("cp")
                .help("Set the measurement mode to counting period.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set measurement count clear")
                .long("cc")
                .help("Clear the count on measurement mode.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set burst pulse number")
                .long("bn")
                .help("Burst pulse number. Set the number of burst pulses.")
                .takes_value(true)
                .value_name("NUM PULSES")
        )
        .arg(
            Arg::with_name("set burst pulse once")
                .long("b1")
                .help("Burst pulse once.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set burst mode manual trigger")
                .long("bm")
                .help("Set the burst mode to manual trigger.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set burst mode ch2 burst")
                .long("bc")
                .help("Set the burst mode to CH2 burst.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set burst mode external burst ac")
                .long("ba")
                .help("Set the burst mode to external burst AC.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set burst mode external burst dc")
                .long("bd")
                .help("Set the burst mode to external burst DC.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set sweep starting frequency")
                .long("ss")
                .help("Set the sweep starting frequency.")
                .takes_value(true)
                .value_name("START FREQ HZ")
        )
        .arg(
            Arg::with_name("set sweep termination frequency")
                .long("se")
                .help("Set the sweep termination frequency.")
                .takes_value(true)
                .value_name("TERMINATION FREQ HZ")
        )
        .arg(
            Arg::with_name("set sweep time")
                .long("st")
                .help("Set the sweep time.")
                .takes_value(true)
                .value_name("SWEEP TIME SECONDS")
        )
        .arg(
            Arg::with_name("set sweep direction normal")
                .long("sdn")
                .help("Set the sweep direction to normal (rise).")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set sweep direction reverse")
                .long("sdr")
                .help("Set the sweep direction to reverse (fall).")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("set sweep direction round trip")
                .long("sdt")
                .help("Set the sweep direction to round trip (rise and fall).")
                .takes_value(false)
        );

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


                // If set tracking mode is requested.
                if matches.is_present("set tracking") {
                    let track = matches.value_of("set tracking").unwrap_or_default();
                    
                    match match_set_tracking_arg(&mut port, track) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
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


                // If set frequency for channel1 in MHz is requested.
                if matches.is_present("set frequency in MHz channel1") {
                    let amount = matches.value_of("set frequency in MHz channel1").unwrap_or_default();
                    
                    match match_set_frequency_megahertz_arg(&mut port, 1, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel2 in MHz is requested.
                if matches.is_present("set frequency in MHz channel2") {
                    let amount = matches.value_of("set frequency in MHz channel2").unwrap_or_default();
                    
                    match match_set_frequency_megahertz_arg(&mut port, 2, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set amplitude for channel1 in volts is requested.
                if matches.is_present("set amplitude in volts channel1") {
                    let amount = matches.value_of("set amplitude in volts channel1").unwrap_or_default();
                    
                    match match_set_amplitude_arg(&mut port, 1, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set frequency for channel2 in MHz is requested.
                if matches.is_present("set amplitude in volts channel2") {
                    let amount = matches.value_of("set amplitude in volts channel2").unwrap_or_default();
                    
                    match match_set_amplitude_arg(&mut port, 2, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set duty cycle for channel1 in percent is requested.
                if matches.is_present("set duty cycle channel1") {
                    let amount = matches.value_of("set duty cycle channel1").unwrap_or_default();
                    
                    match match_set_duty_cycle_arg(&mut port, 1, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set duty cycle for channel2 in percent is requested.
                if matches.is_present("set duty cycle channel2") {
                    let amount = matches.value_of("set duty cycle channel2").unwrap_or_default();
                    
                    match match_set_duty_cycle_arg(&mut port, 2, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set voltage offset for channel1 in volts is requested.
                if matches.is_present("set voltage offset channel1") {
                    let amount = matches.value_of("set voltage offset channel1").unwrap_or_default();
                    
                    match match_set_voltage_offset_arg(&mut port, 1, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set voltage offset for channel2 in volts is requested.
                if matches.is_present("set voltage offset channel2") {
                    let amount = matches.value_of("set voltage offset channel2").unwrap_or_default();
                    
                    match match_set_voltage_offset_arg(&mut port, 2, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set phase in degrees is requested.
                if matches.is_present("set phase") {
                    let amount = matches.value_of("set phase").unwrap_or_default();
                    
                    match match_set_phase_arg(&mut port, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set switch function panel main ch1 is requested.
                if matches.is_present("set switch main ch1") {
                    match set_switch_function_panel_main(&mut port, 1) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set switch function panel main ch2 is requested.
                if matches.is_present("set switch main ch2") {
                    match set_switch_function_panel_main(&mut port, 2) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set switch function panel system settings is requested.
                if matches.is_present("set switch sys") {
                    match set_switch_function_panel_sys(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set switch function panel counting is requested.
                if matches.is_present("set switch counting") {
                    match set_switch_function_panel_counting(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set counting starting is requested.
                if matches.is_present("set counting starting") {
                    match set_counting_starting(&mut port) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }
                

                // If set switch function panel sweep ch1 is requested.
                if matches.is_present("set switch sweep ch1") {
                    match set_switch_function_panel_sweep(&mut port, 1) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set switch function panel sweep ch2 is requested.
                if matches.is_present("set switch sweep ch2") {
                    match set_switch_function_panel_sweep(&mut port, 2) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set sweep starting ch1 is requested.
                if matches.is_present("set sweep starting ch1") {
                    match set_sweep_starting(&mut port, 1) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set sweep starting ch2 is requested.
                if matches.is_present("set sweep starting ch2") {
                    match set_sweep_starting(&mut port, 2) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set switch function panel pulse is requested.
                if matches.is_present("set switch pulse") {
                    match set_switch_function_panel_pulse(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set pulse starting is requested.
                if matches.is_present("set pulse starting") {
                    match set_pulse_starting(&mut port) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set switch function panel bursting is requested.
                if matches.is_present("set switch bursting") {
                    match set_switch_function_panel_bursting(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set bursting starting is requested.
                if matches.is_present("set bursting starting") {
                    match set_bursting_starting(&mut port) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set switch function panel measurement is requested.
                if matches.is_present("set switch measurement") {
                    match set_switch_function_panel_measurement(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set measurement coupling ac is requested.
                if matches.is_present("set measurement coupling ac") {
                    match set_measurement_coupling_ac(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set measurement coupling dc is requested.
                if matches.is_present("set measurement coupling dc") {
                    match set_measurement_coupling_dc(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set measurement gate time in seconds is requested.
                if matches.is_present("set measurement gate time") {
                    let amount = matches.value_of("set measurement gate time").unwrap_or_default();
                    
                    match match_set_measurement_gate_time_arg(&mut port, amount) {
                        Ok(_res) => {},
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set measurement mode count frequency is requested.
                if matches.is_present("set measurement mode count frequency") {
                    match set_measurement_mode_count_frequency(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set measurement mode counting period is requested.
                if matches.is_present("set measurement mode counting period") {
                    match set_measurement_mode_counting_period(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set measurement count clear is requested.
                if matches.is_present("set measurement count clear") {
                    match set_measurement_count_clear(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set burst pulse number is requested.
                if matches.is_present("set burst pulse number") {
                    let amount = matches.value_of("set burst pulse number").unwrap_or_default();

                    match match_set_burst_pulse_number_arg(&mut port, amount) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set burst pulse once is requested.
                if matches.is_present("set burst pulse once") {
                    match set_burst_pulse_once(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set burst mode manual trigger is requested.
                if matches.is_present("set burst mode manual trigger") {
                    match set_burst_mode_manual_trigger(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set burst mode CH2 burst is requested.
                if matches.is_present("set burst mode ch2 burst") {
                    match set_burst_mode_ch2_burst(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set burst mode external burst AC is requested.
                if matches.is_present("set burst mode external burst ac") {
                    match set_burst_mode_external_burst_ac(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set burst mode external burst DC is requested.
                if matches.is_present("set burst mode external burst dc") {
                    match set_burst_mode_external_burst_dc(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set sweep starting frequency is requested.
                if matches.is_present("set sweep starting frequency") {
                    let amount = matches.value_of("set sweep starting frequency").unwrap_or_default();

                    match match_set_sweep_starting_frequency_arg(&mut port, amount) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set sweep termination frequency is requested.
                if matches.is_present("set sweep termination frequency") {
                    let amount = matches.value_of("set sweep termination frequency").unwrap_or_default();

                    match match_set_sweep_termination_frequency_arg(&mut port, amount) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set sweep time is requested.
                if matches.is_present("set sweep time") {
                    let amount = matches.value_of("set sweep time").unwrap_or_default();

                    match match_set_sweep_time_arg(&mut port, amount) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set sweep direction normal is requested.
                if matches.is_present("set sweep direction normal") {
                    match set_sweep_direction_normal(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set sweep direction reverse is requested.
                if matches.is_present("set sweep direction reverse") {
                    match set_sweep_direction_reverse(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }

                // If set sweep direction round trip is requested.
                if matches.is_present("set sweep direction round trip") {
                    match set_sweep_direction_round_trip(&mut port) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            println!("\nError: {}\n", e);
                        },
                    }
                }


                // If set measurement starting is requested.
                if matches.is_present("set measurement starting") {
                    match set_measurement_starting(&mut port) {
                        Ok(_res) => {
                            match set_channel_output(&mut port, false, false) {
                                Ok(_res) => {},
                                Err(e) => {
                                    println!("\nError: {}\n", e);
                                },
                            }
                        },
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
