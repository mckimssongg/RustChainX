use crate::block::Block;
use sha2::{Sha256, Digest};
const DIFFICULTY: usize = 4;

pub fn proof_of_work(block: &Block) -> u64 {
    let mut nonce = 0;
    loop {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            block.index,
            block.timestamp,
            block.prev_hash,
            block.nonce,
            block.transactions.len()
        ));
        let hash = format!("{:x}", hasher.finalize_reset());
        if &hash[0..DIFFICULTY] == "0".repeat(DIFFICULTY).as_str() {
            return nonce;
        }
        nonce += 1;
    }
}
