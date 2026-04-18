RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin risc0-host -- --n-signatures 1 --max-segment-limit 22 > risc0_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin risc0-host -- --n-signatures 10 --max-segment-limit 22 > risc0_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin risc0-host -- --n-signatures 50 --max-segment-limit 22 > risc0_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin risc0-host -- --n-signatures 100 --max-segment-limit 22 > risc0_50_sig.txt

RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 1 --max-segment-limit 22 prove > sp1_1_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 10 --max-segment-limit 22 prove > sp1_10_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 50 --max-segment-limit 22 prove > sp1_50_sig.txt
RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 100 --max-segment-limit 22 prove > sp1_50_sig.txt

# RTX PRO 4500

# Proving...
2026-04-18T21:13:30.477372Z  INFO sp1_gpu_server::server: Server listening @ /tmp/sp1-cuda-0.sock
2026-04-18T21:13:30.477443Z  INFO sp1_gpu_server::server: Connection accepted
2026-04-18T21:13:30.481078Z  INFO sp1_gpu_server::server: Running setup
2026-04-18T21:13:30.523567Z  INFO starting proof generation mode=Core
2026-04-18T21:13:30.523625Z  INFO sp1_gpu_server::server: Proving with mode: Core
Elapsed: 5658
Successfully generated proof!
Successfully verified proof!
2026-04-18T21:13:36.547400Z  INFO sp1_gpu_server::server: Destroying key
root@71f91846f4fd:/zkvm_benchmarks# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 10 --max-segment-limit 23 prove
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /zkvm_benchmarks/leanSig/Cargo.toml
workspace: /zkvm_benchmarks/Cargo.toml
warning: sp1-host@0.1.0: rustc +succinct --version: "rustc 1.93.0-dev\n"
warning: sp1-host@0.1.0: sp1_xmss_benchmark built at 2026-04-18 21:11:40
    Finished `release` profile [optimized + debuginfo] target(s) in 0.27s
     Running `target/release/sp1-host --n-signatures 10 --max-segment-limit 23 prove`
Generating signatures (this will be cached for future runs)...
2026-04-18T21:13:46.525984Z  INFO initializing cuda prover
Running sp1-gpu-server 6.1.0 with device 0
Proving...
2026-04-18T21:13:52.255472Z  INFO sp1_gpu_server::server: Server listening @ /tmp/sp1-cuda-0.sock
2026-04-18T21:13:52.255573Z  INFO sp1_gpu_server::server: Connection accepted
2026-04-18T21:13:52.260764Z  INFO sp1_gpu_server::server: Running setup
2026-04-18T21:13:52.304466Z  INFO starting proof generation mode=Core
2026-04-18T21:13:52.304639Z  INFO sp1_gpu_server::server: Proving with mode: Core
Elapsed: 10185
Successfully generated proof!
Successfully verified proof!
2026-04-18T21:14:03.298971Z  INFO sp1_gpu_server::server: Destroying key
root@71f91846f4fd:/zkvm_benchmarks# RUSTFLAGS="-C target-cpu=native" RUST_LOG=info cargo run --release --bin sp1-host -- --n-signatures 100 --max-segment-limit 23 prove
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /zkvm_benchmarks/leanSig/Cargo.toml
workspace: /zkvm_benchmarks/Cargo.toml
warning: sp1-host@0.1.0: rustc +succinct --version: "rustc 1.93.0-dev\n"
warning: sp1-host@0.1.0: sp1_xmss_benchmark built at 2026-04-18 21:11:40
    Finished `release` profile [optimized + debuginfo] target(s) in 0.26s
     Running `target/release/sp1-host --n-signatures 100 --max-segment-limit 23 prove`
Generating signatures (this will be cached for future runs)...
2026-04-18T21:14:13.235213Z  INFO initializing cuda prover
Running sp1-gpu-server 6.1.0 with device 0
Proving...
2026-04-18T21:14:18.977859Z  INFO sp1_gpu_server::server: Server listening @ /tmp/sp1-cuda-0.sock
2026-04-18T21:14:18.977935Z  INFO sp1_gpu_server::server: Connection accepted
2026-04-18T21:14:18.982181Z  INFO sp1_gpu_server::server: Running setup
2026-04-18T21:14:19.022246Z  INFO starting proof generation mode=Core
2026-04-18T21:14:19.023921Z  INFO sp1_gpu_server::server: Proving with mode: Core
Elapsed: 60552
Successfully generated proof!
Successfully verified proof!
2026-04-18T21:15:24.461151Z  INFO sp1_gpu_server::server: Destroying key
2026-04-18T21:15:24.466714Z  INFO sp1_gpu_server::server: Connection disconnected
