use crate::transaction::Transaction;
use crate::pow::proof_of_work;
use crate::block::Block;
use std::time::SystemTime;

#[derive(Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            0,
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(),
            vec![],
            String::from("0")
        );
        let mut blocks = Vec::new();
        blocks.push(genesis_block);
        Blockchain { blocks, pending_transactions: Vec::new() }
    }

    pub fn add_block(&mut self) {
        let prev_block = self.blocks.last().unwrap().clone();
        let index = prev_block.index + 1;
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        let prev_hash = prev_block.hash.clone();
        let mut block = Block::new(index, timestamp, self.pending_transactions.clone(), prev_hash);
        block.nonce = proof_of_work(&block);
        self.blocks.push(block);
        self.pending_transactions.clear();
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }
}
