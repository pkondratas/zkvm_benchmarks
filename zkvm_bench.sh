# max=0; while true; do vram=$(nvidia-smi --query-gpu=memory.used --format=csv,noheader,nounits | tr -d '\r'); if [ "$vram" -gt "$max" ]; then max=$vram; fi; echo -ne "Current: ${vram} MiB | Max: ${max} MiB\r"; sleep 1; done

#!/bin/bash

# RISC0
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 1 --max-segment-limit 20 >> risc0_1_sig.txt 2>&1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 10 --max-segment-limit 20 >> risc0_10_sig.txt 2>&1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 50 --max-segment-limit 20 >> risc0_50_sig.txt 2>&1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info RISC0_INFO=1 cargo run --release --bin risc0-host -- --n-signatures 100 --max-segment-limit 20 >> risc0_50_sig.txt 2>&1

# SP1
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 1 --max-segment-limit 20 execute >> sp1_1_sig.txt 2>&1
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 1 --max-segment-limit 20 prove >> sp1_1_sig.txt 2>&1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 10 --max-segment-limit 20 execute >> sp1_1_sig.txt 2>&1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 10 --max-segment-limit 20 prove >> sp1_10_sig.txt 2>&1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 50 --max-segment-limit 20 execute >> sp1_1_sig.txt 2>&1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 50 --max-segment-limit 20 prove >> sp1_50_sig.txt 2>&1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 100 --max-segment-limit 20 execute >> sp1_1_sig.txt 2>&1
# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 100 --max-segment-limit 20 prove >> sp1_50_sig.txt 2>&1