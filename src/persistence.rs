//! Data to be used with a persistent router.

use iron::typemap::Key;
use rusqlite::Connection;

use config;

/// Contains data found in the website configuration.
#[derive(Copy, Clone)]
pub struct Config;

impl Key for Config {
    type Value = config::Config;
}

/// Create a new connection to an in-memory database.
///
/// Reading and writing from multiple database connections is thread-safe.
///
/// # Panics
/// This function panics when a connection cannot be established.
pub fn get_db_connection() -> Connection {
    Connection::open("test.sqlite").expect("problem connecting to database.")
}
