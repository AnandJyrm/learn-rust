export RUSTUP_HOME=$PWD/rust-bin
export CARGO_HOME=$PWD/rust-bin
echo $RUSTUP_HOME
echo $CARGO_HOME
curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
source $PWD/rust-bin/env
rustup update