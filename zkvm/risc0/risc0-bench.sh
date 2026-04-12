#!/bin/bash
set -euo pipefail

apt-get update && apt-get install -y curl build-essential libssl-dev pkg-config git protobuf-compiler libclang-dev time

git clone --no-recurse-submodules https://github.com/pkondratas/zkvm_benchmarks.git && \
    cd zkvm_benchmarks && \
    git submodule update --init leanSig && \
    git checkout gpu-proving

cd

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

curl -L https://risczero.com/install | bash
source ~/.bashrc

rzup install

export RISC0_DEV_MODE=$1

echo 'export PATH=/usr/local/cuda/bin:$PATH' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=/usr/local/cuda/lib64:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc

cd zkvm_benchmarks/

cargo build --release --bin risc0-host
/usr/bin/time -v cargo run --release --bin risc0-host