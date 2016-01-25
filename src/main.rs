#![feature(plugin)]

extern crate bodyparser;
extern crate chrono;
extern crate dotenv;
extern crate hyper;
extern crate iron;
extern crate iron_test;
extern crate logger;
extern crate persistent;
extern crate router;
extern crate rustc_serialize;

use dotenv::dotenv;

use iron::prelude::*;
use logger::Logger;

use persistent::Read;

use router::Router;

mod handlers;
use handlers::*;

static MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    dotenv().ok();

    let mut router = Router::new();

    router.get("/", StatusCheckHandler);
    router.post("/swap", SwapTokenHandler);
    router.post("/refresh", RefreshTokenHandler);

    let mut chain = Chain::new(router);

    let (logger_before, logger_after) = Logger::new(None);

    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
