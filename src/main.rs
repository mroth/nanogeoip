use hyper::rt::Future;
use hyper::service::{service_fn_ok};
use hyper::{Server};

use std::sync::Arc;

fn main() {
    // TODO CLI parsing
    
    // TODO handle open error gracefully
    let db = tinygeoip::Reader::open("data/GeoLite2-City.mmdb").unwrap();

    let db_ref = Arc::new(db);
    let make_svc = move || {
        let mydb = Arc::clone(&db_ref);
        service_fn_ok(move |req| tinygeoip::lookup(req, &mydb))
    };

    let addr = ([127, 0, 0, 1], 9000).into();
    let server = Server::bind(&addr)
        .http1_pipeline_flush(true)
        .serve(make_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
