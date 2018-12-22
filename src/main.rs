#[macro_use]
extern crate clap;
use clap::{App, Arg};

use httpdate;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::Server;
use nanogeoip::{Options, Reader};

use std::process;
use std::sync::Arc;

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
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

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

    let port = match value_t!(matches, "port", u16) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let cors_raw = matches.value_of("cors").unwrap(); //safe bc default val
    let opts = Options {
        cors_header: Some(cors_raw.to_string()),
    };

    let (db_ref, opts_ref) = (Arc::new(db), Arc::new(opts));
    let make_svc = move || {
        let (svc_db, svc_opts) = (db_ref.clone(), opts_ref.clone());
        service_fn_ok(move |req| nanogeoip::lookup(req, &svc_db, &svc_opts))
    };

    let addr = ([127, 0, 0, 1], port).into();
    println!("{} listening for connections on {}", crate_name!(), addr);
    let server = Server::bind(&addr)
        .http1_pipeline_flush(true)
        .serve(make_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
