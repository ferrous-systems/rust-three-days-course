extern crate tokio;
extern crate futures;

use std::net::SocketAddr;
use std::collections::VecDeque;

use tokio::net::{TcpListener,TcpStream};
use tokio::prelude::*;
use tokio::io::{read_to_end, write_all};

use futures::sync::mpsc;
use futures::sync::oneshot;

struct StorageService {
    receiver: mpsc::Receiver<(String, oneshot::Sender<String>)>,
    vec: VecDeque<String>,
}

impl StorageService {
    fn run(self) -> impl Future<Item=(), Error=()> {
        let StorageService { receiver, mut vec } = self;

        receiver.for_each(move |(message, backchannel)| {
            vec.push_back(message);

            backchannel.send(String::from("Message confirmed"))
                .map_err(|e| {
                    println!("Got error: {:?}", e);
                })
                .map(|_| ())
        })
    }
}

fn main() {
    let socket: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&socket).unwrap();

    let vec = VecDeque::new();

    let (sender, receiver) = mpsc::channel::<(String, oneshot::Sender<String>)>(10);
    
    let storage_service = StorageService { vec, receiver };

    let system_future = listener.incoming()
        .map_err(|e| {
            println!("Got error: {:?}", e);
        })
        .for_each(move |stream|  {
            println!("Got connection!");
            
            let sender_handle = sender.clone();
            let (backchannel_sender, backchannel_receiver) = oneshot::channel::<String>();
        
            read_stream(stream)
                .map_err(|e| {
                    println!("Error reading: {:?}", e);
                })
                .and_then(move |(stream, string)| {
                    sender_handle.send((string, backchannel_sender))
                        .map_err(|e| {
                            println!("Error sending: {:?}", e);
                        }).and_then(move |_| {
                            backchannel_receiver
                                .map_err(|e| {
                                    println!("Error receiving from service: {:?}", e);
                                })
                                .and_then(|string| {
                                    write_all(stream, string.into_bytes())
                                        .map_err(|e| {
                                            println!("Error writing back: {:?}", e);
                                        }).map(|_| ())
                                })
                        })
                })
        });

    let service_future = storage_service.run();

    let (shutdown_sender, shutdown_receiver) = oneshot::channel::<()>();
    
    let shutdown_future = shutdown_receiver
                          .then(|_| Ok(()));

    let mut futures: Vec<Box<dyn Future<Item=(), Error=()> + Send>> = Vec::new();
    futures.push(Box::new(system_future));
    futures.push(Box::new(service_future));
    futures.push(Box::new(shutdown_future));

    let select_future = futures::future::select_all(futures).then(|_| Ok(()));

    tokio::run(select_future);
}

fn read_stream(stream: TcpStream) -> impl Future<Item=(TcpStream, String), Error=std::io::Error> {
    let buf = Vec::new();

    read_to_end(stream, buf)
        .and_then(|(stream, buf)| {
            let string = String::from_utf8(buf).unwrap();

            Ok((stream, string))
        })
}