use iron::prelude::*;
use iron::{Handler};
use iron::status;
use iron::headers::{ContentType};
use iron::mime::{Mime};

use hyper::{Client};
use hyper::client::IntoUrl;
use hyper::header::{Headers, Authorization};
use hyper::status as hyper_status;
use std::io::Read;

pub struct StatusCheckHandler;
pub struct SwapTokenHandler;
pub struct RefreshTokenHandler;

static SPOTIFY_ACCOUNTS_ENDPOINT: &'static str = "https://accounts.spotify.com";

impl Handler for StatusCheckHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((hyper_status::StatusCode::Ok, "Hello World!")))
    }
}

impl Handler for SwapTokenHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(Authorization("Basic".to_owned()));

        let url = format!("{}/api/token", SPOTIFY_ACCOUNTS_ENDPOINT)
            .into_url()
            .unwrap();

        let mut res = client.post(url)
            .headers(headers)
            .send()
            .unwrap();

        let mut body = String::new();

        res.read_to_string(&mut body).unwrap();

        let mime: Mime = "application/json".parse().unwrap();

        Ok(Response::with((res.status as status::Status, mime, body)))
    }
}

impl Handler for RefreshTokenHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((hyper_status::StatusCode::Ok, "Hello World!")))
    }
}
