use common::{constants, generate_signatures, utils};
use leansig::{
    serialization::Serializable
};
use risc0_zkvm::host::client::env::SegmentPath;
use methods::{RISC0_XMSS_BENCHMARK_ELF, RISC0_XMSS_BENCHMARK_ID};
use risc0_zkvm::{Executor, ExecutorEnv, ExecutorImpl, FileSegmentRef, ProverOpts, VerifierContext, default_prover, get_prover_server};
use std::{env::temp_dir, time::Instant};
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

    let opts = ProverOpts::succinct();

    let prover = get_prover_server(&opts).unwrap();
    let ctx = VerifierContext::default();

    let segment_dir = temp_dir();

    let time = Instant::now();
    let session = ExecutorImpl::from_elf(env, &RISC0_XMSS_BENCHMARK_ELF)
        .unwrap()
        .run()
        .unwrap();
    println!("Execution time: {}", time.elapsed().as_millis());

    println!("Number of cycles: {}", session.user_cycles);

    let time = Instant::now();
    let receipt = prover.prove_session(&ctx, &session).unwrap().receipt;
    println!("Proving time: {}", time.elapsed().as_millis());

    let size = utils::get_proof_size(&receipt.inner.succinct().unwrap());
    println!("Proof size: {} bytes", size);

    let time = Instant::now();
    receipt.verify(RISC0_XMSS_BENCHMARK_ID).unwrap();
    println!("Verfication time: {}", time.elapsed().as_millis());
}