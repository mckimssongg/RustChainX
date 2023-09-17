use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;

/// Representa un bloque en la cadena de bloques.
///
/// # Campos
///
/// * `index` - El índice del bloque en la cadena.
/// * `timestamp` - La marca de tiempo del bloque.
/// * `transactions` - Las transacciones incluidas en el bloque.
/// * `prev_hash` - El hash del bloque anterior.
/// * `hash` - El hash del bloque actual.
/// * `nonce` - Un número que se usa una sola vez en la minería de criptomonedas.
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
