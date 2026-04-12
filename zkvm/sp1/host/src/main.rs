use clap::{Parser, Subcommand};
use common::{constants, generate_signatures};
use sp1_sdk::{
    Elf, ProvingKey, SP1Stdin, blocking::{EnvProver, ProveRequest, Prover, ProverClient}, include_elf
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
}

const ELF: Elf = include_elf!("sp1_xmss_benchmark");

fn execute_xmss_verification(stdin: SP1Stdin, client: EnvProver) {
    println!("Executing...");

    let (_, report) = client.execute(ELF, stdin).run().unwrap();

    println!("Number of cycles: {}", report.total_instruction_count());
    println!("PGUs: {}", report.gas().unwrap_or(0));
}

fn prove_xmss_verification(stdin: SP1Stdin, client: EnvProver) {
    println!("Proving...");

    let pk = client.setup(ELF).expect("failed to setup elf");

    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");

    println!("Successfully generated proof!");

    client
        .verify(&proof, pk.verifying_key(), None)
        .expect("failed to verify proof");

    println!("Successfully verified proof!");
}

fn main() {
    std::env::set_var("SHARD_SIZE", "524288");
    std::env::set_var("SHARD_BATCH_SIZE", "1");
    std::env::set_var("RUST_LOG", "info");

    sp1_sdk::utils::setup_logger();

    dotenv::dotenv().ok();

    let args = Args::parse();

    let input = generate_signatures::generate_and_cache_signatures(constants::N_SIGNATURES);

    let mut stdin = SP1Stdin::new();

    stdin.write(&input);
    let client = ProverClient::from_env();

    match args.command {
        Command::Execute => execute_xmss_verification(stdin, client),
        Command::Prove => prove_xmss_verification(stdin, client)
    }
}