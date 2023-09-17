use rocket::serde::json::Json;
use crate::blockchain::Blockchain;
use crate::block::Block;
use std::sync::Mutex;

#[get("/last_block")]
pub fn get_last_block(blockchain: &rocket::State<Mutex<Blockchain>>) -> Json<Block> {
    let blockchain = blockchain.inner().lock().unwrap();
    let last_block = blockchain.blocks.last().unwrap().clone();
    Json(last_block)
}


#[post("/add_block")]
pub fn add_block(blockchain: &rocket::State<Mutex<Blockchain>>) -> Json<&'static str> {
    let mut blockchain = blockchain.inner().lock().unwrap();
    blockchain.add_block();
    Json("Bloque añadido")
}
