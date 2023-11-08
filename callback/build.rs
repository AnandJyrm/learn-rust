use std::env;
use std::path::PathBuf;

fn main() {
    let lib_path = env::var("REPO").unwrap() + "/callback/C_lib";
    println!("cargo:rustc-link-search=native={}", lib_path);
    // timed_event will be converted to libtimed_event.so
    println!("cargo:rustc-link-lib=dylib=timed_event");

    let builder = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_default(true);

    let mut bindgen_args: Vec<String> = vec![];
    bindgen_args.push(String::to_string(&["-I", &lib_path].concat()));
    println!("bindgen_args: {:?}", bindgen_args);
    let bindings = builder
        .clang_args(bindgen_args)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("REPO").unwrap() + "/callback/src/");
    bindings
        .write_to_file(out_path.join("bindings_gen.rs"))
        .expect("Couldn't write bindings!");
}
