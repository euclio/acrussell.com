//! Utilities for managing the configuration of the website.
//!
//! The configuration should contain values that I expect to change. This way, I can edit them all
//! in a single human-readable file instead of having to track down the values in the code.

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use log::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use url::Url;
use url_serde;

use crate::errors::*;

/// Configuration values for a given page.
#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    /// The human-readable title for the page.
    pub name: String,

    /// The name of the template that should be used to render the page.
    pub template: String,

    /// The relative path that the page should be written to.
    path: PathBuf,
}

impl Page {
    /// The file path that the page's rendered HTML should be written to.
    pub fn html_path(&self) -> PathBuf {
        let path = self.path.strip_prefix("/").unwrap();
        path.join("index.html")
    }
}

/// Configuration values for the website.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// A link to a PDF copy of my Resume.
    #[serde(with = "url_serde")]
    pub resume_link: Url,

    /// The pages of the site.
    pub pages: Vec<Page>,
}

fn parse_config<R>(reader: R) -> Result<Config>
where
    R: Read,
{
    let config = serde_yaml::from_reader(reader)?;
    Ok(config)
}

/// Load the website configuration from a path.
pub fn load<P>(config_path: P) -> Result<Config>
where
    P: AsRef<Path>,
{
    let path = config_path.as_ref().to_str().unwrap();
    info!("loading configuration from {}", path);
    let config_file = File::open(&config_path).chain_err(|| "error opening config file")?;
    parse_config(config_file)
}
