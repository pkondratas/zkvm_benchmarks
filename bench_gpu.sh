#!/bin/bash

#/usr/bin/time -v /root/.cargo/bin/cargo 

# max=0; while true; do vram=$(nvidia-smi --query-gpu=memory.used --format=csv,noheader,nounits | tr -d '\r'); if [ "$vram" -gt "$max" ]; then max=$vram; fi; echo -ne "Current: ${vram} MiB | Max: ${max} MiB\r"; sleep 1; done

mkdir -p /workspace/cpu

git clone --no-recurse-submodules https://github.com/pkondratas/zkvm_benchmarks.git
cd zkvm_benchmarks
git submodule update --init leanSig

# SP1
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 1 --max-segment-limit 20 execute 2>&1 | tee -a sp1_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 1 --max-segment-limit 20 prove 2>&1 | tee -a sp1_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 10 --max-segment-limit 20 execute 2>&1 | tee -a sp1_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 10 --max-segment-limit 20 prove 2>&1 | tee -a sp1_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 20 --max-segment-limit 20 execute 2>&1 | tee -a sp1_20_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 20 --max-segment-limit 20 prove 2>&1 | tee -a sp1_20_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 50 --max-segment-limit 20 execute 2>&1 | tee -a sp1_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 50 --max-segment-limit 20 prove 2>&1 | tee -a sp1_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 100 --max-segment-limit 20 execute 2>&1 | tee -a sp1_100_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release -p sp1-host-gpu -- --n-signatures 100 --max-segment-limit 20 prove 2>&1 | tee -a sp1_100_sig.txt

# RISC0
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 /usr/bin/time -v /root/.cargo/bin/cargo run --release -p risc0-host-gpu -- --n-signatures 1 --max-segment-limit 21 2>&1 | tee -a risc0_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 /usr/bin/time -v /root/.cargo/bin/cargo run --release -p risc0-host-gpu -- --n-signatures 10 --max-segment-limit 21 2>&1 | tee -a risc0_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 /usr/bin/time -v /root/.cargo/bin/cargo run --release -p risc0-host-gpu -- --n-signatures 20 --max-segment-limit 21 2>&1 | tee -a risc0_20_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 /usr/bin/time -v /root/.cargo/bin/cargo run --release -p risc0-host-gpu -- --n-signatures 50 --max-segment-limit 21 2>&1 | tee -a risc0_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 /usr/bin/time -v /root/.cargo/bin/cargo run --release -p risc0-host-gpu -- --n-signatures 100 --max-segment-limit 21 2>&1 | tee -a risc0_50_sig.txt

runpodctl remove pod $RUNPOD_POD_IDs