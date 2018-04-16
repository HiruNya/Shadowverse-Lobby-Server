/// All the networking code willbe in this module

use std::io::Result;
use std::sync::{Arc, Mutex};
use futures::prelude::*;
use tokio::net::TcpStream;

use data::GameData;
type Data = Arc<Mutex<GameData>>;

#[async]
pub fn on_message(_: TcpStream, _: Data) -> Result<()> {
    println!("We got one!");
    Ok(())
}