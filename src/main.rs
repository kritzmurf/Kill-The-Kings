#[allow(unused)]
mod ui;

use crate::ui::app::{App};

use std::{io};

pub fn hello_world() -> &'static str {
    "Hello, Kill The Kings!"
}


fn main() ->Result<(), io::Error> {
    println!("Hello, world!");
    println!("This is Kill The Kings..... someday");

    let _app = App::default(); 
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;


    #[test]
    fn test_hello_world() {
        assert_eq!("Hello, Kill The Kings!", hello_world());
    }
}
