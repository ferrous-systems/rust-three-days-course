extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate tokio_core;

use std::rc::Rc;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use futures::future::Future;
use futures::future::ok;
use futures::Stream;

use futures_cpupool::CpuPool;

use tokio_core::reactor::Core;

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

static INDEX: &'static [u8] = b"Try POST /hello";

struct Proxy {
    pool: Rc<CpuPool>
}

impl Service for Proxy {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let (method, uri, _, _, body) = req.deconstruct();

        match (method, uri.path()) {
            (Get, "/") | (Get, "/echo") => {
                Box::new(ok(Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX)))
            },
            (Post, _) => {
                let f = self.pool.spawn_fn(move || {
                    let p = Path::new(uri.path());
                    let mut f = File::create(p.components().last().unwrap().as_os_str()).unwrap();

                    body.for_each(move |chunk| {
                        f.write(chunk.as_ref());

                        Ok(())
                    })
                }).and_then(|_| {
                    Box::new(ok(Response::new()
                        .with_status(StatusCode::Ok)))
                });

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

    let pool = Rc::new(CpuPool::new(4));

    let server = Http::new().serve_addr_handle(&addr, &handle, move || Ok(Proxy { pool: pool.clone() })).unwrap();

    let handle1 = handle.clone();
    handle.spawn(server.for_each(move |conn| {
        handle1.spawn(conn.map(|_| ()).map_err(|err| println!("srv1 error: {:?}", err)));
        Ok(())
    }).map_err(|_| ()));

    core.run(futures::future::empty::<(), ()>()).unwrap();
}