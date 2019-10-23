use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
fn main() -> Result<(), io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    write!(stream, "RETRIEVE\n")?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut message = String::new();
    stream.read_to_string(&mut message)?;

    println!("Message: {}", message);

    Ok(())
}