#!/bin/bash

# max=0; while true; do vram=$(nvidia-smi --query-gpu=memory.used --format=csv,noheader,nounits | tr -d '\r'); if [ "$vram" -gt "$max" ]; then max=$vram; fi; echo -ne "Current: ${vram} MiB | Max: ${max} MiB\r"; sleep 1; done

# RISC0
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 1 --max-segment-limit 20 2>&1 | tee -a risc0_1_sig.txt
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 10 --max-segment-limit 20 2>&1 | tee -a risc0_10_sig.txt
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 50 --max-segment-limit 20 2>&1 | tee -a risc0_50_sig.txt
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 100 --max-segment-limit 20 2>&1 | tee -a risc0_50_sig.txt