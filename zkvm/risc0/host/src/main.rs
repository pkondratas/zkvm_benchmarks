use common::signing_round::XmssSigningRound;
use leansig::signature::{
    generalized_xmss::instantiations_poseidon::lifetime_2_to_the_18::target_sum::SIGTargetSumLifetime18W1NoOff,
    SignatureScheme, SignatureSchemeSecretKey,
};
use std::{fs, path::Path, time::Instant};
use methods::{RISC0_XMSS_BENCHMARK_ELF, RISC0_XMSS_BENCHMARK_ID};
use rand::RngExt;
use bincode;
use risc0_zkvm::{default_prover, CompositeReceipt, ExecutorEnv, InnerReceipt, Receipt};
use serde::{Deserialize, Serialize};


const N_SIGNATURES: usize = 1;

fn generate_xmss_signatures<S: SignatureScheme>() -> (S::PublicKey, Vec<XmssSigningRound<S>>) {
    let mut rng = rand::rng();

    let (pk, sk) = S::key_gen(&mut rng, 0, S::LIFETIME as usize);
    let prepared_interval = sk.get_prepared_interval();

    let signing_rounds: Vec<XmssSigningRound<S>> = (0..N_SIGNATURES)
        .map(|_| {
            let message = rng.random();
            let epoch =
                rng.random_range(prepared_interval.start as u32..prepared_interval.end as u32);

            let signature = S::sign(&sk, epoch, &message).unwrap();

            XmssSigningRound::new(epoch, message, signature)
        })
        .collect();

    (pk, signing_rounds)
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let cache_path = Path::new("signatures_cache.bin");
    let input: (<SIGTargetSumLifetime18W1NoOff as SignatureScheme>::PublicKey, Vec<XmssSigningRound<SIGTargetSumLifetime18W1NoOff>>) =
        if cache_path.exists() {
            println!("Loading signatures from cache...");
            let bytes = fs::read(cache_path).expect("failed to read cache");
            bincode::deserialize(&bytes).expect("failed to deserialize cache")
        } else {
            println!("Generating signatures (this will be cached for future runs)...");
            let data = generate_xmss_signatures::<SIGTargetSumLifetime18W1NoOff>();
            let bytes = bincode::serialize(&data).expect("failed to serialize");
            fs::write(cache_path, &bytes).expect("failed to write cache");
            data
        };

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
