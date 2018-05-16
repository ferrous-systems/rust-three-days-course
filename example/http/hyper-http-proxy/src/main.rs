extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::future::Future;
use futures::future::ok;
use futures::Stream;

use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};
use hyper::client;
use hyper::Method;

static INDEX: &'static [u8] = b"Try POST /echo";

struct Proxy {
    handle: Handle 
}

impl Service for Proxy {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Get, "/") | (&Get, "/echo") => {
                Box::new(ok(Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX)))
            },
            (&Post, "/post") => {
                let url = format!("http://httpbin.org/post").parse().unwrap();
                let handle = self.handle.clone();
                let client = client::Client::new(&handle);

                let mut request = client::Request::new(Method::Post, url);
                let length = req.headers().get::<ContentLength>().unwrap().clone();

                request.headers_mut().set(length);

                request.set_body(req.body());
                let f = client.request(request);

                Box::new(f)
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
    let service_handle = core.handle();

    let server = Http::new().serve_addr_handle(&addr, &handle, move || Ok(Proxy { handle: service_handle.clone() })).unwrap();

    let handle1 = handle.clone();
    handle.spawn(server.for_each(move |conn| {
        handle1.spawn(conn.map(|_| ()).map_err(|err| println!("srv1 error: {:?}", err)));
        Ok(())
    }).map_err(|_| ()));

    core.run(futures::future::empty::<(), ()>()).unwrap();
}