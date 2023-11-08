# callback

Exercise passing a callback function to C library to be invoked by thread created in C.

## Key concepts

1. FFI binding -> [build.rs](build.rs)
2. Function Pointers -> [main.rs](src/main.rs)
3. Linker rpath options -> [config](.cargo/config)

## Build/Execution

1. First build the C code under C_lib

```bash
cd C_lib
clang -g -shared -fPIC timed_event.c -o libtimed_event.so
```

2. Run the Rust code using Cargo

```bash
cargo run
```
