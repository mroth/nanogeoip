extern crate hyper;

use hyper::rt::Future;
use hyper::service::{service_fn_ok, Service};
use hyper::{Body, Request, Response, Server, StatusCode};

use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;

// struct IPLookupMicroservice {
//     cors: String,
//     db: Rc<tinygeoip::Reader>,
// }

// impl Service for IPLookupMicroservice {
//     type Request = Request;
//     type Response = Response;
//     type Error = hyper::Error;
//     type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

//     fn call(&self, req: Request) -> Self::Future {
//         // info!("Microservice received a request: {:?}", request);
//         Box::new(futures::future::ok(Response::new()))
//     }
// }

fn lookup(req: Request<Body>, db: &tinygeoip::Reader) -> Response<Body> {
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

    let results: tinygeoip::Record = match db.lookup(ip) {
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
    // Response::new(Body::from(payload))
}

fn main() {
    // TODO CLI parsing
    let db = tinygeoip::Reader::open("data/GeoLite2-City.mmdb").unwrap();
    // TODO handle error gracefully

    let db_ref = Arc::new(db);
    let new_svc = move || {
        let mydb = Arc::clone(&db_ref);
        service_fn_ok(move |req| lookup(req, &mydb))
    };

    let addr = ([127, 0, 0, 1], 9000).into();
    let server = Server::bind(&addr)
        .http1_pipeline_flush(true)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
