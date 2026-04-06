use common::{constants, generate_signatures};
use std::{time::Instant};
use methods::{RISC0_XMSS_BENCHMARK_ELF, RISC0_XMSS_BENCHMARK_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let input = generate_signatures::generate_and_cache_signatures(constants::N_SIGNATURES);

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let time = Instant::now();
    let receipt = prover
        .prove(env, RISC0_XMSS_BENCHMARK_ELF)
        .unwrap()
        .receipt;
    println!("Elapsed: {}", time.elapsed().as_millis());

    receipt.verify(RISC0_XMSS_BENCHMARK_ID).unwrap();
    println!("Verification successful.");
}
