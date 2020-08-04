extern crate signal_gen_cjds66_lib; // For compatibility with Rust pre-2018 versions.

use std::env;
use signal_gen_cjds66_lib::serial::open;
use signal_gen_cjds66_lib::command::*;

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}

fn real_main() -> i32 {
    let mut args = env::args_os();
    
    if args.len() < 2 {
        println!("Error: The first argument must be the serial port to connect to, such as: /dev/ttyUSB0");
        return 1;
    }

    let mut port = &mut open(&args.nth(1).unwrap()).unwrap();

    get_machine_model(&mut port).unwrap();

    get_machine_number(&mut port).unwrap();

    0
}
