use std::path::PathBuf;

use hubcaps;
use rusqlite;
use serde_yaml;

error_chain! {
    foreign_links {
        GitHub(hubcaps::Error);
        Sqlite(rusqlite::Error);
        Yaml(serde_yaml::Error);
    }

    errors {
        PostParse(path: PathBuf) {
            description("could not parse blog post")
            display("could not parse blog post: {}", path.to_str().unwrap())
        }
    }
}
