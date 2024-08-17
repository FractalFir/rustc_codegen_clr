#!/usr/bin/env bash

# Scrpit adapted from: https://github.com/rust-lang/rustc_codegen_cranelift 

set -e

cargo build && cargo build --release 

echo "[SETUP] Downloading the Rust compiler"
test -e "rust" || git clone https://github.com/rust-lang/rust.git --filter=tree:0 --quiet  --depth 500 || true
pushd rust
echo "[SETUP] Finished donwloading the Rust compiler"
git fetch
git checkout --no-progress -- .
git checkout --no-progress "$(rustc -V | cut -d' ' -f3 | tr -d '(')"
echo "[SETUP] Updating git submodules"
git submodule update --quiet --init src/tools/cargo library/backtrace library/stdarch
echo "[SETUP] Finished updating git submodules"
#git -c user.name=Dummy -c user.email=dummy@example.com -c commit.gpgSign=false \
 #   am ../patches/*-stdlib-*.patch
echo "[SETUP] Writing config"
cat > config.toml <<EOF
change-id = 999999

[build]
rustc = "$(rustup which rustc)"
cargo = "$(rustup which cargo)"
full-bootstrap = false
local-rebuild = true

[rust]

deny-warnings = false
verbose-tests = false
EOF
popd

# Allow the testsuite to use llvm tools
host_triple=$(rustc -vV | grep host | cut -d: -f2 | tr -d " ")
export LLVM_BIN_DIR="$(rustc --print sysroot)/lib/rustlib/$host_triple/bin"
