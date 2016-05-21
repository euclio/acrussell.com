//! Data to be used with a persistent router.

use iron::typemap::Key;
use rusqlite::{Connection, SQLITE_OPEN_READ_WRITE, SQLITE_OPEN_URI};

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
    Connection::open_with_flags("file::memory:?cache=shared",
                                SQLITE_OPEN_URI | SQLITE_OPEN_READ_WRITE)
        .expect("problem connecting to database.")
}
