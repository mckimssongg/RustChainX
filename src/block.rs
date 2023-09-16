use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u32, timestamp: u128, transactions: Vec<Transaction>, prev_hash: String) -> Self {
        Block {
            index,
            timestamp,
            transactions,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        }
    }
}
