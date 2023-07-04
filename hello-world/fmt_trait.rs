use fmt::Formatter;
use std::fmt;

fn main() {
    struct Structure(i32);
    impl fmt::Debug for Structure {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "hello this is Structure's Debug: {}", self.0)
        }
    }
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "hello this is Structure's Display: {}", self.0)
        }
    }

    println!("Debug: {:?}", Structure(3));
    println!("Display: {}", Structure(3));

    #[derive(Debug)]
    struct AnotherStructure(i32);
    println!("Debug: {:#?}", AnotherStructure(3));
}
