extern crate hyper;
extern crate maxminddb;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod db;
pub use self::db::Reader;
pub use self::db::Record;
pub use self::db::Country;
pub use self::db::Location;

mod http;
pub use self::http::hello;
pub use self::http::lookup;
