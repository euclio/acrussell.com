//! Contains convenient type aliases and traits for error handling.

use std::io;
use std::path::{PathBuf, Path};

use diesel;
use derive_error_chain::ErrorChain;
use hubcaps;
use hyper;
use serde_yaml;
use url;

/// Possible Error variants of the website.
#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    /// A convenient wrapper around a string message.
    Msg(String),

    /// Error communicating with GitHub.
    #[error_chain(foreign)]
    GitHub(hubcaps::Error),

    /// Error communicating over HTTP.
    #[error_chain(foreign)]
    HTTP(hyper::Error),

    /// Error performing IO.
    #[error_chain(foreign)]
    Io(io::Error),

    /// Error communicating with the database.
    #[error_chain(foreign)]
    Sql(diesel::result::Error),

    /// Error parsing a URL from a string.
    #[error_chain(foreign)]
    UrlParse(url::ParseError),

    /// Error parsing YAML.
    #[error_chain(foreign)]
    Yaml(serde_yaml::Error),

    /// Error parsing a blog post.
    #[error_chain(custom)]
    #[error_chain(description = r#"|_| "could not parse blog post""#)]
    #[error_chain(display = r#"|p: &Path| write!(f, "could not parse blog post: {}", p.display())"#)]
    PostParse(PathBuf),
}
