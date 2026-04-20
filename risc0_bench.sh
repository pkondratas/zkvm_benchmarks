#!/bin/bash

# GPU

# RTX PRO 45001x

# vCPU

# 32 (AMD EPYC 7713P 64-Core Processor)

# Memory

# 62 GB

# Container disk

# 100 GB

# max=0; while true; do vram=$(nvidia-smi --query-gpu=memory.used --format=csv,noheader,nounits | tr -d '\r'); if [ "$vram" -gt "$max" ]; then max=$vram; fi; echo -ne "Current: ${vram} MiB | Max: ${max} MiB\r"; sleep 1; done

# RISC0
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 1 --max-segment-limit 21 2>&1 | tee -a risc0_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 10 --max-segment-limit 21 2>&1 | tee -a risc0_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 20 --max-segment-limit 21 2>&1 | tee -a risc0_20_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 50 --max-segment-limit 21 2>&1 | tee -a risc0_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 100 --max-segment-limit 21 2>&1 | tee -a risc0_100_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 200 --max-segment-limit 21 2>&1 | tee -a risc0_200_sig.txt