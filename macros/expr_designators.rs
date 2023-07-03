

// expr is a designator for expressions
macro_rules! print_results {
    ($expres:expr) => {
        println!("{:?} = {:?}", stringify!($expres), $expres);
    };
}

fn main() {
    print_results!(1u32 + 1);

    print_results!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}
