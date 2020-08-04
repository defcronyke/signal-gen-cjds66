pub mod serial;
pub mod protocol;
pub mod command;

pub fn test() {
    println!("test");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
