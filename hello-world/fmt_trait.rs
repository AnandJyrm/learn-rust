#![allow(dead_code)]
use fmt::Formatter;
use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

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

    let minmax = MinMax(0, 14);
    println!("Display {}", minmax);
    println!("Debug {:?}", minmax);
}
