macro_rules! print_statement {
    ($statement:stmt) => {
        println!("{}", stringify!($statement))
    };
}

fn main() {
    print_statement!({
        println!("hello");
    })
}
