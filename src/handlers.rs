use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::mime::Mime;

use hyper::Client;
use hyper::client::IntoUrl;
use hyper::header::{Authorization, Headers};
use hyper::status as hyper_status;

use std::io::Read;
use std::env;

use rustc_serialize::base64::{STANDARD, ToBase64};

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
        let body = req.get::<bodyparser::Json>();

        println!("{}", body);

        let client = Client::new();
        let headers = SwapTokenHandler::get_headers();
        let url = format!("{}/api/token", SPOTIFY_ACCOUNTS_ENDPOINT)
                      .into_url()
                      .unwrap();

        let mut res = client.post(url)
                            .headers(headers)
                            .send()
                            .unwrap();

        let mut response_body = String::new();

        res.read_to_string(&mut response_body).unwrap();

        let mime: Mime = "application/json".parse().unwrap();

        Ok(Response::with((res.status as status::Status, mime, response_body)))
    }
}

impl SwapTokenHandler {
    fn get_headers() -> Headers {
        let client_id = match env::var("CLIENT_ID") {
            Ok(client_id) => client_id,
            Err(_) => panic!("Missing CLIENT_ID"),
        };

        let client_secret = match env::var("CLIENT_SECRET") {
            Ok(client_secret) => client_secret,
            Err(_) => panic!("Missing CLIENT_ID"),
        };

        let basic = "Basic ".to_owned() + &client_id + ":" + &client_secret;
        let secret = basic.as_bytes().to_base64(STANDARD);
        let mut headers = Headers::new();
        headers.set(Authorization(secret));

        headers
    }
}

impl Handler for RefreshTokenHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((hyper_status::StatusCode::Ok, "Hello World!")))
    }
}

#[cfg(test)]
mod test {
    use iron::Headers;
    use iron_test::mock::request;
    use super::*;

    fn test_swap_token() {
        let url = "http://localhost:300/swap";
        let response = request::get(&url, Headers::new(), SwapTokenHandler);
        let result = extract_body(response);
        let expected_json = "";

        assert_eq!(result, expected_json);
    }
}
