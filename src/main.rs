#![feature(plugin)]

extern crate chrono;
extern crate iron;
extern crate logger;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use logger::Logger;

use router::Router;

mod handlers;
use handlers::*;

fn main() {
    let mut router = Router::new();

    router.get("/", StatusCheckHandler);
    router.post("/swap", SwapTokenHandler);
    router.post("/refresh", RefreshTokenHandler);

    let mut chain = Chain::new(router);

    let (logger_before, logger_after) = Logger::new(None);

    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
