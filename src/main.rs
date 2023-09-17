#[macro_use] extern crate rocket;

mod api;
mod blockchain;
mod block;
mod transaction;
mod pow;
mod p2p;

use api::blocks::{get_last_block, add_block};
use api::transactions::add_transaction;
use std::sync::Mutex;

#[launch]
fn rocket() -> _ {
    let initial_blockchain = Mutex::new(blockchain::Blockchain::new());

    rocket::build()
        .manage(initial_blockchain) 
        .mount("/", routes![get_last_block, add_transaction, add_block])
}
