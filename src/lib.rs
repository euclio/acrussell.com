//! My personal website.

#![feature(associated_consts)]
#![feature(custom_derive)]
#![feature(plugin)]

#![plugin(clippy)]
#![plugin(serde_macros)]
#![plugin(regex_macros)]

#![warn(missing_docs)]

// FIXME: Yeah, it's slow, but the compile-time guarantee of correct syntax is worth it.
#![allow(regex_macro)]

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
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate hoedown;
extern crate hyper;
extern crate iron;
extern crate mount;
extern crate persistent;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate staticfile;
extern crate toml;
extern crate yaml_rust as yaml;

pub mod blog;
pub mod helpers;
pub mod persistence;
pub mod projects;
pub mod routes;

use std::error::Error;
use std::net::ToSocketAddrs;
use std::path::Path;

use hbs::{DirectorySource, HandlebarsEngine, Template};
use iron::AfterMiddleware;
use iron::prelude::*;
use iron::status;
use mount::Mount;
use persistent::Read;
use router::{NoRoute, Router};
use staticfile::Static;

use persistence::Projects;

const PROJECT_PATH: &'static str = "config.yaml";

fn initialize_templates(folder: &str,
                        extension: &str)
                        -> Result<HandlebarsEngine, hbs::SourceError> {
    let mut hbse = HandlebarsEngine::new2();
    hbse.add(Box::new(DirectorySource::new(folder, extension)));
    try!(hbse.reload());

    {
        let mut reg = hbse.registry.write().unwrap();
        reg.register_helper("join", Box::new(helpers::join));
    }

    Ok(hbse)
}

/// Starts the server listening on the provided socket address.
pub fn listen<A>(addr: A)
    where A: ToSocketAddrs
{

    let router: Router = routes::get_router();
    let mut chain = Chain::new(router);

    info!("loading projects from {}", PROJECT_PATH);
    let projects = projects::projects(PROJECT_PATH).unwrap();
    info!("loaded {} projects successfully", projects.len());
    chain.link_before(Read::<Projects>::one(projects));

    chain.link_after(ErrorReporter);
    chain.link_after(ErrorHandler);
    chain.link_after(initialize_templates("./templates/", ".hbs").unwrap());

    let mut mount = Mount::new();
    mount.mount("/", chain);
    mount.mount("/static", Static::new(Path::new("static")));
    mount.mount("/favicon.ico",
                Static::new(Path::new("static/images/favicon.ico")));
    mount.mount("/robots.txt", Static::new(Path::new("static/robots.txt")));

    Iron::new(mount)
        .http(addr)
        .unwrap_or_else(|e| {
            panic!("Error: {:?}", e.description());
        });

}

struct ErrorReporter;

impl AfterMiddleware for ErrorReporter {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        error!("{}", err.description());
        Err(err)
    }
}

struct ErrorHandler;

impl AfterMiddleware for ErrorHandler {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        let mut res = Response::new();

        if let Some(_) = err.error.downcast::<NoRoute>() {
            res.set_mut(Template::new("not_found", ())).set_mut(status::NotFound);
            Ok(res)
        } else {
            Err(err)
        }
    }
}
