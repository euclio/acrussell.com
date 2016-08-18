#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate docopt;
extern crate env_logger;
extern crate log;
extern crate rustc_serialize;
extern crate website;

use env_logger::LogBuilder;
use log::LogLevelFilter;

use website::persistence::DEFAULT_DATABASE_URI;

const DEFAULT_PORT: u16 = 9000;

docopt!(Args derive Debug, r"
My personal website.

Usage:
    website [options] [<port>]
    website (-h | --help)

Options:
    --db-uri=<URI>      A sqlite database URI to use for the website's backing store. By default,
                        this is a shared, in-memory database.

                        Please note that any existing data in the database pointed at by this URI
                        will be dropped upon server initialization.

                        This option is useful for debugging purposes.
", arg_port: Option<u16>, flag_db_uri: Option<String>);

fn main() {
    LogBuilder::new().filter(None, LogLevelFilter::Info).init().unwrap();

    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    let address = ("localhost", args.arg_port.unwrap_or(DEFAULT_PORT));
    website::listen(address,
                    &args.flag_db_uri.unwrap_or_else(|| String::from(DEFAULT_DATABASE_URI)));
}
