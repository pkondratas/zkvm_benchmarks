use common::{constants, generate_signatures};
use leansig::{
    serialization::Serializable
};
use methods::{RISC0_XMSS_BENCHMARK_ELF, RISC0_XMSS_BENCHMARK_ID};
use risc0_zkvm::{ExecutorEnv, ProverOpts, default_prover};
use std::time::Instant;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 22)]
    max_segment_limit: u32,

    #[arg(long, default_value_t = 1)]
    n_signatures: usize
}

fn main() {
    // Set for development purposes
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RISC0_INFO", "1");

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let (public_key, signatures_rounds) =
        generate_signatures::generate_and_cache_signatures(args.n_signatures);

    let pk_bytes = public_key.to_bytes();

    let mut epochs_bytes: Vec<u8> = vec![];
    let mut messages_bytes: Vec<u8> = vec![];
    let mut signatures_bytes: Vec<u8> = vec![];

    signatures_rounds.iter().for_each(|s| {
        epochs_bytes.extend_from_slice(&s.epoch.to_le_bytes());
        messages_bytes.extend(s.message.to_bytes());
        signatures_bytes.extend(s.signature.to_bytes());
    });

    
    let env = ExecutorEnv::builder()
    // .write(&(public_key, signatures_rounds))
    // .unwrap()
    .write(&pk_bytes.len())
    .unwrap()
    .write(&epochs_bytes.len())
    .unwrap()
    .write(&messages_bytes.len())
    .unwrap()
    .write(&signatures_bytes.len())
    .unwrap()
    .write_slice(&pk_bytes)
    .write_slice(&epochs_bytes)
    .write_slice(&messages_bytes)
    .write_slice(&signatures_bytes)
    .segment_limit_po2(args.max_segment_limit)
    .build()
    .unwrap();

    let prover = default_prover();

    let time = Instant::now();
    let receipt = prover.prove(env, RISC0_XMSS_BENCHMARK_ELF).unwrap().receipt;
    println!("Elapsed: {}", time.elapsed().as_millis());

    println!("Proof size: {}", receipt.seal_size());
    receipt.verify(RISC0_XMSS_BENCHMARK_ID).unwrap();
    println!("Verification successful.");
}

// signatures_bytes len: 4880
// WARNING: proving in dev mode. This will not generate valid, secure proofs.
// Received input
// 2026-04-10T09:00:04.061526Z  INFO risc0_zkvm::host::server::exec::executor: execution time: 1.520434733s
// 2026-04-10T09:00:04.061540Z  INFO risc0_zkvm::host::server::session: number of segments: 78
// 2026-04-10T09:00:04.061542Z  INFO risc0_zkvm::host::server::session: 81264640 total cycles
// 81788928
// 2026-04-10T09:00:04.061544Z  INFO risc0_zkvm::host::server::session: 75143287 user cycles (92.47%)
// 2026-04-10T09:00:04.061546Z  INFO risc0_zkvm::host::server::session: 3869977 paging cycles (4.76%)
// 2026-04-10T09:00:04.061548Z  INFO risc0_zkvm::host::server::session: 2251376 reserved cycles (2.77%)
// 2026-04-10T09:00:04.061550Z  INFO risc0_zkvm::host::server::session: ecalls
// 2026-04-10T09:00:04.061552Z  INFO risc0_zkvm::host::server::session:    2536 Read calls, 7605 cycles, (0.01%)
// 2026-04-10T09:00:04.061554Z  INFO risc0_zkvm::host::server::session:    4 Sha2 calls, 296 cycles, (0.00%)
// 2026-04-10T09:00:04.061555Z  INFO risc0_zkvm::host::server::session:    1 Terminate calls, 2 cycles, (0.00%)
// 2026-04-10T09:00:04.061557Z  INFO risc0_zkvm::host::server::session:    0 Write calls, 0 cycles, (0.00%)
// 2026-04-10T09:00:04.061559Z  INFO risc0_zkvm::host::server::session:    0 User calls, 0 cycles, (0.00%)
// 2026-04-10T09:00:04.061560Z  INFO risc0_zkvm::host::server::session:    0 Poseidon2 calls, 0 cycles, (0.00%)
// 2026-04-10T09:00:04.061562Z  INFO risc0_zkvm::host::server::session:    0 BigInt calls, 0 cycles, (0.00%)
// 2026-04-10T09:00:04.061563Z  INFO risc0_zkvm::host::server::session: syscalls
// 2026-04-10T09:00:04.061565Z  INFO risc0_zkvm::host::server::session:    1265 Read calls
// 2026-04-10T09:00:04.061567Z  INFO risc0_zkvm::host::server::session:    1 Write calls
// 2026-04-10T09:00:04.061568Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity2 calls
// 2026-04-10T09:00:04.061570Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity calls
// 2026-04-10T09:00:04.061571Z  INFO risc0_zkvm::host::server::session:    0 ProveKeccak calls
// 2026-04-10T09:00:04.061573Z  INFO risc0_zkvm::host::server::session:    0 Keccak calls
// WARNING: Proving in dev mode does not generate a valid receipt. Receipts generated from this process are invalid and should never be used in production.

