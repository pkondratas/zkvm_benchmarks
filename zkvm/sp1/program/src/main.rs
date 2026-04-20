#![no_main]
sp1_zkvm::entrypoint!(main);

use leansig::serialization::Serializable;
use leansig::signature::{
    generalized_xmss::instantiations_poseidon::lifetime_2_to_the_18::target_sum::SIGTargetSumLifetime18W1NoOff,
    SignatureScheme,
};

pub fn main() {
    let pk_bytes = sp1_zkvm::io::read_vec();
    let epochs_bytes = sp1_zkvm::io::read_vec();
    let messages_bytes = sp1_zkvm::io::read_vec();
    let signatures_bytes = sp1_zkvm::io::read_vec();

    let pk = <SIGTargetSumLifetime18W1NoOff as SignatureScheme>::PublicKey::from_bytes(&pk_bytes).unwrap();
}