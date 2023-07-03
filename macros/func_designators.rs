macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You have called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();
}
