use libc;

fn main() {
    println!("Hello, world!");
    unsafe {
        _ = libc::malloc(20);
    }
}
