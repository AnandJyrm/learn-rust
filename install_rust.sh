#! /bin/bash

echo "Installing Rust"
printf '%*s\n' "${COLUMNS:-$(tput cols)}" '' | tr ' ' -
export RUSTUP_HOME=$PWD/rust-bin
export CARGO_HOME=$PWD/rust-bin

curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
source $PWD/rust-bin/env
rustup update

echo
echo
echo "Done"
printf '%*s\n' "${COLUMNS:-$(tput cols)}" '' | tr ' ' -

echo "use 'source $PWD/RUSTENV' to activate the rust env"
echo "use 'deactivate' to deactivate the env"
echo
printf '%*s\n' "${COLUMNS:-$(tput cols)}" '' | tr ' ' -
