use std::{fs, path::Path};

use leansig::signature::{
    SignatureScheme, SignatureSchemeSecretKey, generalized_xmss::instantiations_poseidon::lifetime_2_to_the_18::target_sum::SIGTargetSumLifetime18W1NoOff,
};
use rand::RngExt;

use crate::signing_round::XmssSigningRound;


fn generate_xmss_signatures<S: SignatureScheme>(n: usize) -> (S::PublicKey, Vec<XmssSigningRound<S>>) {
    let mut rng = rand::rng();

    let (pk, sk) = S::key_gen(&mut rng, 0, S::LIFETIME as usize);
    let prepared_interval = sk.get_prepared_interval();

    let signing_rounds: Vec<XmssSigningRound<S>> = (0..n)
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

pub fn generate_and_cache_signatures(n: usize) -> (<SIGTargetSumLifetime18W1NoOff as SignatureScheme>::PublicKey, Vec<XmssSigningRound<SIGTargetSumLifetime18W1NoOff>>) {
    let cache_filename = format!("signatures_cache{n}.bin");
    let cache_path = Path::new(&cache_filename);
    
    if cache_path.exists() {
        println!("Loading signatures from cache...");

        let bytes = fs::read(cache_path).expect("failed to read cache");
        
        bincode::deserialize(&bytes).expect("failed to deserialize cache")
    } else {
        println!("Generating signatures (this will be cached for future runs)...");

        let data = generate_xmss_signatures::<SIGTargetSumLifetime18W1NoOff>(n);
        let bytes = bincode::serialize(&data).expect("failed to serialize");
        fs::write(cache_path, &bytes).expect("failed to write cache");

        data
    }
}
