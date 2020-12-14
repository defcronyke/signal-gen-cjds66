#[macro_use]
extern crate bitflags;
extern crate clap;

pub mod util;
pub mod serial;
pub mod protocol;
pub mod command;
pub mod error;

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
}
