use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::thread;
use std::sync::{Arc, Mutex};

struct SyncedMailbox {
    inner: Mutex<Vec<String>>,
}

impl SyncedMailbox {
    fn new() -> SyncedMailbox {
        let inner = Mutex::new(Vec::new());
        SyncedMailbox { inner: inner }
    }

    fn write(&self, message: String) {
        let mut vector = self.inner.lock().unwrap();
        vector.push(message);
    }

    fn read(&self) -> Option<String> {
        let mut vector = self.inner.lock().unwrap();
        vector.pop()
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7200").unwrap();

    let storage = Arc::new(SyncedMailbox::new());

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let local_storage = storage.clone();
                thread::spawn(move || {
                    handle(&mut s, &local_storage);
                });
            }
            Err(e) => {
                println!("A connection failed. Error: {}", e);
            }
        }
    }
}

fn handle(stream: &mut TcpStream, storage: &SyncedMailbox) {
    let message = read_message(stream);
    match message.trim() {
        "READ" => {
            handle_read(stream, storage);
        }
        _ => {
            handle_write(message, storage);
        }
    }
}

fn handle_read(stream: &mut TcpStream, storage: &SyncedMailbox) {
    let data = storage.read();

    match data {
        Some(message) => write!(stream, "{}", message),
        None => write!(stream, "No message in inbox!\n")
    }.ok().expect("Write failed!");
}

fn handle_write(message: String, storage: &SyncedMailbox) {
    storage.write(message);
}

fn read_message(stream: &mut TcpStream) -> String {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    let res = buffered_stream.read_line(&mut read_buffer);
    res.ok().expect("An error occured while reading!");
    read_buffer
}
