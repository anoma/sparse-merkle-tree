use crate::{traits::Hasher, H256};
use blake2b_rs::{Blake2b, Blake2bBuilder};

const BLAKE2B_KEY: &[u8] = &[];
const BLAKE2B_LEN: usize = 32;
const PERSONALIZATION: &[u8] = b"sparsemerkletree";

pub struct Blake2bHasher(Blake2b);

impl Default for Blake2bHasher {
    fn default() -> Self {
        let blake2b = Blake2bBuilder::new(BLAKE2B_LEN)
            .personal(PERSONALIZATION)
            .key(BLAKE2B_KEY)
            .build();
        Blake2bHasher(blake2b)
    }
}

impl Hasher for Blake2bHasher {
    fn write_bytes(&mut self, h: &[u8]) {
        self.0.update(h);
    }
    fn finish(self) -> H256 {
        let mut hash = [0u8; 32];
        self.0.finalize(&mut hash);
        hash.into()
    }
}
