#![feature(proc_macro, generators)]
extern crate tokio;
extern crate futures_await as futures;

use tokio::prelude::*;
use tokio::net::TcpListener;
use futures::prelude::*;

mod server;

fn main() {
    // Initial Set Up
    let address = "127.0.0.1:8080".parse()
        .unwrap();
    let listener = TcpListener::bind(&address)
        .expect("Error: Unable to bind TCP Listener");

    // Creating the server
    let server = listener.incoming()
        .map_err(|e| println!("Error: {:?}", e))
        .for_each(|sock| {
            let task = async_block! {
                await!(server::on_message(sock));
                Ok(())
            };
            tokio::spawn(task);
            Ok(())
        });
    tokio::run(server);
}
