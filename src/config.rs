//! Utilities for managing the configuration of the website.
//!
//! The configuration should contain values that I expect to change. This way, I can edit them all
//! in a single human-readable file instead of having to track down the values in the code.

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

use url::Url;
use serde_yaml;

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
        YamlSyntax(err: serde_yaml::Error) {
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
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    /// A link to a PDF copy of my Resume.
    pub resume_link: Url,
}

fn parse_config<R>(reader: R) -> Result<Config, ConfigError>
    where R: Read
{
    let config = try!(serde_yaml::from_reader(reader));
    Ok(config)
}

/// Load the website configuration from a path.
pub fn load<P>(config_path: P) -> Result<Config, ConfigError>
    where P: AsRef<Path>
{
    info!("loading configuration from {:?}", config_path.as_ref());
    let config_file = try!(File::open(config_path));
    parse_config(config_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    use url::Url;

    #[test]
    fn load_config() {
        let test_config = String::from(r#"
---
resume_link: http://google.com
"#);
        let expected_config = Config { resume_link: Url::parse("http://google.com").unwrap() };
        assert_eq!(expected_config,
                   super::parse_config(test_config.as_bytes()).unwrap());
    }
}
