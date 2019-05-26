#![feature(async_await)]

use futures::prelude::*;
use runtime::net::TcpListener;

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Listening on {}", listener.local_addr()?);

    // accept connections and process them in parallel
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        runtime::spawn(async move {
            let mut stream = stream?;
            let mut buffer = Vec::new();
            println!("Accepting from: {}", stream.peer_addr()?);

            stream.read_to_end(&mut buffer).await?;
            let s = String::from_utf8(buffer).unwrap();

            let parsed = redisish::parse(&s).unwrap();
            println!("{:?}", parsed);
            
            Ok::<(), std::io::Error>(())
        })
        .await?;
    }
    Ok(())
}
