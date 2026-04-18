use std::time::Instant;
use clap::{Parser, Subcommand};
use common::{generate_signatures};
use leansig::serialization::Serializable;
use sp1_core_executor::SP1CoreOpts;
use sp1_sdk::{
    CudaProver, Elf, ProveRequest, Prover, ProverClient, SP1Stdin, include_elf
};

#[derive(Subcommand, Debug)]
enum Command {
    Execute,
    Prove,
}

#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    // Command to either prove or execute
    command: Command,

    #[arg(long, default_value_t = 22)]
    max_segment_limit: u32,

    #[arg(long, default_value_t = 1)]
    n_signatures: usize
}

const ELF: Elf = include_elf!("sp1_xmss_benchmark");

async fn execute_xmss_verification(stdin: SP1Stdin, client: CudaProver) {
    println!("Executing...");

    let (_, report) = client.execute(ELF, stdin).await.unwrap();

    println!("Number of cycles: {}", report.total_instruction_count());
    println!("PGUs: {}", report.gas().unwrap_or(0));
}

async fn prove_xmss_verification(stdin: SP1Stdin, client: CudaProver) {
    println!("Proving...");

    let pk = client.setup(ELF).await.unwrap();

    let time = Instant::now();
    let proof = client.prove(&pk, stdin).compressed().await.unwrap();
    println!("Elapsed: {}", time.elapsed().as_millis());

    println!("Successfully generated proof!");

    client
        .verify(&proof, pk.verifying_key(), None)
        .expect("failed to verify proof");

    println!("Successfully verified proof!");
}

#[tokio::main]
async fn main() {
    sp1_sdk::utils::setup_logger();
    
    dotenv::dotenv().ok();
    
    let args = Args::parse();
    let max_shards_po2 = 1 << args.max_segment_limit;

    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("HEIGHT_THRESHOLD", format!("{}", max_shards_po2));

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

    let mut stdin = SP1Stdin::new();

    stdin.write_vec(pk_bytes);
    stdin.write_vec(epochs_bytes);
    stdin.write_vec(messages_bytes);
    stdin.write_vec(signatures_bytes);

    let opts = SP1CoreOpts::default();

    let client = ProverClient::builder()
        .cuda()
        .with_opts(opts)
        .build()
        .await;

    match args.command {
        Command::Execute => execute_xmss_verification(stdin, client).await,
        Command::Prove => prove_xmss_verification(stdin, client).await,
    }
}
