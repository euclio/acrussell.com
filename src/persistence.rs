//! Data to be used with a persistent router.

use std::ops::Deref;

use diesel::SqliteConnection;
use iron::typemap::Key;
use r2d2::{self, Pool};
use r2d2_diesel::ConnectionManager;

use config;
use errors::*;
use projects;

/// The database URI that the website connects to by default. This may be overridden at runtime.
pub const DEFAULT_DATABASE_URI: &str = "file::memory:?cache=shared";

/// The key for accessing the website configuration.
#[derive(Copy, Clone)]
pub struct Config;

impl Key for Config {
    type Value = config::Config;
}

/// The key for accessing the persistence that contains projects parsed from a configuration file.
#[derive(Copy, Clone)]
pub struct Projects;

impl Key for Projects {
    type Value = Vec<projects::Project>;
}

/// The key for accessing the database connection pool persistence.
#[derive(Copy, Clone)]
pub struct DatabaseConnectionPool;

impl Key for DatabaseConnectionPool {
    type Value = ConnectionPool;
}

/// A connection pool for maintaining multiple database connections.
pub struct ConnectionPool(Pool<ConnectionManager<SqliteConnection>>);

impl Deref for ConnectionPool {
    type Target = Pool<ConnectionManager<SqliteConnection>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Creates a new connection pool to the given database URI.
///
/// # Panics
/// This function panics when a connection pool cannot be established.
pub fn get_connection_pool(database_uri: &str) -> Result<ConnectionPool> {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::new(database_uri);
    let pool = Pool::new(config, manager).chain_err(
        || "error initializing database",
    )?;
    Ok(ConnectionPool(pool))
}
