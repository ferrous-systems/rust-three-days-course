extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tcp_mailbox;

use tcp_mailbox::SyncedMailbox;

use std::io;
use std::str;
use std::sync::Arc;

use tokio_core::io::{Codec, EasyBuf};
use tokio_core::io::{Io, Framed};

use tokio_service::Service;
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;

use futures::{future, Future, BoxFuture};

pub enum Command {
    Get,
    Set(String)
}

pub enum GetSetResponse {
    Empty,
    Value(String),
    Stored
}

pub struct GetSetCodec;
pub struct GetSetProto;
pub struct MailboxService {
    mailbox: Arc<SyncedMailbox>
}

impl Codec for GetSetCodec {
    type In = Command;
    type Out = GetSetResponse;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            let input = match str::from_utf8(line.as_slice()) {
                Ok(s) => s,
                Err(_) => return Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            };

            let command_string = &input[0..3];

            match command_string {
                "GET" => Ok(Some(Command::Get)),
                "SET" => Ok(Some(Command::Set(input[4..].to_string()))),
                _ => Err(io::Error::new(io::ErrorKind::Other,
                                             "unknown command"))
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: GetSetResponse, buf: &mut Vec<u8>)
             -> io::Result<()>
    {
        let msg = match msg {
            GetSetResponse::Empty => "No message stored",
            GetSetResponse::Value(ref v) => v.as_ref(),
            GetSetResponse::Stored => "Stored message"
        };

        buf.extend(msg.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}


impl<T: Io + 'static> ServerProto<T> for GetSetProto {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = Command;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = GetSetResponse;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, GetSetCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(GetSetCodec))
    }
}

impl Service for MailboxService {
    // These types must match the corresponding protocol types:
    type Request = Command;
    type Response = GetSetResponse;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, command: Self::Request) -> Self::Future {
        let result = match command {
            Command::Get => {
                if let Some(result) = self.mailbox.read() {
                    GetSetResponse::Value(result)
                } else {
                    GetSetResponse::Empty
                }
            }
            Command::Set(s) => {
                self.mailbox.write(s);
                GetSetResponse::Stored
            }
        };

        future::ok(result).boxed()
    }
}

fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(GetSetProto, addr);

    let mailbox = Arc::new(SyncedMailbox::new());

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(move || Ok(MailboxService { mailbox: mailbox.clone() }));
}