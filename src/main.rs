extern crate signal_gen_cjds66_lib;
#[macro_use]
extern crate clap;

use signal_gen_cjds66_lib::serial::open;
use signal_gen_cjds66_lib::command::*;
use signal_gen_cjds66_lib::error;
use signal_gen_cjds66_lib::error::From;

use clap::{
    App,
    values_t,
    ErrorKind,
};

fn main() {
    let res = real_main();

    std::process::exit(error::handle_exit(res)
        .map_or_else(
        |e| {
            e.code
        },
        |code| {
            code
        })
    );
}

fn real_main() -> Result<i32, error::Error> {
    let mut err: Option<error::Error> = None;
    
    let yaml = load_yaml!("../clap.yaml");
    let app = App::from_yaml(yaml);

    let matches = app.clone().get_matches_safe().map_err(|e| {
        error::Error::from_clap_error(e)
    })?;




    /* ----- Command that sets the verbosity 
             level.                          ----- */

    let verbose: u64;

    if matches.occurrences_of("verbose") > 0 {
        let verbose_str = matches.value_of("verbose").unwrap();

        match verbose_str.parse::<u64>() {
            Ok(val) => {
                if val > 1 {
                    verbose = 0;
                    err = Some(error::Error::with_description(&format!("invalid verbosity level ({}): must be a value in the range of 0-1 and defaults to 1 if no value specified", val), clap::ErrorKind::InvalidValue));
                    println!("{}", err.as_ref().unwrap());

                } else { // if val > 0
                    verbose = val;
                    println!("verbosity level: {}", verbose);
                }
            },

            Err(e) => {
                verbose = 0;
                err = Some(error::Error::with_description(&format!("invalid verbosity level ({}): must be a value in the range of 0-1 and defaults to 1 if no value specified: {}", verbose_str, e), clap::ErrorKind::InvalidValue));
                println!("{}", err.as_ref().unwrap());
            },
        }
    } else {
        verbose = 0;
    }

    /* ----- END Command that sets the verbosity 
             level.                              ----- */


    
    /* ----- Utility commands ----- */

    // If wav to txt is requested.
    if matches.is_present("wav_to_txt") {
        let path = matches.value_of("wav_to_txt").unwrap_or_default();

        match wav_to_txt(path, verbose) {
            Ok(_res) => {
                return Ok(0);
            },
            Err(e) => {
                if e.kind != ErrorKind::Io {
                    println!("{}", e);
                }

                err = Some(error::Error::from_clap_error(e));
            },
        }
    }

    // If txt to wav is requested.
    if matches.is_present("txt_to_wav") {
        let path = matches.value_of("txt_to_wav").unwrap_or_default();

        let output_binary = matches.is_present("output_binary");

        match txt_to_wav(path, output_binary, verbose) {
            Ok(_res) => {
                return Ok(0);
            },
            Err(e) => {
                if e.kind != ErrorKind::Io {
                    println!("{}", e);
                }

                err = Some(error::Error::from_clap_error(e));
            },
        }
    }

    /* ----- END Utility commands ----- */    



    /* ----- Command that selects the devices 
             to use.                          ----- */

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
    
    if verbose > 0 {
        println!("\n\nDevice(s) selected: {:?}\n", devices);
    }

    /* ----- END Command that selects the devices 
             to use.                              ----- */




    // Iterate over each device and perform commands on each.
    for device in &devices {
        if verbose > 0 {
            println!("\n\nOpening communication link with device: {}\n", device);
        }

        let err = &mut err;

        // Open the device.
        let opened = open(device).map_or_else(
            // If opening the device failed.
            |e| {
                Err(error::Error::with_description(&format!("(device: {}): {}: make sure the device is connected and turned on, or try specifying a different device path with -d /path/to/device", device, e), clap::ErrorKind::Io))
            },

            // If the device was opened successfully.
            |mut port| {
                let mut err: Option<error::Error> = None;

                /* ----- Commands which retrieve values from 
                         the device.                         ----- */


                // If model number is requested.
                if matches.is_present("get_model") {
                    read_machine_model(&mut port, verbose).unwrap();
                }

                // If serial number is requested.
                if matches.is_present("get_serial") {
                    read_machine_number(&mut port, verbose).unwrap();
                }


                // If read arbitrary wave is requested.
                if matches.is_present("read_arbitrary_wave") {
                    let arg = matches.value_of("read_arbitrary_wave").unwrap_or_default();

                    match match_read_arbitrary_wave_arg(&mut port, arg, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If get channel output is requested.
                if matches.is_present("get_channel_output") {
                    match get_channel_output(&mut port, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If get waveform for channel1 is requested.
                if matches.is_present("get_waveform_channel1") {
                    match get_waveform_preset(&mut port, 1, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If get waveform for channel2 is requested.
                if matches.is_present("get_waveform_channel2") {
                    match get_waveform_preset(&mut port, 2, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If get frequency for channel1 in Hz is requested.
                if matches.is_present("get_frequency_hz_channel1") {
                    match get_frequency_hertz(&mut port, 1, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If get frequency for channel1 in Hz is requested.
                if matches.is_present("get_frequency_hz_channel2") {
                    match get_frequency_hertz(&mut port, 2, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If get amplitude for channel1 in volts is requested.
                if matches.is_present("get_amplitude_volts_channel1") {
                    match get_amplitude(&mut port, 1, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If get frequency for channel2 in MHz is requested.
                if matches.is_present("get_amplitude_volts_channel2") {
                    match get_amplitude(&mut port, 2, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If get duty cycle for channel1 in percent is requested.
                if matches.is_present("get_duty_cycle_channel1") {
                    match get_duty_cycle(&mut port, 1, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If get duty cycle for channel2 in percent is requested.
                if matches.is_present("get_duty_cycle_channel2") {
                    match get_duty_cycle(&mut port, 2, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If get voltage offset for channel1 in volts is requested.
                if matches.is_present("get_voltage_offset_channel1") {
                    match get_voltage_offset(&mut port, 1, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If get voltage offset for channel2 in volts is requested.
                if matches.is_present("get_voltage_offset_channel2") {
                    match get_voltage_offset(&mut port, 2, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                /* ----- END Commands which retrieve values from 
                         the device.                             ----- */




                /* ----- Commands which navigate to a different
                         view on the device's display panel.    ----- */


                // If set switch function panel main ch1 is requested.
                if matches.is_present("switch_main_ch1") {
                    match set_switch_function_panel_main(&mut port, 1, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set switch function panel main ch2 is requested.
                if matches.is_present("switch_main_ch2") {
                    match set_switch_function_panel_main(&mut port, 2, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set switch function panel system settings is requested.
                if matches.is_present("switch_sys") {
                    match set_switch_function_panel_sys(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set switch function panel counting is requested.
                if matches.is_present("switch_counting") {
                    match set_switch_function_panel_counting(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                
                // If set switch function panel sweep ch1 is requested.
                if matches.is_present("switch_sweep_ch1") {
                    match set_switch_function_panel_sweep(&mut port, 1, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set switch function panel sweep ch2 is requested.
                if matches.is_present("switch_sweep_ch2") {
                    match set_switch_function_panel_sweep(&mut port, 2, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                
                // If set switch function panel pulse is requested.
                if matches.is_present("switch_pulse") {
                    match set_switch_function_panel_pulse(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                
                // If set switch function panel bursting is requested.
                if matches.is_present("switch_burst") {
                    match set_switch_function_panel_bursting(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                
                // If set switch function panel measurement is requested.
                if matches.is_present("switch_measurement") {
                    match set_switch_function_panel_measurement(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }
                

                /* ----- END Commands which navigate to a different
                         view on the device's display panel.        ----- */




                /* ----- Commands which change the device's 
                         settings or state, but don't 
                         activate the channels.             ----- */


                // If set tracking mode is requested.
                if matches.is_present("set_tracking") {
                    let track = matches.value_of("set_tracking").unwrap_or_default();
                    
                    match match_set_tracking_arg(&mut port, track, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set waveform for channel1 is requested.
                if matches.is_present("set_waveform_channel1") {
                    let preset = matches.value_of("set_waveform_channel1").unwrap_or_default();
                    
                    match match_set_waveform_preset_arg(&mut port, 1, preset, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set waveform for channel2 is requested.
                if matches.is_present("set_waveform_channel2") {
                    let preset = matches.value_of("set_waveform_channel2").unwrap_or_default();
                    
                    match match_set_waveform_preset_arg(&mut port, 2, preset, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set arbitrary waveform for channel1 is requested.
                if matches.is_present("set_arbitrary_waveform_channel1") {
                    let preset = matches.value_of("set_arbitrary_waveform_channel1").unwrap_or_default();
                    
                    match match_set_waveform_arbitrary_arg(&mut port, 1, preset, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set arbitrary waveform for channel2 is requested.
                if matches.is_present("set_arbitrary_waveform_channel2") {
                    let preset = matches.value_of("set_arbitrary_waveform_channel2").unwrap_or_default();
                    
                    match match_set_waveform_arbitrary_arg(&mut port, 2, preset, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set frequency for channel1 in uHz is requested.
                if matches.is_present("set_frequency_uhz_channel1") {
                    let amount = matches.value_of("set_frequency_uhz_channel1").unwrap_or_default();
                    
                    match match_set_frequency_microhertz_arg(&mut port, 1, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel2 in uHz is requested.
                if matches.is_present("set_frequency_uhz_channel2") {
                    let amount = matches.value_of("set_frequency_uhz_channel2").unwrap_or_default();
                    
                    match match_set_frequency_microhertz_arg(&mut port, 2, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel1 in mHz is requested.
                if matches.is_present("set_frequency_millihz_channel1") {
                    let amount = matches.value_of("set_frequency_millihz_channel1").unwrap_or_default();
                    
                    match match_set_frequency_millihertz_arg(&mut port, 1, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel2 in mHz is requested.
                if matches.is_present("set_frequency_millihz_channel2") {
                    let amount = matches.value_of("set_frequency_millihz_channel2").unwrap_or_default();
                    
                    match match_set_frequency_millihertz_arg(&mut port, 2, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel1 in Hz is requested.
                if matches.is_present("set_frequency_hz_channel1") {
                    let amount = matches.value_of("set_frequency_hz_channel1").unwrap_or_default();
                    
                    match match_set_frequency_hertz_arg(&mut port, 1, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel2 in Hz is requested.
                if matches.is_present("set_frequency_hz_channel2") {
                    let amount = matches.value_of("set_frequency_hz_channel2").unwrap_or_default();
                    
                    match match_set_frequency_hertz_arg(&mut port, 2, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel1 in kHz is requested.
                if matches.is_present("set_frequency_khz_channel1") {
                    let amount = matches.value_of("set_frequency_khz_channel1").unwrap_or_default();
                    
                    match match_set_frequency_kilohertz_arg(&mut port, 1, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel2 in kHz is requested.
                if matches.is_present("set_frequency_khz_channel2") {
                    let amount = matches.value_of("set_frequency_khz_channel2").unwrap_or_default();
                    
                    match match_set_frequency_kilohertz_arg(&mut port, 2, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel1 in MHz is requested.
                if matches.is_present("set_frequency_mega_channel1") {
                    let amount = matches.value_of("set_frequency_mega_channel1").unwrap_or_default();
                    
                    match match_set_frequency_megahertz_arg(&mut port, 1, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel2 in MHz is requested.
                if matches.is_present("set_frequency_mega_channel2") {
                    let amount = matches.value_of("set_frequency_mega_channel2").unwrap_or_default();
                    
                    match match_set_frequency_megahertz_arg(&mut port, 2, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set amplitude for channel1 in volts is requested.
                if matches.is_present("set_amplitude_volts_channel1") {
                    let amount = matches.value_of("set_amplitude_volts_channel1").unwrap_or_default();
                    
                    match match_set_amplitude_arg(&mut port, 1, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set frequency for channel2 in MHz is requested.
                if matches.is_present("set_amplitude_volts_channel2") {
                    let amount = matches.value_of("set_amplitude_volts_channel2").unwrap_or_default();
                    
                    match match_set_amplitude_arg(&mut port, 2, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set duty cycle for channel1 in percent is requested.
                if matches.is_present("set_duty_cycle_channel1") {
                    let amount = matches.value_of("set_duty_cycle_channel1").unwrap_or_default();
                    
                    match match_set_duty_cycle_arg(&mut port, 1, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set duty cycle for channel2 in percent is requested.
                if matches.is_present("set_duty_cycle_channel2") {
                    let amount = matches.value_of("set_duty_cycle_channel2").unwrap_or_default();
                    
                    match match_set_duty_cycle_arg(&mut port, 2, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set voltage offset for channel1 in volts is requested.
                if matches.is_present("set_voltage_offset_channel1") {
                    let amount = matches.value_of("set_voltage_offset_channel1").unwrap_or_default();
                    
                    match match_set_voltage_offset_arg(&mut port, 1, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set voltage offset for channel2 in volts is requested.
                if matches.is_present("set_voltage_offset_channel2") {
                    let amount = matches.value_of("set_voltage_offset_channel2").unwrap_or_default();
                    
                    match match_set_voltage_offset_arg(&mut port, 2, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set phase in degrees is requested.
                if matches.is_present("set_phase") {
                    let amount = matches.value_of("set_phase").unwrap_or_default();
                    
                    match match_set_phase_arg(&mut port, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                
                // If set measurement coupling ac is requested.
                if matches.is_present("set_measurement_coupling_ac") {
                    match set_measurement_coupling_ac(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set measurement coupling dc is requested.
                if matches.is_present("set_measurement_coupling_dc") {
                    match set_measurement_coupling_dc(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set measurement gate time in seconds is requested.
                if matches.is_present("set_measurement_gate_time") {
                    let amount = matches.value_of("set_measurement_gate_time").unwrap_or_default();
                    
                    match match_set_measurement_gate_time_arg(&mut port, amount, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set measurement mode count frequency is requested.
                if matches.is_present("set_measurement_count_frequency") {
                    match set_measurement_mode_count_frequency(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set measurement mode counting period is requested.
                if matches.is_present("set_measurement_counting_period") {
                    match set_measurement_mode_counting_period(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set burst pulse number is requested.
                if matches.is_present("set_burst_pulse_number") {
                    let amount = matches.value_of("set_burst_pulse_number").unwrap_or_default();

                    match match_set_burst_pulse_number_arg(&mut port, amount, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set burst mode manual trigger is requested.
                if matches.is_present("set_burst_manual_trigger") {
                    match set_burst_mode_manual_trigger(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set burst mode CH2 burst is requested.
                if matches.is_present("set_burst_ch2") {
                    match set_burst_mode_ch2_burst(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set burst mode external burst AC is requested.
                if matches.is_present("set_burst_external_ac") {
                    match set_burst_mode_external_burst_ac(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set burst mode external burst DC is requested.
                if matches.is_present("set_burst_external_dc") {
                    match set_burst_mode_external_burst_dc(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set sweep starting frequency is requested.
                if matches.is_present("set_sweep_start_freq") {
                    let amount = matches.value_of("set_sweep_start_freq").unwrap_or_default();

                    match match_set_sweep_starting_frequency_arg(&mut port, amount, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set sweep termination frequency is requested.
                if matches.is_present("set_sweep_stop_freq") {
                    let amount = matches.value_of("set_sweep_stop_freq").unwrap_or_default();

                    match match_set_sweep_termination_frequency_arg(&mut port, amount, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                
                // If set sweep time is requested.
                if matches.is_present("set_sweep_time") {
                    let amount = matches.value_of("set_sweep_time").unwrap_or_default();

                    match match_set_sweep_time_arg(&mut port, amount, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set sweep direction normal (rise) is requested.
                if matches.is_present("set_sweep_direction_rise") {
                    match set_sweep_direction_normal(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set sweep direction reverse (fall) is requested.
                if matches.is_present("set_sweep_direction_fall") {
                    match set_sweep_direction_reverse(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set sweep direction round trip (rise and fall) is requested.
                if matches.is_present("set_sweep_direction_rise_fall") {
                    match set_sweep_direction_round_trip(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set sweep mode linear is requested.
                if matches.is_present("set_sweep_linear") {
                    match set_sweep_mode_linear(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set sweep mode logarithm is requested.
                if matches.is_present("set_sweep_logarithm") {
                    match set_sweep_mode_logarithm(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set pulse width nanoseconds is requested.
                if matches.is_present("set_pulse_width_nanoseconds") {
                    let amount = matches.value_of("set_pulse_width_nanoseconds").unwrap_or_default();

                    match match_set_pulse_width_arg(&mut port, amount, false, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set pulse width microseconds is requested.
                if matches.is_present("set_pulse_width_microseconds") {
                    let amount = matches.value_of("set_pulse_width_microseconds").unwrap_or_default();

                    match match_set_pulse_width_arg(&mut port, amount, true, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set pulse period nanoseconds is requested.
                if matches.is_present("set_pulse_period_nanoseconds") {
                    let amount = matches.value_of("set_pulse_period_nanoseconds").unwrap_or_default();

                    match match_set_pulse_period_arg(&mut port, amount, false, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }

                // If set pulse period microseconds is requested.
                if matches.is_present("set_pulse_period_microseconds") {
                    let amount = matches.value_of("set_pulse_period_microseconds").unwrap_or_default();

                    match match_set_pulse_period_arg(&mut port, amount, true, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set pulse offset is requested.
                if matches.is_present("set_pulse_offset") {
                    let amount = matches.value_of("set_pulse_offset").unwrap_or_default();

                    match match_set_pulse_offset_arg(&mut port, amount, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set pulse amplitude is requested.
                if matches.is_present("set_pulse_amplitude") {
                    let amount = matches.value_of("set_pulse_amplitude").unwrap_or_default();

                    match match_set_pulse_amplitude_arg(&mut port, amount, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set measurement count clear is requested.
                if matches.is_present("clear_measurement_count") {
                    match set_measurement_count_clear(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If save preset is requested.
                if matches.is_present("save_preset") {
                    let amount = matches.value_of("save_preset").unwrap_or_default();

                    match match_save_preset_arg(&mut port, amount, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If recall preset is requested.
                if matches.is_present("recall_preset") {
                    let amount = matches.value_of("recall_preset").unwrap_or_default();

                    match match_recall_preset_arg(&mut port, amount, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If clear preset is requested.
                if matches.is_present("clear_preset") {
                    let amount = matches.value_of("clear_preset").unwrap_or_default();

                    match match_clear_preset_arg(&mut port, amount, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If write arbitrary wave stdin is requested.
                if matches.is_present("write_arbitrary_wave_stdin") {
                    let arg = matches.value_of("write_arbitrary_wave_stdin").unwrap_or_default();

                    match match_write_arbitrary_wave_stdin_arg(&mut port, arg, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If write arbitrary wavecad is requested.
                if matches.is_present("write_arbitrary_wavecad") {
                    let arg = matches.value_of("write_arbitrary_wavecad").unwrap_or_default();

                    match match_write_arbitrary_wavecad_arg(&mut port, arg, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                /* ----- END Commands which change the device's 
                         settings or state, but don't  
                         activate the channels.                 ----- */




                /* ----- Commands which set one or both of the 
                         device's channels ON or OFF.          ----- */


                // If set counting starting is requested.
                if matches.is_present("start_counting") {
                    match set_counting_starting(&mut port, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set sweep starting ch1 is requested.
                if matches.is_present("start_sweeping_ch1") {
                    match set_sweep_starting(&mut port, 1, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set sweep starting ch2 is requested.
                if matches.is_present("start_sweeping_ch2") {
                    match set_sweep_starting(&mut port, 2, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set pulse starting is requested.
                if matches.is_present("start_pulsing") {
                    match set_pulse_starting(&mut port, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set burst pulse once is requested.
                if matches.is_present("burst_pulse_once") {
                    match set_burst_pulse_once(&mut port, verbose) {
                        Ok(_res) => {
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set bursting starting is requested.
                if matches.is_present("start_bursting") {
                    match set_bursting_starting(&mut port, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set measurement starting is requested.
                if matches.is_present("start_measuring") {
                    match set_measurement_starting(&mut port, verbose) {
                        Ok(_res) => {
                            match set_channel_output(&mut port, false, false, verbose) {
                                Ok(_res) => {},
                                Err(e) => {
                                    err = Some(error::Error::from_clap_error(e));
                                    println!("{}", err.as_ref().unwrap());
                                },
                            }
                        },
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                // If set channel output is requested.
                if matches.is_present("set_channel_output") {
                    let sco = matches.value_of("set_channel_output").unwrap_or_default();
                    
                    match match_set_channel_output_arg(&mut port, sco, verbose) {
                        Ok(_res) => {},
                        Err(e) => {
                            err = Some(error::Error::from_clap_error(e));
                            println!("{}", err.as_ref().unwrap());
                        },
                    }
                }


                /* ----- END Commands which set one or both of the 
                         device's channels ON.                     ----- */

                err.map_or_else(|| { Ok(0) }, |v| { Err(v) })
            },
        );

        // If we can't connect to a certain device, continue with any 
        // devices we can connect to.
        if opened.is_err() {
            *err = opened.map_or_else(
                |e| {
                    if e.kind == ErrorKind::Io {
                        println!("{}", e);
                    }

                    Some(e)
                },

                |_val| {
                    None
                }
            );
            
            continue;
        }
    }

    if err.is_some() {
        Err(err.unwrap())
    } else {
        Ok(0)
    }
}
