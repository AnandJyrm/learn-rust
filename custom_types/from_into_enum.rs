use std::convert::Into;

#[derive(Debug, Clone, Copy)]
enum Alpha {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy)]
enum Number {
    One,
    Two,
    Three,
    Four,
}

impl Into<Alpha> for Number {
    fn into(&self) -> Alpha {
        match self {
            Self::One => Alpha::A,
            Self::Two => Alpha::B,
            Self::Three => Alpha::C,
            Self::Four => Alpha::D,
        }
    }
}

fn main() {
    let a: Number = Number::One;
    let b: Alpha = a.into();
    println!("number is {:?}, alphabet is {:?}", a, b);
}
