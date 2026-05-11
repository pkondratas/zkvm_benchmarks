use common::{constants, generate_signatures};
use leansig::{
    serialization::Serializable
};
use methods::{RISC0_XMSS_BENCHMARK_ELF, RISC0_XMSS_BENCHMARK_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::time::Instant;

fn main() {
    // Set for development purposes
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RISC0_INFO", "1");
    std::env::set_var("RISC0_DEV_MODE", "0");

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let (public_key, signatures_rounds) =
        generate_signatures::generate_and_cache_signatures(constants::N_SIGNATURES);

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