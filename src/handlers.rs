use iron::prelude::*;
use iron::status;
use iron::{Handler};

use router::Router;

pub struct StatusCheckHandler;
pub struct SwapTokenHandler;
pub struct RefreshTokenHandler;

impl Handler for StatusCheckHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }
}

impl Handler for SwapTokenHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }
}

impl Handler for RefreshTokenHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }
}
