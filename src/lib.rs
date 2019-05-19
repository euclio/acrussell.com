//! My personal website.

#![warn(missing_docs)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate error_chain;

extern crate ammonia;
extern crate chrono;
extern crate dotenv;
extern crate handlebars_iron;
extern crate hoedown;
extern crate hubcaps;
extern crate iron;
extern crate log;
extern crate mount;
extern crate params;
extern crate pathdiff;
extern crate persistent;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate staticfile;
extern crate tokio;
extern crate toml;
extern crate url;
extern crate url_serde;

pub mod blog;
pub mod config;
pub mod errors;
pub mod helpers;
pub mod markdown;
pub mod persistence;
pub mod projects;
pub mod routes;

mod models;
mod schema;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::net::ToSocketAddrs;

use diesel::connection::SimpleConnection;
use handlebars_iron::handlebars;
use iron::prelude::*;
use iron::Listening;
use log::*;

use errors::*;

/// Starts the server listening on the provided socket address.
pub fn listen<A>(addr: A, database_uri: &str) -> Result<Listening>
where
    A: ToSocketAddrs,
{
    let config_path = env::var("WEBSITE_CONFIG").unwrap_or_else(|_| String::from("config.yaml"));
    let config = config::load(config_path).chain_err(|| "could not parse configuration")?;
    let projects = projects::load("projects.yaml").chain_err(|| "problem parsing projects")?;

    // Insert blog posts into the database.
    let pool = persistence::get_connection_pool(database_uri)?;
    let connection = pool.get().chain_err(|| "database connection timed out")?;

    let schema = {
        let mut schema_file = File::open("schema.sql")?;
        let mut schema = String::new();
        schema_file.read_to_string(&mut schema)?;
        schema
    };
    connection.batch_execute(&schema).unwrap();

    blog::load("blog/", &connection).chain_err(|| "problem parsing blog posts")?;

    let handler = routes::handler(config, projects, pool)?;

    info!("initialization complete");

    let listening = Iron::new(handler).http(addr)?;
    info!("listening on {}", listening.socket);

    Ok(listening)
}
