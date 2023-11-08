#! /bin/bash
# Install a rust env at bin

# find current repo path
export REPO=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

echo "Installing Rust"
# rustenv variables
export RUSTUP_HOME=${REPO}/bin
export CARGO_HOME=${REPO}/bin

# create the cargo install env
mkdir -p "${CARGO_HOME}"

# ignore previous cargo installations
export RUSTUP_INIT_SKIP_PATH_CHECK=yes

# download install script
curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain stable-x86_64-unknown-linux-gnu
# shellcheck source=/dev/null
source "${RUSTUP_HOME}/env"

# update rust to latest and install additional tools
rustup update
# rust-analyzer is the lsp
rustup component add rust-analyzer

# optional tools to expand the macro, generate rpms and create coverage
cargo install cargo-expand cargo-generate-rpm grcov

