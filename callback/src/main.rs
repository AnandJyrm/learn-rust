#![allow(non_camel_case_types)]
include!(concat!(env!("REPO"), "/callback/src/bindings_gen.rs"));

extern "C" fn implement_callback_fn() {
    println!(
        "TID: {:?} -> hello from the rust callback function!",
        gettid::gettid()
    );
}

fn main() -> Result<(), ()> {
    println!("TID: {:?} -> hello from main", gettid::gettid());
    unsafe {
        register_fn(Some(implement_callback_fn));
        init_timer();
    }
    std::thread::sleep(std::time::Duration::new(20, 0));
    Ok(())
}
