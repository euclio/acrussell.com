//! Data to be used with a persistent router.

use iron::typemap::Key;
use rusqlite::Connection;

use config;

/// The string format of dates used in the database.
///
/// TODO: Remove once `rusqlite` implements `ToSql` and `FromSql` for `chrono::NaiveDateTime`. See
/// [jgallagher/rusqlite#133](https://github.com/jgallagher/rusqlite/pull/133).
pub const DATETIME_FORMAT: &'static str = "%F %T";

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
