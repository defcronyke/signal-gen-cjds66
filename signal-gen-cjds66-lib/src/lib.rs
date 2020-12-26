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
