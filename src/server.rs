use std::io::Result;
use futures::prelude::*;
use tokio::net::TcpStream;

#[async]
pub fn on_message(_: TcpStream) -> Result<()> {
    println!("We got one!");
    Ok(())
}