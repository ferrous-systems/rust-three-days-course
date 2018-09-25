extern crate redisish;
extern crate tokio;
extern crate tokio_io;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::read_to_end;
use tokio::io::write_all;

#[derive(Debug)]
enum ServerError {
    ParseError(redisish::Error),
    IoError(std::io::Error),
}

impl From<redisish::Error> for ServerError {
    fn from(e: redisish::Error) -> ServerError {
        ServerError::ParseError(e)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> ServerError {
        ServerError::IoError(e)
    }
}

fn main() -> std::io::Result<()> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listener = TcpListener::bind(&socket)?;

    let storage = VecDeque::new();
    let rced_storage: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(storage));

    let action = listener.incoming()
    .map_err(|error|{
        error.into()
    }).for_each(move |stream| {
        let storage_handle = rced_storage.clone();
    
        handle(stream, storage_handle)
    }).map_err(|error| {
        println!("An error occured: {:?}", error)
    });

    tokio::run(action);
    Ok(())
}

fn handle(stream: TcpStream, mutex: Arc<Mutex<VecDeque<String>>>) -> impl Future<Item=(), Error=ServerError> {
    let buffer = Vec::new();

    read_to_end(stream, buffer)
    .map_err(|e| e.into())
    .and_then(move |(stream, buffer)| {
        let input = String::from_utf8(buffer).unwrap();
        let command = redisish::parse(&input).unwrap();
    
        let output = match command {
            redisish::Command::Publish(message) => {
                let mut storage = mutex.lock().unwrap();
                storage.push_back(message);
                String::from("Message accepted")
            }
            redisish::Command::Retrieve => {
                let mut storage = mutex.lock().unwrap();
                let data = storage.pop_front();
                match data {
                    Some(message) => format!("{}", message),
                    None => format!("No message in inbox!\n")
                }
            }
        };

        write_all(stream, output.into_bytes())
            .map(|_| ())
            .map_err(|e| e.into())
    })

}