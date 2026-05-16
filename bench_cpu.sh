#!/bin/bash

mkdir -p /workspace/cpu

# SP1
echo "Running SP1 Hypercube benchmarks | CPU | 1 signature"
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info /usr/bin/time -v /root/.cargo/bin/cargo build --release -p sp1-host-cpu -- --n-signatures 1 --max-segment-limit 18 execute 2>&1 | tee -a /workspace/cpu/sp1_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info /usr/bin/time -v /root/.cargo/bin/cargo run --release -p sp1-host-cpu -- --n-signatures 1 --max-segment-limit 18 prove 2>&1 | tee -a /workspace/cpu/sp1_1_sig.txt

# RISC0
echo "Running RISC Zero benchmarks | CPU | 1 signature"
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 /usr/bin/time -v /root/.cargo/bin/cargo run --release -p risc0-host-cpu -- --n-signatures 1 --max-segment-limit 22 2>&1 | tee -a /workspace/cpu/risc0_1_sig.txt

runpodctl remove pod $RUNPOD_POD_ID