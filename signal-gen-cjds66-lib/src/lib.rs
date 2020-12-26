/* Copyright © 2020 Jeremy Carter <jeremy@jeremycarter.ca>

By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.
*/

#[macro_use]
extern crate bitflags;
extern crate clap;

pub mod command;
pub mod error;
pub mod protocol;
pub mod serial;
pub mod util;

#[cfg(test)]
mod tests {
	// #[test]
	// fn it_works() {
	//     assert_eq!(2 + 2, 4);
	// }
}
