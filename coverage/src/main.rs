#![allow(non_camel_case_types)]
include!(concat!(env!("REPO"), "/coverage/src/bindings_gen.rs"));

fn main() {
    println!("Hello, world!");
    unsafe {
        cov_function(3);
    }
}
