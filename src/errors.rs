use std::path::{PathBuf, Path};

use hubcaps;
use rusqlite;
use serde_yaml;

#[derive(Debug, error_chain)]
pub enum ErrorKind {
    Msg(String),

    #[error_chain(foreign)]
    GitHub(hubcaps::Error),

    #[error_chain(foreign)]
    Sqlite(rusqlite::Error),

    #[error_chain(foreign)]
    Yaml(serde_yaml::Error),

    #[error_chain(custom)]
    #[error_chain(description = r#"|_| "could not parse blog post""#)]
    #[error_chain(display = r#"|p: &Path| write!(f, "could not parse blog post: {}", p.display())"#)]
    PostParse(PathBuf),
}
