//! My personal website.

#![feature(associated_consts)]
#![feature(custom_derive)]
#![feature(plugin)]

#![plugin(dotenv_macros)]
#![plugin(serde_macros)]

#![warn(missing_docs)]

#[macro_use]
extern crate maplit;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate regex;
#[macro_use]
extern crate router;
#[macro_use]
extern crate quick_error;

extern crate ammonia;
extern crate chrono;
extern crate dotenv;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate hoedown;
extern crate hubcaps;
extern crate hyper;
extern crate iron;
extern crate mount;
extern crate persistent;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
extern crate staticfile;
extern crate toml;
extern crate url;
extern crate serde_yaml;

pub mod blog;
pub mod config;
pub mod helpers;
pub mod markdown;
pub mod persistence;
pub mod projects;
pub mod routes;

use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::net::ToSocketAddrs;

use iron::prelude::*;

/// Starts the server listening on the provided socket address.
pub fn listen<A>(addr: A)
    where A: ToSocketAddrs
{
    let config_path = env::var("WEBSITE_CONFIG").unwrap_or_else(|_| String::from("config.yaml"));
    let config = config::load(config_path).expect("could not parse configuration");
    let projects = projects::load("projects.yaml").expect("problem parsing projects");

    // Insert blog posts into the database.
    let connection = persistence::get_db_connection();
    let schema = {
        let mut schema_file = File::open("schema.sql").unwrap();
        let mut schema = String::new();
        schema_file.read_to_string(&mut schema).unwrap();
        schema
    };

    connection.execute_batch(&schema).unwrap();

    blog::load("blog/", &connection).expect("problem parsing blog posts");

    let handler = routes::handler(config, projects);

    info!("initialization complete");

    let listening = Iron::new(handler)
        .http(addr)
        .unwrap_or_else(|e| {
            panic!("Error: {:?}", e.description());
        });

    info!("listening on {}", listening.socket);
}
