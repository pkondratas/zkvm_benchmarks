use common::signing_round::XmssSigningRound;
use common::constants;
use leansig::serialization::Serializable;
use leansig::signature::{
    generalized_xmss::instantiations_poseidon::lifetime_2_to_the_18::target_sum::SIGTargetSumLifetime18W1NoOff,
    SignatureScheme,
};
use risc0_zkvm::guest::env;


fn main() {
    println!("Received input");

    let pk_bytes_len: usize = env::read();
    let epochs_bytes_len: usize = env::read();
    let messages_bytes_len: usize = env::read();
    let signatures_bytes_len: usize = env::read();

    let mut pk_bytes = vec![0_u8; pk_bytes_len];
    let mut epochs_bytes = vec![0_u8; epochs_bytes_len];
    let mut messages_bytes = vec![0_u8; messages_bytes_len];
    let mut signatures_bytes = vec![0_u8; signatures_bytes_len];

    env::read_slice(&mut pk_bytes);
    env::read_slice(&mut epochs_bytes);
    env::read_slice(&mut messages_bytes);
    env::read_slice(&mut signatures_bytes);

    let pk = <SIGTargetSumLifetime18W1NoOff as SignatureScheme>::PublicKey::from_bytes(&pk_bytes).unwrap();

    signatures_bytes
        .chunks_exact(constants::SIGNATURE_LEN)
        .enumerate()
        .for_each(|(i, s)| {
            let signature = <SIGTargetSumLifetime18W1NoOff as SignatureScheme>::Signature::from_bytes(s).unwrap();

            let epochs_bytes_slice = &epochs_bytes[(i * constants::EPOCH_LEN)..((i + 1) * constants::EPOCH_LEN)];
            let message_bytes_slice = &messages_bytes[(i * constants::MESSAGE_LEN)..((i + 1) * constants::MESSAGE_LEN)];
            
            let epoch = u32::from_le_bytes(epochs_bytes_slice.try_into().unwrap());
            let message = <[u8; 32]>::from_bytes(message_bytes_slice).unwrap();
            SIGTargetSumLifetime18W1NoOff::verify(
                &pk, 
                epoch,
                &message,
                &signature
            );
        });
}
