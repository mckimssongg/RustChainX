use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::Read;
use std::thread;
use serde_json::Value;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::block::Block;

pub struct Node {
    pub address: SocketAddr,
    pub blockchain: Blockchain,
}

impl Node {
    pub fn new(address: SocketAddr, blockchain: Blockchain) -> Self {
        Node { address, blockchain }
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind(&self.address).unwrap_or_else(|_| {
            eprintln!("Error al enlazar el socket TCP.");
            std::process::exit(1);
        });
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    let blockchain = self.blockchain.clone();
                    thread::spawn(move || {
                        handle_client(s, blockchain);
                    });
                }
                Err(e) => eprintln!("Error al recibir un nuevo TcpStream: {}", e),
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, mut blockchain: Blockchain) {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Error al leer del stream: {}", e);
        return;
    }
    let message: Result<Value, _> = serde_json::from_slice(&buffer);
    match message {
        Ok(msg) => {
            match msg["type"].as_str() {
                Some("new_block") => {
                    if let Ok(new_block) = serde_json::from_value::<Block>(msg["block"].clone()) {
                        let _transactions = new_block.transactions;
                        blockchain.add_block();
                    } else {
                        eprintln!("Error al deserializar el nuevo bloque.");
                    }
                },
                Some("transaction") => {
                    if let Ok(new_transaction) = serde_json::from_value::<Transaction>(msg["transaction"].clone()) {
                        blockchain.add_transaction(new_transaction);
                    } else {
                        eprintln!("Error al deserializar la nueva transacción.");
                    }
                },
                _ => {
                    eprintln!("Tipo de mensaje desconocido.");
                }
            }
        },
        Err(e) => {
            eprintln!("Error al deserializar el mensaje: {}", e);
        }
    }
}
