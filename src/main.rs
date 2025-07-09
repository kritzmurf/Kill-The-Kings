mod tui;

pub fn hello_world() -> &'static str {
    "Hello, Kill The Kings!"
}


fn main() {
    println!("Hello, world!");
    println!("This is Kill The Kings..... someday");
}

#[cfg(test)]
pub mod tests {
    use super::*;


    #[test]
    fn test_hello_world() {
        assert_eq!("Hello, Kill The Kings!", hello_world());
    }
}
