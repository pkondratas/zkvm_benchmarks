#![no_main]
sp1_zkvm::entrypoint!(main);

use common::signing_round::XmssSigningRound;
use leansig::signature::{
    generalized_xmss::instantiations_poseidon::lifetime_2_to_the_18::target_sum::SIGTargetSumLifetime18W1NoOff,
    SignatureScheme,
};

pub fn main() {
    let (public_key, signing_rounds) = sp1_zkvm::io::read::<(<SIGTargetSumLifetime18W1NoOff as SignatureScheme>::PublicKey, Vec<XmssSigningRound<SIGTargetSumLifetime18W1NoOff>>)>();

    for round in signing_rounds {
        if !SIGTargetSumLifetime18W1NoOff::verify(&public_key, round.epoch, &round.message, &round.signature) {
            panic!("Error");
        }
    }
}