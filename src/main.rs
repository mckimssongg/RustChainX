mod blockchain;
mod block;
mod transaction;
mod pow;
mod p2p;

use std::net::SocketAddr;
use p2p::Node;
use blockchain::Blockchain;

fn main() {
    let initial_blockchain = Blockchain::new();
    let address = "127.0.0.1:8080".parse::<SocketAddr>().unwrap_or_else(|_| {
        eprintln!("Error al analizar la dirección del socket.");
        std::process::exit(1);
    });
    let mut node = Node::new(address, initial_blockchain);
    node.start();
}
