use std::path::PathBuf;

use hubcaps;
use rusqlite;
use serde_yaml;

error_chain! {
    foreign_links {
        hubcaps::Error, GitHub;
        rusqlite::Error, Sqlite;
        serde_yaml::Error, Yaml;
    }

    errors {
        PostParse(path: PathBuf) {
            description("could not parse blog post")
            display("could not parse blog post: {}", path.to_str().unwrap())
        }
    }
}
