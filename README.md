# learn-rust

Learn rust basics

## Source

- [rust-by-example](https://doc.rust-lang.org/rust-by-example)

## Content

1. [hello-world](hello-world)
2. [macros](macros)
3. [types](types)

### Warning

- rust-bin folder contents are part of .gitignore. This contains the rust installation files.
- To reset the repo, use `git clean -Xfd`.
- `git clean -xfd` will reset the rust installation.

## Setup

```bash
git clone git@github.com:AnandJyrm/learn-rust.git
cd learn-rust
./install_rust.sh
source RUSTENV
```

## Execute in RUSTENV

```bash
rustc hello.rs -o a.out
./a.out
```
