use leansig::signature::SignatureScheme;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct XmssSigningRound<S: SignatureScheme> {
    pub epoch: u32,
    pub signature: S::Signature,
}

impl<S: SignatureScheme> XmssSigningRound<S> {
    pub fn new(epoch: u32, signature: S::Signature) -> Self {
        Self { epoch, signature: signature }
    }
}