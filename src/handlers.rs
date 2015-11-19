use iron::prelude::*;
use iron::status;
use iron::{Handler};

use hyper::{Client};
use hyper::header::{Headers, Authorization};
use hyper::status::{StatusCode};

pub struct StatusCheckHandler;
pub struct SwapTokenHandler;
pub struct RefreshTokenHandler;

static SPOTIFY_ACCOUNTS_ENDPOINT: &'static str = "https://accounts.spotify.com";

impl Handler for StatusCheckHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }
}

impl Handler for SwapTokenHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(Authorization("Basic".to_owned()));

        let res = client.post(SPOTIFY_ACCOUNTS_ENDPOINT)
            .headers(headers)
            .send()
            .unwrap();

        match res.status {
            StatusCode::Ok => {
                println!("Success: {}", res.status);
            }

            _ => {
                println!("Error");
            }
        }

        Ok(Response::with((res.status, "Hello World!")));
    }
}

impl Handler for RefreshTokenHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }
}
