extern crate hyper;

use super::db::{Reader, Record};

use hyper::{Body, Request, Response, StatusCode};

use std::net::IpAddr;
use std::str::FromStr;

pub fn hello(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("hello world"))
}

pub fn lookup(req: Request<Body>, db: &Reader) -> Response<Body> {
    let mut response = Response::builder();
    response.header("Content-Type", "application/json");

    let path = req.uri().path().trim_start_matches("/");
    // TODO: nice error msg on empty request

    let ip: IpAddr = match FromStr::from_str(path) {
        Ok(ip) => ip,
        Err(_e) => {
            let r = response
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(
                    r#"{"error": "could not parse invalid IP address"}"#,
                ))
                .unwrap();
            return r;
        }
    };

    let results: Record = match db.lookup(ip) {
        Ok(res) => res,
        Err(_e) => {
            // TODO: use value of e in msg?
            let r = response
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(r#"{"error": "something went horribly wrong"}"#))
                .unwrap();
            return r;
        }
    };

    let payload = serde_json::to_vec(&results).unwrap();
    response.body(Body::from(payload)).unwrap()
}
