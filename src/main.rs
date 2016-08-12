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

const DEFAULT_PORT: u16 = 9000;

docopt!(Args derive Debug, r"
My personal website.

Usage:
    website [<port>]
", arg_port: Option<u16>);

fn main() {
    LogBuilder::new().filter(None, LogLevelFilter::Info).init().unwrap();

    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    website::listen(("localhost", args.arg_port.unwrap_or(DEFAULT_PORT)))
}
