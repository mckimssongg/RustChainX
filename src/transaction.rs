use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

impl Transaction {
    #[allow(dead_code)]
    pub fn new(sender: String, receiver: String, amount: f32) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }
}
