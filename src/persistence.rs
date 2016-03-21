//! Data to be used with a persistent router.

use iron::typemap::Key;

use config;

/// Contains data found in the website configuration.
#[derive(Copy, Clone)]
pub struct Config;

impl Key for Config {
    type Value = config::Config;
}
