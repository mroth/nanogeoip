#[macro_use]
extern crate criterion;

use criterion::Criterion;
use hyper::{Body, Request, Response};

use std::sync::Arc;

fn criterion_benchmark(c: &mut Criterion) {
    let db = _init_reader();
    let db_ref = Arc::new(db);
    c.bench_function("http1", move |b| {
        b.iter(|| {
            _get("/89.160.20.112", &db_ref);
        })
    });
}

fn _req(path: &str) -> Request<Body> {
    Request::builder().uri(path).body(Body::empty()).unwrap()
}

fn _get(path: &str, db: &nanogeoip::Reader) -> Response<Body> {
    nanogeoip::lookup(_req(path), db, &nanogeoip::Options::default())
}

fn _init_reader() -> nanogeoip::Reader {
    nanogeoip::Reader::open("testdata/GeoIP2-City-Test.mmdb").unwrap()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
