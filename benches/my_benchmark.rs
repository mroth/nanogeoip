#[macro_use]
extern crate criterion;

use criterion::Criterion;

use std::net::IpAddr;
use std::str::FromStr;

fn criterion_benchmark(c: &mut Criterion) {
    let db = tinygeoip::Reader::open("data/GeoLite2-City.mmdb").unwrap();
    let ip: IpAddr = FromStr::from_str("89.160.20.112").unwrap();
    let mmdb = maxminddb::Reader::open("data/GeoLite2-City.mmdb").unwrap();

    c.bench_function("lookup", move |b| b.iter(|| db.lookup(ip)));
    c.bench_function("mmlook", move |b| {
        b.iter(|| {
            let _res: maxminddb::geoip2::City = mmdb.lookup(ip).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
