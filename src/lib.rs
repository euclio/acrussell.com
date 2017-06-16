//! My personal website.

#![feature(associated_consts)]
#![feature(plugin)]

#![plugin(dotenv_macros)]

#![warn(missing_docs)]

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate iron;
#[macro_use]
extern crate log;
#[macro_use]
extern crate router;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate ammonia;
extern crate chrono;
extern crate dotenv;
extern crate handlebars_iron;
extern crate hoedown;
extern crate hubcaps;
extern crate hyper;
extern crate hyper_native_tls;
extern crate mount;
extern crate params;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate serde;
extern crate serde_yaml;
extern crate staticfile;
extern crate toml;
extern crate url;
extern crate url_serde;

// This must be after the error_chain import to avoid an error message.
#[macro_use]
extern crate derive_error_chain;

pub mod blog;
pub mod config;
pub mod errors;
pub mod helpers;
pub mod markdown;
pub mod persistence;
pub mod projects;
pub mod routes;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::net::ToSocketAddrs;

use handlebars_iron::handlebars;
use iron::prelude::*;
use iron::Listening;

use errors::*;

/// Starts the server listening on the provided socket address.
pub fn listen<A>(addr: A, database_uri: &str) -> Result<Listening>
where
    A: ToSocketAddrs,
{
    let config_path = env::var("WEBSITE_CONFIG").unwrap_or_else(|_| String::from("config.yaml"));
    let config = config::load(config_path).chain_err(
        || "could not parse configuration",
    )?;
    let projects = projects::load("projects.yaml").chain_err(
        || "problem parsing projects",
    )?;

    // Insert blog posts into the database.
    let pool = persistence::get_connection_pool(database_uri)?;
    let connection = pool.get().chain_err(|| "database connection timed out")?;

    let schema = {
        let mut schema_file = File::open("schema.sql")?;
        let mut schema = String::new();
        schema_file.read_to_string(&mut schema)?;
        schema
    };
    connection.execute_batch(&schema)?;

    blog::load("blog/", &connection).chain_err(
        || "problem parsing blog posts",
    )?;

    let handler = routes::handler(config, projects, pool)?;

    info!("initialization complete");

    let listening = Iron::new(handler).http(addr)?;
    info!("listening on {}", listening.socket);

    Ok(listening)
}
