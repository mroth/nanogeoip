#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate maxminddb;

use std::net::IpAddr;
use std::path::Path;
use maxminddb::MaxMindDBError;

// use maxminddb::geoip2;

#[derive(Deserialize, Serialize, Debug)]
pub struct Record {
    pub country: Country,
    pub location: Location,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Country {
    pub iso_code: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Location {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy_radius: Option<u16>,
}

pub struct Reader {
    db: maxminddb::OwnedReader<'static>,
}

impl Reader {
    pub fn open<P: AsRef<Path>>(database: P) -> Result<Reader, MaxMindDBError> {
        let reader = maxminddb::Reader::open(database)?;
        Ok(Reader { db: reader })
    }

    pub fn lookup(&self, ip: IpAddr) -> Result<Record, MaxMindDBError> {
        let results: Record = self.db.lookup(ip)?;
        Ok(results)
    }
}
