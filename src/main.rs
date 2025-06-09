pub fn hello_world() -> String {
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
