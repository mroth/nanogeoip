use serde_derive::{Deserialize, Serialize};

use maxminddb::MaxMindDBError;
use std::net::IpAddr;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Record is a minimal set of information that is queried for and returned from
/// our lookups, consisting of a `Country` and `Location`.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Record {
    pub country: Country,
    pub location: Location,
}

/// Contains information about the country associated with an IP address.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Country {
    /// A two-character ISO 3166-1 country code for the country associated with
    /// the IP address.
    pub iso_code: Option<String>,
}

/// Contains information about the approximate geographic coordinates associated
/// with an IP address.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Location {
    /// The approximate latitude of the postal code, city, subdivision or
    /// country associated with the IP address.
    pub latitude: Option<f64>,
    /// The approximate longitude of the postal code, city, subdivision or
    /// country associated with the IP address.
    pub longitude: Option<f64>,
    /// The approximate accuracy radius, in kilometers, around the latitude and
    /// longitude for the geographical entity (country, subdivision, city or
    /// postal code) associated with the IP address. We have a 67% confidence
    /// that the location of the end-user falls within the area defined by the
    /// accuracy radius and the latitude and longitude coordinates.
    pub accuracy_radius: Option<u16>,
}

/// Reader essentially wraps a `maxminddb::Reader` to query for and retrieve our
/// minimal data structure only. By querying for less, lookups are faster.
pub struct Reader {
    db: maxminddb::Reader<Vec<u8>>,
    // timestamp for when we loaded the database
    load_ts: SystemTime,
    // timestamp as cached string
    load_tss: String,
}

impl Reader {
    /// opens a new DB reader.
    ///
    /// Argument must be the path to a valid maxmindDB file containing city precision.
    pub fn open<P: AsRef<Path>>(database: P) -> Result<Reader, MaxMindDBError> {
        let reader = maxminddb::Reader::open_readfile(database)?;
        let ts = SystemTime::now();
        Ok(Reader {
            db: reader,
            load_ts: ts,
            load_tss: httpdate::fmt_http_date(ts),
        })
    }

    /// lookup returns the results for a given IP address, or an error if
    /// results can not be obtained for some reason.
    pub fn lookup(&self, ip: IpAddr) -> Result<Record, MaxMindDBError> {
        let results: Record = self.db.lookup(ip)?;
        Ok(results)
    }

    // node count metadata of the underlying database
    pub fn node_count(&self) -> u32 {
        self.db.metadata.node_count
    }

    // timestamp metadata for when the underlying database originally built
    pub fn build_time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(self.db.metadata.build_epoch)
    }

    // timestamp for when the underlying database was loaded into memory
    pub fn load_time(&self) -> SystemTime {
        self.load_ts
    }

    // load_time() but as a cached HTTP date, suitable for a Last-Modified header
    //
    // TODO: This is not really where I'd like to stick this in the program, but
    // caching here for now until we can figure out how to get reliable
    // MakeService structs working in hyper.
    pub fn load_time_str(&self) -> &str {
        &self.load_tss
    }
}
