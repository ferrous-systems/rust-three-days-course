extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;
use tokio_io::io::read_to_end;
use tokio_core::net::TcpStream;

use std::io::BufReader;
use std::io::BufRead;

use futures::{Stream, Future};

fn main() {
    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:8000".parse().unwrap();

    let handle = core.handle();

    let listener = TcpListener::bind(&addr, &handle).unwrap();
    
    let connection_handling = listener.incoming()
            .for_each(|(stream,_)| {
                let buffer: Vec<u8> = Vec::new();

                read_to_end(stream, buffer)
                    .and_then(|(_, input)| {
                        let message = String::from_utf8(input).unwrap();
                        println!("{}", message);
                        Ok(())
                    })
            });

    core.run(connection_handling).unwrap();
}
