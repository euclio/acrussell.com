#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate docopt;
extern crate env_logger;
extern crate rustc_serialize;
extern crate website;

const DEFAULT_PORT: u16 = 9000;

docopt!(Args derive Debug, r"
My personal website.

Usage:
    website [<port>]
", arg_port: Option<u16>);

fn main() {
    env_logger::init().unwrap();

    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    website::listen(("localhost", args.arg_port.unwrap_or(DEFAULT_PORT)))
}
