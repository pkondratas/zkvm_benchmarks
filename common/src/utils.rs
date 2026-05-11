use serde::Serialize;

pub fn get_proof_size<T: Serialize>(item: &T) -> usize {
    bincode::serialized_size(item).unwrap() as usize
}