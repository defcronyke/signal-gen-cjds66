extern crate signal_gen_cjds66_lib; // For compatibility with Rust pre-2018 versions.
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
                .help("Set the output state to on or off for channels 1 and 2. Ex. -o 1 0")
                .takes_value(true)
                .value_name("CH1 ON, CH2 ON")
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
                
                // If set channel output is requested.
                if matches.is_present("set channel output") {
                    let sco = matches.value_of("set channel output").unwrap_or_default();
                    match sco {
                        "1,1" | "11" | "on,on" | "1" | "on" => {
                            set_channel_output(&mut port, true, true).unwrap();
                        },
                        
                        "0,0" | "00" | "off,off" | "0" | "off" => {
                            set_channel_output(&mut port, false, false).unwrap();
                        },

                        "1,0" | "10" | "on,off" => {
                            set_channel_output(&mut port, true, false).unwrap();
                        },

                        "0,1" | "01" | "off,on" => {
                            set_channel_output(&mut port, false, true).unwrap();
                        },

                        _ => {
                            println!("\nError: unsupported value passed to \"set-channel-output\" argument: {}\n", sco);
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
