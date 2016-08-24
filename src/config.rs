//! Utilities for managing the configuration of the website.
//!
//! The configuration should contain values that I expect to change. This way, I can edit them all
//! in a single human-readable file instead of having to track down the values in the code.

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use url::Url;
use serde_yaml;

use errors::*;

/// Configuration values for the website.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    /// A link to a PDF copy of my Resume.
    pub resume_link: Url,
}

fn parse_config<R>(reader: R) -> Result<Config>
    where R: Read
{
    let config = try!(serde_yaml::from_reader(reader));
    Ok(config)
}

/// Load the website configuration from a path.
pub fn load<P>(config_path: P) -> Result<Config>
    where P: AsRef<Path>
{
    let path = config_path.as_ref().to_str().unwrap();
    info!("loading configuration from {}", path);
    let config_file = try!(File::open(&config_path).chain_err(|| "error opening config file"));
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
