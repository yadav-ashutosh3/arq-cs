#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{Rocket, State};
use rocket_contrib::json::Json;
use std::sync::Mutex;

// Import your Blockchain and related structs from blockchain.rs
mod blockchain;
use blockchain::{Blockchain, Transaction};

// Define a global state to hold your blockchain
struct BlockchainState {
    blockchain: Mutex<Blockchain>,
}

#[post("/transaction", data = "<transaction>")]
fn initiate_transaction(
    transaction: Json<Transaction>,
    blockchain_state: State<BlockchainState>,
) -> Json<bool> {
    let mut blockchain = blockchain_state.blockchain.lock().unwrap();
    
    // Save and validate the transaction data
    let result = blockchain.add_transaction(
        transaction.sender.clone(),
        transaction.receiver.clone(),
        transaction.amount,
    );

    Json(result)
}

// Add more routes for other functionalities like saving to the database and hashing data.

fn rocket() -> Rocket {
    rocket::ignite()
        .manage(BlockchainState {
            blockchain: Mutex::new(Blockchain::new()),
        })
        .mount("/", routes![initiate_transaction])
}

fn main() {
    rocket().launch();
}
