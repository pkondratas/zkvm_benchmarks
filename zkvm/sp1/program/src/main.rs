#![no_main]
sp1_zkvm::entrypoint!(main);

use common::constants;
use common::generate_signatures::Message;
use leansig::serialization::Serializable;
use leansig::signature::{
    generalized_xmss::instantiations_poseidon::lifetime_2_to_the_18::target_sum::SIGTargetSumLifetime18W1NoOff,
    SignatureScheme,
};

pub fn main() {
    let pk_bytes = sp1_zkvm::io::read_vec();
    let epochs_bytes = sp1_zkvm::io::read_vec();
    let message_bytes = sp1_zkvm::io::read_vec();
    let signatures_bytes = sp1_zkvm::io::read_vec();

    let pk = <SIGTargetSumLifetime18W1NoOff as SignatureScheme>::PublicKey::from_bytes(&pk_bytes).unwrap();
    let message = Message::from_bytes(&message_bytes).unwrap();

    signatures_bytes
        .chunks_exact(constants::SIGNATURE_LEN)
        .enumerate()
        .for_each(|(i, s)| {
            let signature = <SIGTargetSumLifetime18W1NoOff as SignatureScheme>::Signature::from_bytes(s).unwrap();
            let epochs_bytes_slice = &epochs_bytes[(i * constants::EPOCH_LEN)..((i + 1) * constants::EPOCH_LEN)];
            
            let epoch = u32::from_le_bytes(epochs_bytes_slice.try_into().unwrap());
            SIGTargetSumLifetime18W1NoOff::verify(
                &pk, 
                epoch,
                &message,
                &signature
            );
        });
}