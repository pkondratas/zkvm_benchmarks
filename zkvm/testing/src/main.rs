use core::panic;

use common::signing_round::XmssSigningRound;
use leansig::signature::{
    generalized_xmss::instantiations_poseidon::lifetime_2_to_the_18::target_sum::SIGTargetSumLifetime18W1NoOff,
    SignatureScheme,
};
use risc0_zkvm::guest::env;

fn main() {
    // TODO: Implement your guest code here
    println!("Hello, zkVM!");

    // read the input
    let (public_key, signing_rounds): (<SIGTargetSumLifetime18W1NoOff as SignatureScheme>::PublicKey, Vec<XmssSigningRound<SIGTargetSumLifetime18W1NoOff>>) = env::read();
    println!("Received input");

    for round in signing_rounds {
        if SIGTargetSumLifetime18W1NoOff::verify(&public_key, round.epoch, &round.message, &round.signature) {
            panic!("Error");
        }
    }

    // TODO: do something with the input
    let output: u32 = 42;
    // write public output to the journal
    env::commit(&output);
}