RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin risc0-host -- --n-signatures 1 --max-segment-limit 22 > risc0_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin risc0-host -- --n-signatures 10 --max-segment-limit 22 > risc0_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin risc0-host -- --n-signatures 50 --max-segment-limit 22 > risc0_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin risc0-host -- --n-signatures 100 --max-segment-limit 22 > risc0_50_sig.txt

RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 1 --max-segment-limit 22 prove > sp1_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 10 --max-segment-limit 22 prove > sp1_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 50 --max-segment-limit 22 prove > sp1_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 100 --max-segment-limit 22 prove > sp1_50_sig.txt