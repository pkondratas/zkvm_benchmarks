use common::{constants, generate_signatures};
use sp1_sdk::{
    Elf, SP1Stdin, blocking::{Prover, ProverClient}, include_elf
};

const ELF: Elf = include_elf!("sp1_xmss_benchmark");

fn main() {
    std::env::set_var("SHARD_SIZE", "524288");
    std::env::set_var("SHARD_BATCH_SIZE", "1");
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    let input = generate_signatures::generate_and_cache_signatures(constants::N_SIGNATURES);

    let mut stdin = SP1Stdin::new();

    stdin.write(&input);

    let client = ProverClient::from_env();
    let pk = client.setup(ELF).expect("failed to setup elf");

    let (output, report) = client.execute(ELF, stdin.clone()).run().unwrap();
    println!("Number of cycles: {}", report.total_instruction_count());
    println!("PGUs: {}", report.gas().unwrap_or(0));

    // let proof = client
    //     .prove(&pk, stdin)
    //     .compressed()
    //     .run()
    //     .expect("failed to generate proof");

    // println!("Successfully generated proof!");

    // client
    //     .verify(&proof, pk.verifying_key(), None)
    //     .expect("failed to verify proof");
    
    // println!("Successfully verified proof!");
}