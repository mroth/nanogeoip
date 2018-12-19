use tinygeoip::{Country, Location, Reader, Record};

use std::net::IpAddr;

// location of the test data database
const TEST_DB_PATH: &str = "testdata/GeoIP2-City-Test.mmdb";

// These test cases are hard coded based on values present in the
// GeoIP2-City-Test test database.  If you override the DB and use a more up to
// date production dataset, they will likely fail due to changes in the data.
//
// We intentionally use a test database here for size reasons and to try to
// keep the tests "evergreen" for unit testing our DB parsing rather than the
// data itself.

#[test]
fn db_ipv4_lookup1() {
    _test_ip_lookup("89.160.20.112", ("SE", 58.4167, 15.6167, 76));
}

#[test]
fn db_ipv4_lookup2() {
    _test_ip_lookup("81.2.69.142", ("GB", 51.5142, -0.0931, 10));
}

#[test]
fn db_ipv6_lookup1() {
    _test_ip_lookup(
        "2001:218:85a3:0000:0000:8a2e:0370:7334",
        ("JP", 35.68536, 139.75309, 100),
    );
}

#[test]
fn db_ipv6_lookup2() {
    _test_ip_lookup("2001:220::1337", ("KR", 37.0, 127.5, 100));
}

fn _test_ip_lookup(ip_str: &str, expected: (&str, f64, f64, u16)) {
    // if we want to reduce test startup time in future, it will make sense to
    // wrap the reader in a lazy_static! to avoid re-opening DB every time
    let reader = Reader::open(TEST_DB_PATH).unwrap();
    let ip: IpAddr = ip_str.parse().unwrap();

    let (iso_code, lat, long, accuracy) = expected;
    let expect_results = Record {
        country: Country {
            iso_code: Some(iso_code.to_string()),
        },
        location: Location {
            latitude: Some(lat),
            longitude: Some(long),
            accuracy_radius: Some(accuracy),
        },
    };
    let results = reader.lookup(ip);
    assert_eq!(results, Ok(expect_results));
}
