use common::{constants, generate_signatures};
use leansig::{
    serialization::Serializable
};
use methods::{RISC0_XMSS_BENCHMARK_ELF, RISC0_XMSS_BENCHMARK_ID};
use risc0_zkvm::{Executor, ExecutorEnv, ExecutorImpl, FileSegmentRef, ProverOpts, default_prover};
use risc0_custom;
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

    let opts = ProverOpts::succinct().with_receipt_kind(risc0_zkvm::ReceiptKind::Succinct);

    let prover = default_prover();
    let segment_dir = temp_dir().unwrap();

    let session = ExecutorImpl::from_elf(env, &RISC0_XMSS_BENCHMARK_ELF)
        .unwrap()
        .run_with_callback(|segment| {
            Ok(Box::new(FileSegmentRef::new(
                &segment,
                &segment_dir.path()
            )?))
        })
        .unwrap();

    println!("Segment count: {:?}", session.segments.len());

    let time = Instant::now();
    let receipt = session.prove().unwrap().receipt;
    println!("Elapsed: {}", time.elapsed().as_millis());

    println!("Proof size: {}", receipt.seal_size());
    receipt.verify(RISC0_XMSS_BENCHMARK_ID).unwrap();
    println!("Verification successful.");
}