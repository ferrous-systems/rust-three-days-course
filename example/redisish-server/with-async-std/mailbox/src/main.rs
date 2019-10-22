use async_std::prelude::*;
use async_std::task;
use async_std::sync::{Mutex};
use async_std::net::{TcpListener, TcpStream};
use std::collections::VecDeque;
use std::sync::Arc;

type Error =  Box<dyn std::error::Error + Send + Sync>;
fn main() -> Result<(), Error> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:7878").await?;
        println!("Listening on {}", listener.local_addr()?);

        let storage = VecDeque::new();
        let rced_storage = Arc::new(Mutex::new(storage));

        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            let storage = rced_storage.clone();
            task::spawn(async move {
                handle(stream?, &storage).await?;

                Ok::<(), Error>(())
            }).await?;
        }
        Ok(())
    })
}

async fn handle(mut stream: TcpStream, mutex: &Mutex<VecDeque<String>>) -> Result<(), Error> {
    let command = read_command(&mut stream).await?;

    match command {
        redisish::Command::Publish(message) => {
            mutex.lock().await.push_back(message);
            Ok(())
        }
        redisish::Command::Retrieve => {
            let data = mutex.lock().await.pop_front();
            let output = match data {
                Some(message) => format!("{}", message),
                None => "No message in inbox!\n".to_string(),
            };
            stream.write_all(output.as_bytes()).await.map_err(|e| e.into())
        }
    }
}

async fn read_command(stream: &mut TcpStream) -> Result<redisish::Command, Error> {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;
    let s = String::from_utf8(buffer)?;
    redisish::parse(&s).map_err(|e| e.into())
}