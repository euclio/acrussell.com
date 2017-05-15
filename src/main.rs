#[macro_use]
extern crate clap;

extern crate env_logger;
extern crate log;
extern crate website;

use std::env;
use std::io::prelude::*;
use std::io;
use std::process;

use clap::{App, Arg};
use env_logger::LogBuilder;
use log::LogLevelFilter;

use website::errors::*;
use website::persistence::DEFAULT_DATABASE_URI;

const DEFAULT_PORT: u16 = 9000;

static ABOUT: &str = r"
My personal website.
";

fn main() {
    if env::var("RUST_LOG").is_err() {
        LogBuilder::new()
            .filter(None, LogLevelFilter::Info)
            .filter(Some("html5ever"), LogLevelFilter::Error)
            .init()
            .unwrap();
    } else {
        env_logger::init().unwrap();
    }

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(ABOUT)
        .arg(Arg::with_name("port")
             .help("The port that the server should listen for connections on."))
        .arg(Arg::with_name("db_uri")
                .long("db-uri")
                .value_name("URI")
                .help(
                    "A sqlite databse URI to use for the website's backing store. By default, \
                    this is a shared, in-memory database. It may be helpful to use a file for \
                    debugging purposes. \

                    Please note that any existing data in the database pointed at by this URI \
                    will be dropped upon server initialization.
            "))
        .get_matches();

    let port = matches
        .value_of("port")
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);
    let db_uri = matches
        .value_of("db_uri")
        .unwrap_or_else(|| DEFAULT_DATABASE_URI);

    if let Err(ref e) = run(port, db_uri) {
        let stderr = &mut io::stderr();
        let errmsg = "error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        process::exit(1);
    }
}

fn run(port: u16, db_uri: &str) -> Result<()> {
    let _ = website::listen(("localhost", port), db_uri)
        .chain_err(|| "could not start server")?;
    Ok(())
}
