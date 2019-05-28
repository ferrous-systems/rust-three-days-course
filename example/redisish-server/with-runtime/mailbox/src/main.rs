#![feature(async_await)]

use futures::prelude::*;
use runtime::net::{TcpListener, TcpStream};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum ServerError {
    ParseError(redisish::Error),
    IoError(std::io::Error),
    EncodingError(std::string::FromUtf8Error)
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

impl From<std::string::FromUtf8Error> for ServerError {
    fn from(e: std::string::FromUtf8Error) -> ServerError {
        ServerError::EncodingError(e)
    }
}

#[runtime::main]
async fn main() -> Result<(), ServerError> {
    let mut listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Listening on {}", listener.local_addr()?);

    let storage = VecDeque::new();
    let rced_storage = Arc::new(Mutex::new(storage));

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let storage = rced_storage.clone();
        runtime::spawn(async move {
            let mut stream = stream?;
            handle(&mut stream, &storage).await?;

            Ok::<(), ServerError>(())
        }).await?;
    }
    Ok(())
}

async fn handle<'a>(stream: &'a mut TcpStream, mutex: &'a Mutex<VecDeque<String>>) -> Result<(), ServerError> {
    let command = read_command(stream).await?;

    match command {
        redisish::Command::Publish(message) => {
            mutex.lock().unwrap().push_back(message);
            Ok(())
        }
        redisish::Command::Retrieve => {
            let data = mutex.lock().unwrap().pop_front();
            let output = match data {
                Some(message) => format!("{}", message),
                None => "No message in inbox!\n".to_string(),
            };
            stream.write_all(output.as_bytes()).await.map_err(|e| e.into())
        }
    }
}

async fn read_command(stream: &mut TcpStream) -> Result<redisish::Command, ServerError> {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;
    let s = String::from_utf8(buffer)?;
    redisish::parse(&s).map_err(|e| e.into())
}