use std::io::prelude::*;
use std::io;
use std::net::TcpStream;

fn main() -> Result<(), io::Error> {
    let message = std::env::args().nth(1).unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    write!(stream, "PUBLISH {}\n", message)?;
    stream.shutdown(std::net::Shutdown::Write)?;

    println!("Message published");

    Ok(())
}
