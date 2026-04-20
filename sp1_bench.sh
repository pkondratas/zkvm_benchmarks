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

# SP1
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 1 --max-segment-limit 20 execute 2>&1 | tee -a sp1_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 1 --max-segment-limit 20 prove 2>&1 | tee -a sp1_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 10 --max-segment-limit 20 execute 2>&1 | tee -a sp1_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 10 --max-segment-limit 20 prove 2>&1 | tee -a sp1_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 50 --max-segment-limit 20 execute 2>&1 | tee -a sp1_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 50 --max-segment-limit 20 prove 2>&1 | tee -a sp1_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 100 --max-segment-limit 20 execute 2>&1 | tee -a sp1_100_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 100 --max-segment-limit 20 prove 2>&1 | tee -a sp1_100_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 200 --max-segment-limit 20 execute 2>&1 | tee -a sp1_200_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 200 --max-segment-limit 20 prove 2>&1 | tee -a sp1_200_sig.txt