// 50 Senas
// WARNING: proving in dev mode. This will not generate valid, secure proofs.
// Received input
// 2026-04-10T09:11:54.193450Z  INFO risc0_zkvm::host::server::exec::executor: execution time: 32.544155132s
// 2026-04-10T09:11:54.193466Z  INFO risc0_zkvm::host::server::session: number of segments: 1568
// 2026-04-10T09:11:54.193469Z  INFO risc0_zkvm::host::server::session: 1643249664 total cycles
// 2026-04-10T09:11:54.193473Z  INFO risc0_zkvm::host::server::session: 1542919776 user cycles (93.89%)
// 2026-04-10T09:11:54.193477Z  INFO risc0_zkvm::host::server::session: 54706791 paging cycles (3.33%)
// 2026-04-10T09:11:54.193480Z  INFO risc0_zkvm::host::server::session: 45623097 reserved cycles (2.78%)
// 2026-04-10T09:11:54.193483Z  INFO risc0_zkvm::host::server::session: ecalls
// 2026-04-10T09:11:54.193486Z  INFO risc0_zkvm::host::server::session:    125232 Read calls, 375693 cycles, (0.02%)
// 2026-04-10T09:11:54.193490Z  INFO risc0_zkvm::host::server::session:    4 Sha2 calls, 296 cycles, (0.00%)
// 2026-04-10T09:11:54.193493Z  INFO risc0_zkvm::host::server::session:    1 Terminate calls, 2 cycles, (0.00%)
// 2026-04-10T09:11:54.193497Z  INFO risc0_zkvm::host::server::session:    0 Write calls, 0 cycles, (0.00%)
// 2026-04-10T09:11:54.193500Z  INFO risc0_zkvm::host::server::session:    0 User calls, 0 cycles, (0.00%)
// 2026-04-10T09:11:54.193503Z  INFO risc0_zkvm::host::server::session:    0 Poseidon2 calls, 0 cycles, (0.00%)
// 2026-04-10T09:11:54.193506Z  INFO risc0_zkvm::host::server::session:    0 BigInt calls, 0 cycles, (0.00%)
// 2026-04-10T09:11:54.193509Z  INFO risc0_zkvm::host::server::session: syscalls
// 2026-04-10T09:11:54.193512Z  INFO risc0_zkvm::host::server::session:    62613 Read calls
// 2026-04-10T09:11:54.193515Z  INFO risc0_zkvm::host::server::session:    1 Write calls
// 2026-04-10T09:11:54.193517Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity2 calls
// 2026-04-10T09:11:54.193520Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity calls
// 2026-04-10T09:11:54.193522Z  INFO risc0_zkvm::host::server::session:    0 ProveKeccak calls
// 2026-04-10T09:11:54.193525Z  INFO risc0_zkvm::host::server::session:    0 Keccak calls
// WARNING: Proving in dev mode does not generate a valid receipt. Receipts generated from this process are invalid and should never be used in production.
// Elapsed: 32651

// 50 naujas
// Loading signatures from cache...
// WARNING: proving in dev mode. This will not generate valid, secure proofs.
// Received input
// 2026-04-10T09:15:17.130946Z  INFO risc0_zkvm::host::server::exec::executor: execution time: 33.808326374s
// 2026-04-10T09:15:17.130963Z  INFO risc0_zkvm::host::server::session: number of segments: 1585
// 2026-04-10T09:15:17.130965Z  INFO risc0_zkvm::host::server::session: 1661468672 total cycles
// 2026-04-10T09:15:17.130966Z  INFO risc0_zkvm::host::server::session: 1559145340 user cycles (93.84%)
// 2026-04-10T09:15:17.130969Z  INFO risc0_zkvm::host::server::session: 56018541 paging cycles (3.37%)
// 2026-04-10T09:15:17.130970Z  INFO risc0_zkvm::host::server::session: 46304791 reserved cycles (2.79%)
// 2026-04-10T09:15:17.130972Z  INFO risc0_zkvm::host::server::session: ecalls
// 2026-04-10T09:15:17.130974Z  INFO risc0_zkvm::host::server::session:    508 Read calls, 16644 cycles, (0.00%)
// 2026-04-10T09:15:17.130976Z  INFO risc0_zkvm::host::server::session:    4 Sha2 calls, 296 cycles, (0.00%)
// 2026-04-10T09:15:17.130978Z  INFO risc0_zkvm::host::server::session:    1 Terminate calls, 2 cycles, (0.00%)
// 2026-04-10T09:15:17.130979Z  INFO risc0_zkvm::host::server::session:    0 Write calls, 0 cycles, (0.00%)
// 2026-04-10T09:15:17.130981Z  INFO risc0_zkvm::host::server::session:    0 User calls, 0 cycles, (0.00%)
// 2026-04-10T09:15:17.130982Z  INFO risc0_zkvm::host::server::session:    0 Poseidon2 calls, 0 cycles, (0.00%)
// 2026-04-10T09:15:17.130984Z  INFO risc0_zkvm::host::server::session:    0 BigInt calls, 0 cycles, (0.00%)
// 2026-04-10T09:15:17.130985Z  INFO risc0_zkvm::host::server::session: syscalls
// 2026-04-10T09:15:17.130987Z  INFO risc0_zkvm::host::server::session:    251 Read calls
// 2026-04-10T09:15:17.130988Z  INFO risc0_zkvm::host::server::session:    1 Write calls
// 2026-04-10T09:15:17.130990Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity2 calls
// 2026-04-10T09:15:17.130991Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity calls
// 2026-04-10T09:15:17.130993Z  INFO risc0_zkvm::host::server::session:    0 ProveKeccak calls
// 2026-04-10T09:15:17.130994Z  INFO risc0_zkvm::host::server::session:    0 Keccak calls
// WARNING: Proving in dev mode does not generate a valid receipt. Receipts generated from this process are invalid and should never be used in production.
// Elapsed: 33871
