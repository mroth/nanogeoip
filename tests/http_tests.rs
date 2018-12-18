use tinygeoip;
use tinygeoip::Options;

use hyper::rt::{Future, Stream};
use hyper::{Response, Request, Body, StatusCode};

const TEST_IPV4_PATH1: &str = "/89.160.20.112";
// const TEST_IPV4_PATH2: &str = "/81.2.69.142";
const TEST_IPV6_PATH1: &str = "/2001:218:85a3:0000:0000:8a2e:0370:7334";
// const TEST_IPV6_PATH2: &str = "/2001:220::1337";

const TEST_IPV4_BODY1: &str = r#"{"country":{"iso_code":"SE"},"location":{"latitude":58.4167,"longitude":15.6167,"accuracy_radius":76}}"#;
// const TEST_IPV4_BODY2: &str = r#"{"country":{"iso_code":"GB"},"location":{"latitude":51.5142,"longitude":-0.0931,"accuracy_radius":10}}"#;
const TEST_IPV6_BODY1: &str = r#"{"country":{"iso_code":"JP"},"location":{"latitude":35.68536,"longitude":139.75309,"accuracy_radius":100}}"#;
// const TEST_IPV6_BODY2: &str = r#"{"country":{"iso_code":"KR"},"location":{"latitude":37,"longitude":127.5,"accuracy_radius":100}}"#;


#[test]
fn no_path() {
    let res = _quickget("/");
    assert_eq!(res.headers().get("Content-Type").unwrap(), "application/json");
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    assert_eq!(_bodystring(res), r#"{"error": "missing IP query in path, try /192.168.1.1"}"#)
}

#[test]
fn malformed_ip() {
    let res = _quickget("/192.168.aaa.bbb");
    assert_eq!(res.headers().get("Content-Type").unwrap(), "application/json");
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    assert_eq!(_bodystring(res), r#"{"error": "could not parse invalid IP address"}"#)
}

#[test]
fn ip_not_found() {
    let res = _quickget("/127.0.0.1");
    assert_eq!(res.headers().get("Content-Type").unwrap(), "application/json");
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(_bodystring(res), r#"{"error": "IP not found"}"#)
}

#[test]
fn happy_ipv4() {
    let res = _quickget(TEST_IPV4_PATH1);
    assert_eq!(res.headers().get("Content-Type").unwrap(), "application/json");
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(_bodystring(res), TEST_IPV4_BODY1);
}

#[test]
fn happy_ipv6() {
    let res = _quickget(TEST_IPV6_PATH1);
    assert_eq!(res.headers().get("Content-Type").unwrap(), "application/json");
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(_bodystring(res), TEST_IPV6_BODY1);
}

#[test]
fn cors_header() {
    let db = _init_reader();
    let res = tinygeoip::lookup(_req(TEST_IPV4_PATH1), &db, &Options::default());
    assert_eq!(res.headers().get("Access-Control-Allow-Origin").unwrap(), "*");
    let res = tinygeoip::lookup(_req(TEST_IPV4_PATH1), &db, &Options{cors_header: None});
    assert_eq!(res.headers().get("Access-Control-Allow-Origin"), None);
    let res = tinygeoip::lookup(_req(TEST_IPV4_PATH1), &db, &Options{cors_header: Some("*".to_string())});
    assert_eq!(res.headers().get("Access-Control-Allow-Origin").unwrap(), "*");
    let res = tinygeoip::lookup(_req(TEST_IPV4_PATH1), &db, &Options{cors_header: Some("https://foo.bar".to_string())});
    assert_eq!(res.headers().get("Access-Control-Allow-Origin").unwrap(), "https://foo.bar");
}


fn _req(path: &str) -> Request<Body> {
    Request::builder()
        .uri(path)
        .body(Body::empty())
        .unwrap()
}

fn _quickget(path: &str) -> Response<Body> {
    tinygeoip::lookup(_req(path), &_init_reader(), &Options::default())
}

// helper function to extract body text from a response
fn _bodystring(res: Response<Body>) -> String {
    res.into_body().map_err(|_| ()).fold(vec![], |mut acc, chunk| {
        acc.extend_from_slice(&chunk);
        Ok(acc)
    }).and_then(|v| String::from_utf8(v).map_err(|_| ())).wait().unwrap()
}

// TODO: load me once at beginning of tests instead, before teardown
fn _init_reader() -> tinygeoip::Reader {
    tinygeoip::Reader::open("testdata/GeoIP2-City-Test.mmdb").unwrap()
}
