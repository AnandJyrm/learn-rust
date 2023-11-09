# learn-rust

Learn rust basics and some toolchains. Steps are Linux specific.

## Content

### Basics
1. [hello-world](hello-world)
2. [macros](macros)
3. [types](types)
4. [callback](callback)
5. [proc_macros](learn_proc)

### Tools
1. [coverage](coverage)
2. [sanitizer](sanitizer)

## Setup
Installs Rust locally at learn-rust/bin

```bash
git clone git@github.com:AnandJyrm/learn-rust.git
cd learn-rust
./install_rust.sh
source RUSTENV
```

## Execute in RUSTENV

Direct rustc usage:

```bash
rustc hello.rs -o a.out
./a.out
```

Cargo usage:

```bash
cargo new hello
cd hello
cargo run
```

### Warning

- bin folder and its contents are part of .gitignore. This will contain the rust installation files.
- To reset the repo, use `git clean -Xfd`.
- `git clean -xfd` will reset the rust installation.

