use my_proc_macro::derive_db_key;

pub trait GetDbKey {
    fn get_db_key() -> &'static str;
}

#[derive(Debug)]
#[derive_db_key(inv)]
struct Inv {
    x: u32,
}

fn main() {
    println!("hello world");
    let c = Inv { x: 123 };
    println!("{} {}", c.x, Inv::get_db_key());
}
