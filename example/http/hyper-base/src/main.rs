extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::future::Future;
use futures::future::ok;
use futures::Stream;

use tokio_core::reactor::Core;

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

static INDEX: &'static [u8] = b"Try POSTing";

// Rename this to _ProxyService_ or _FileService_ or whatever
struct MyService;

impl Service for MyService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {

        match (req.method(), req.path()) {
            (&Get, "/") => {
                Box::new(ok(Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX)))
            },
            (&Post, _) => {
                unimplemented!();
            },
            _ => {
                Box::new(ok(Response::new()
                    .with_status(StatusCode::NotFound)))
            }
        }
    }

}


fn main() {
    let addr = "127.0.0.1:1337".parse().unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let service_constructor = move || Ok(MyService);

    let server = Http::new().serve_addr_handle(&addr, &handle, service_constructor).unwrap();

    let connection_handle = core.handle();

    handle.spawn(server.for_each(move |conn| {
        connection_handle.spawn(conn.map(|_| ()).map_err(|err| println!("srv1 error: {:?}", err)));
        Ok(())
    }).map_err(|_| ()));

    core.run(futures::future::empty::<(), ()>()).unwrap();
}