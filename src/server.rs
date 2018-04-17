/// All the networking code will be in this module
use futures::prelude::*;
use std::net::IpAddr::*;
use std::io::{BufReader, Result, Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use tokio::net::TcpStream;
use tokio::timer::Delay;
use tokio::prelude::*;
use tokio_io::io::{read_until, write_all, ReadHalf, WriteHalf};

use data::{GameData, Game};
use parse::{self, Request::*};
type Data = Arc<Mutex<GameData>>;
type Writer = WriteHalf<TcpStream>;
type Reader = ReadHalf<TcpStream>;

#[async]
pub fn on_message(sock: TcpStream, data: Data) -> Result<()> {
    let address = match sock.peer_addr() {
        Ok(e) => e.ip(),
        Err(_) => return Ok(()),
    };
    let address = match address {
        V4(i) => format!("{}", i),
        V6(i) => format!("{}", i),
    };
    let (reader, writer) = sock.split();
    //let (writer, _) = await!(write_all(writer, ""))?;
    let (reader, request) = match await!(read_message(reader)) {
        Ok(e) => e,
        Err(_) => {
            await!(write_all(writer, "Error! Incorrect request format"));
            return Ok(())
        },
    };
    match request {
        GetCache  => {await!(send_cache(writer, data));},
        UpdateGame(game) => {
            let previous_state = data.lock().unwrap().games.insert(address.clone(),
                       Game::new(game.name,
                                 game.author,
                                 game.join_code));
            data.lock().unwrap().update_cache();
            match previous_state {
                Some(_) => {},
                None => {
                    let time = Instant::now() + Duration::from_secs(30);
                    await!(Delay::new(time));
                    data.lock().unwrap().remove_game(&address);
                }
            }
        },
        RemoveGame => data.lock().unwrap().remove_game(&address),
    };
    Ok(())
}

#[async]
fn read_message(reader: Reader) -> Result<(Reader, parse::Request)> {
    let buf: Vec<u8> = Vec::new();
    let read = BufReader::new(reader);
    let (read, buf) = await!(read_until(read, b'\n', buf))?;
    let reader = read.into_inner();
    let response = match parse::get_request(buf.as_slice()) {
        Ok(res) => res,
        Err(_) => return Err(Error::new(ErrorKind::Other, "Couldn't parse request")),
    };
    Ok((reader, response))
}

#[async]
fn send_cache(writer: Writer, data: Data) -> Result<Writer> {
    let cache = data.lock().unwrap().cache.clone();
    let (writer, _) = await!(write_all(writer, cache)).expect("Could not send message");
    Ok(writer)
}
