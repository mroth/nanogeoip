#[macro_use]
extern crate clap;
use clap::{App, Arg};

use httpdate;
use hyper::rt::Future;
use hyper::Server;
use nanogeoip::{Options, Reader};

use std::net::SocketAddr;
use std::process;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about("A tiny (and blazing fast!) geoip lookup microservice")
        .after_help("For more information see https://github.com/mroth/nanogeoip")
        .arg(
            Arg::with_name("db")
                .index(1)
                .help("MaxMind database file")
                .default_value("data/GeoLite2-City.mmdb")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .short("a")
                .help("Address to listen for connections on")
                .default_value("0.0.0.0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .help("Port to listen for connections on")
                .default_value("9000")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cors")
                .short("o")
                .value_name("origin")
                .help("'Access-Control-Allow-Origin' header")
                .default_value("*")
                .takes_value(true),
        )
        // .arg(
        //     Arg::with_name("v")
        //         .short("v")
        //         .multiple(true)
        //         .help("Sets the level of verbosity"),
        // )
        .get_matches();

    // CLI: handle parsing the SocketAddr and associated syntax errors
    let socket_str = format!(
        "{}:{}",
        matches.value_of("addr").unwrap(),
        matches.value_of("port").unwrap()
    );
    let addr: SocketAddr = match socket_str.parse() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("FATAL: {} {}", socket_str, e);
            process::exit(1);
        }
    };

    // CLI: handle parsing the CORS value, treating an explicit "" as None
    let cors_raw = matches.value_of("cors").unwrap(); //safe bc default val
    let cors = match cors_raw {
        "" => None,
        _ => Some(cors_raw.to_string()),
    };
    let opts = Options { cors_header: cors };

    // Handle trying to load database from user-supplied or default path
    let db_path = matches.value_of("db").unwrap(); //safe bc default val
    let db = match Reader::open(db_path) {
        Ok(val) => {
            println!(
                "Loaded database {}: {} nodes, built {}",
                db_path,
                val.node_count(),
                httpdate::fmt_http_date(val.build_time())
            );
            val
        }
        Err(e) => {
            // default error "error while decoding value" not very helpful
            eprintln!("Failed to load a GeoIP database from {}: {}", db_path, e);
            process::exit(1);
        }
    };

    // Construct our Hyper MakeService using the built-in functions
    //
    // I'd really prefer to have a struct encapsulating that implements
    // MakeService but have not been able to get that working yet.

    // let (db_ref, opts_ref) = (Arc::new(db), Arc::new(opts));
    // let make_svc = move || {
    //     let (svc_db, svc_opts) = (db_ref.clone(), opts_ref.clone());
    //     service_fn_ok(move |req| nanogeoip::lookup(req, &svc_db, &svc_opts))
    // };

    // nope
    let make_svc = nanogeoip::service::MakeLookupService::new(db, opts);

    // Share and enjoy!
    println!("{} listening for connections on {}", crate_name!(), addr);
    let server = match Server::try_bind(&addr) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("FATAL: {}", e);
            process::exit(1);
        }
    }
    .http1_pipeline_flush(true)
    .serve(make_svc)
    .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
