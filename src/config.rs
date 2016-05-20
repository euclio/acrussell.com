//! Utilities for managing the configuration of the website.
//!
//! The configuration should contain values that I expect to change. This way, I can edit them all
//! in a single human-readable file instead of having to track down the values in the code.

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use url::Url;
use yaml::YamlLoader;

use projects::{self, Project};

quick_error!{
    /// Encapsulates errors that might occur while parsing configuration.
    #[derive(Debug)]
    pub enum ConfigError {
        /// There was a problem reading the file.
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }

        /// There was a syntax error in the YAML.
        YamlSyntax(err: ::yaml::ScanError) {
            from()
            description("YAML syntax error")
            display("YAML syntax error: {}", err)
            cause(err)
        }

        /// The configuration file was formatted incorrectly.
        Format(err: &'static str) {
            from()
            description("the configuration file was formatted incorrectly")
            display("Error parsing configuration: {}", err)
        }
    }
}

/// Configuration values for the website.
pub struct Config {
    /// Descriptions of projects that I have implemented.
    pub projects: Vec<Project>,

    /// A link to a PDF copy of my Resume.
    pub resume_link: Url,
}

/// Load the website configuration from a file.
pub fn load<P>(path: P) -> Result<Config, ConfigError>
    where P: AsRef<Path>
{
    let path = path.as_ref();
    info!("loading configuration from {:?}", path);

    let config = {
        let mut yaml_file = try!(File::open(path));
        let mut string = String::new();
        try!(yaml_file.read_to_string(&mut string));
        try!(YamlLoader::load_from_str(&string))[0].to_owned()
    };

    let projects = try!(projects::projects(&config["projects"]));
    info!("loaded {} projects successfully", projects.len());

    let resume_link = try!(config["resume"]["link"]
        .as_str()
        .ok_or("could not find resume link in config"));

    Ok(Config {
        projects: projects,
        resume_link: Url::parse(resume_link).unwrap(),
    })
}
