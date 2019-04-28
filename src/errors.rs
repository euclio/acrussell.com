//! Contains convenient type aliases and traits for error handling.

#![allow(deprecated)]

use std::io;
use std::path::PathBuf;

use diesel;
use hubcaps;
use iron;
use serde_yaml;
use url;

error_chain! {
    errors {
        PostParse(p: PathBuf){
            description("could not parse blog post"),
            display("could not parse blog post: {}", p.display())
        }
    }

    foreign_links {
        GitHub(hubcaps::Error);
        Io(io::Error);
        HTTP(iron::error::HttpError);
        Sql(diesel::result::Error);
        UrlParse(url::ParseError);
        Yaml(serde_yaml::Error);
    }

}
