#[macro_use]
extern crate criterion;

use criterion::Criterion;

use std::net::IpAddr;
use std::str::FromStr;

const TESTDATA: &'static str = "testdata/GeoIP2-City-Test.mmdb";

// These benches don't do too much, but just verify that our minimal lookup
// function should be somewhat faster than a default maxminddb lookup for the
// entire geoip2::City struct which contains a bunch of fields.
fn criterion_benchmark(c: &mut Criterion) {
    let db = nanogeoip::Reader::open(TESTDATA).unwrap();
    let mmdb = maxminddb::Reader::open_readfile(TESTDATA).unwrap();
    let ip: IpAddr = FromStr::from_str("89.160.20.112").unwrap();

    c.bench_function("lookup", move |b| b.iter(|| db.lookup(ip)));
    c.bench_function("mmlook", move |b| {
        b.iter(|| {
            let _res: maxminddb::geoip2::City = mmdb.lookup(ip).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
