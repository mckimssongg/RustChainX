use rocket::serde::json::Json;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use std::sync::Mutex;

#[post("/transaction", format = "json", data = "<transaction>")]
pub fn add_transaction(blockchain: &rocket::State<Mutex<Blockchain>>, transaction: Json<Transaction>) -> Json<&'static str> {
    let mut blockchain = blockchain.inner().lock().unwrap();
    blockchain.add_transaction(transaction.0);

    if blockchain.pending_transactions.len() >= 10 { 
        blockchain.add_block();
    }

    Json("Transacción añadida")
}