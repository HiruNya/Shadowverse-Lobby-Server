#![feature(proc_macro, generators)]
extern crate futures_await as futures;
extern crate serde;
extern crate tokio;
extern crate tokio_io;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use futures::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

// All the networking code will be in this module
mod data;
mod parse;
mod server;

fn main() {
    // Initial Set Up
    let address = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&address).expect("Error: Unable to bind TCP Listener");
    let data = Arc::new(Mutex::new(data::GameData::new()));

    // Creating the server
    let server = listener
        .incoming()
        .map_err(|e| println!("Error: {:?}", e))
        .for_each(move |sock| {
            let game_data = data.clone();
            let task = async_block! {
                await!(server::on_message(sock, game_data));
                Ok(())
            };
            tokio::spawn(task);
            Ok(())
        });
    // Run the server
    tokio::run(server);
}
