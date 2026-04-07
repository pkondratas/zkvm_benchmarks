#!/bin/bash
set -euo pipefail

apt-get update && apt-get install -y curl build-essential libssl-dev pkg-config git protobuf-compiler libclang-dev

git clone --no-recurse-submodules https://github.com/pkondratas/zkvm_benchmarks.git && \
    cd zkvm_benchmarks && \
    git submodule update --init leanSig && \
    git checkout cpu-local-proving

cd

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

curl -L https://risczero.com/install | bash
source "/root/.bashrc"

rzup install

cd zkvm_benchmarks/

RISC0_DEV_MODE=0 cargo run --release --bin risc0-host