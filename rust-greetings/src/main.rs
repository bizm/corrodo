#![deny(warnings)]
extern crate hyper;
extern crate pretty_env_logger;
extern crate hostname;

use hyper::{Body, Request, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

fn main() {
    pretty_env_logger::init();
    let addr = ([0, 0, 0, 0], 8080).into();

    let server = Server::bind(&addr)
        .serve(|| {
            // This is the `Service` that will handle the connection.
            // `service_fn_ok` is a helper to convert a function that
            // returns a Response into a `Service`.
            service_fn_ok(move |_: Request<Body>| {
                Response::new(Body::from(
                    format!("Hi there! Greets from {:?}.",
                    hostname::get_hostname().unwrap_or_else(|| "unknown machine".to_string()))
                ))
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
