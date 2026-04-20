use leansig::serialization::Serializable;
use leansig::signature::{
    generalized_xmss::instantiations_poseidon::lifetime_2_to_the_18::target_sum::SIGTargetSumLifetime18W1NoOff,
    SignatureScheme,
};
use risc0_zkvm::guest::env;

fn main() {
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
}
