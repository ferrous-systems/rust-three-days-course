extern crate redisish;

use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::VecDeque;
use std::sync::{Mutex,Arc};

struct SyncedMailbox<T> {
    inner: Mutex<VecDeque<T>>
}

impl<T> SyncedMailbox<T> {
    fn new() -> Self {
        SyncedMailbox { inner: Mutex::new(VecDeque::new()) }
    }
}

trait Storage<T> {
    fn add_message(&self, message: T) -> Result<(), ServerError>;
    fn retrieve_message(&self) -> Result<Option<T>, ServerError>;
}

impl<T> Storage<T> for SyncedMailbox<T> {
    fn add_message(&self, message: T) -> Result<(), ServerError> {
        self.inner.lock().map( |mut guard| {
            guard.push_back(message)
        }).map_err( |err| err.into() )
    }

    fn retrieve_message(&self) -> Result<Option<T>, ServerError> {
        self.inner.lock().map( |mut guard| {
            guard.pop_front()
        }).map_err( |err| err.into() )
    }
}

#[derive(Debug)]
enum ServerError {
    ParseError(redisish::Error),
    IoError(std::io::Error),
    LockError
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

impl<T> From<std::sync::PoisonError<T>> for ServerError {
    fn from(_: std::sync::PoisonError<T>) -> ServerError {
        ServerError::LockError
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let storage = Arc::new(SyncedMailbox::new());

    for stream in listener.incoming() {
        let storage_handle = storage.clone();

        std::thread::spawn(move || {
            let res = stream.map_err(|e| e.into() )
                    .and_then(|mut s| {
                        handle(&mut s, storage_handle.as_ref())
                    });
        
            if let Err(e) = res {
                println!("Error occured: {:?}", e);
            }
        });
    }
}

fn handle<S: Storage<String>>(stream: &mut TcpStream, storage: &S) -> Result<(), ServerError> {
    let command = read_command(stream)?;
    match command {
        redisish::Command::Publish(message) => {
            storage.add_message(message).map_err(|e| e.into())
        }
        redisish::Command::Retrieve => {
            let data = storage.retrieve_message()?;
            match data {
                Some(message) => write!(stream, "{}", message).map_err( |e| e.into() ),
                None => write!(stream, "No message in inbox!\n").map_err( |e| e.into() )
            }
        }
    }
}

fn read_command(stream: &mut TcpStream) -> Result<redisish::Command, ServerError> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream.read_line(&mut read_buffer)?;
    redisish::parse(&read_buffer).map_err( |e| e.into() )
}