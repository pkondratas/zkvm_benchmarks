#!/bin/bash

mkdir -p /workspace/cpu

echo "hello world" > /workspace/cpu/"test-$(date +%Y-%m-%d_%H-%M)"

# # SP1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info /usr/bin/time -v /root/.cargo/bin/cargo run --release -p sp1-host-cpu -- --n-signatures 1 --max-segment-limit 20 execute 2>&1 | tee -a sp1_1_sig.txt
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info /usr/bin/time -v /root/.cargo/bin/cargo run --release -p sp1-host-cpu -- --n-signatures 1 --max-segment-limit 20 prove 2>&1 | tee -a sp1_1_sig.txt

# # RISC0
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 /usr/bin/time -v /root/.cargo/bin/cargo run --release -p risc0-host-cpu -- --n-signatures 1 --max-segment-limit 22 2>&1 | tee -a risc0_1_sig.txt

runpodctl remove pod $RUNPOD_POD_ID