macro_rules! test_macro {
    () => {
        println!("nothing to test");
    };
    ($i:ident) => {
        println!("Identifier: {:?}: {:?}", stringify!($i), $i)
    };
    ($i:expr) => {
        println!("Identifier: {:?}: {:?}", stringify!($i), $i)
    };
}

fn main() {
    test_macro!();
    test_macro!(1i32);
    let x:u32 = 32;
    test_macro!(x);
    test_macro!(x == 32)
}
