extern crate hyper;

use super::db::{Reader, Record};

use hyper::{Body, Request, Response, StatusCode};

use std::net::IpAddr;
use std::str::FromStr;

pub struct Options {
    pub cors_header: Option<String>,
}

impl Default for Options {
    fn default() -> Options {
        Options { cors_header: Some("*".to_string()) }
    }
}

fn err_json(msg: &str) -> Body {
    Body::from(format!("{{\"error\": \"{}\"}}", msg))
}

pub fn hello(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("hello world"))
}

pub fn lookup(req: Request<Body>, db: &Reader, opts: &Options) -> Response<Body> {
    let mut response = Response::builder();
    response.header("Content-Type", "application/json");
    
    if let Some(ref rule) = opts.cors_header {
        response.header("Access-Control-Allow-Origin", rule.to_owned());
    }

    let path = req.uri().path().trim_start_matches("/");
    if path == "" {
        return response.status(StatusCode::BAD_REQUEST)
            .body(err_json("missing IP query in path, try /192.168.1.1"))
            .unwrap()
    }

    let ip: IpAddr = match FromStr::from_str(path) {
        Ok(ip) => ip,
        Err(_e) => {
            return response.status(StatusCode::BAD_REQUEST)
                .body(err_json("could not parse invalid IP address"))
                .unwrap();
        }
    };

    let results: Record = match db.lookup(ip) {
        Ok(res) => res,
        Err(_e) => {
            // native MaxMindDBError(String) is "error while decoding value"
            return response.status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(err_json("IP not found"))
                .unwrap();
        }
    };

    let payload = serde_json::to_vec(&results).unwrap();
    response.body(Body::from(payload)).unwrap()
}
