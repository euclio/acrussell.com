//! Information about projects that I have worked on.

use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;

use hubcaps::{Credentials, Github};
use hubcaps::repositories::Repository;
use hyper::Client;
use serde_yaml;
use url::Url;

use config::ConfigError;
use markdown::{self, Html, Markdown};

/// Encapsulates a project that I have worked on.
#[derive(Debug, Serialize)]
pub struct Project {
    name: String,
    owner: String,
    languages: Vec<String>,
    description: Html,
    url: Url,
}

/// Returns a list of projects parsed from a file.
pub fn load<P>(projects_path: P) -> Result<Vec<Project>, ConfigError>
    where P: AsRef<Path>
{
    let mut projects_file = try!(File::open(projects_path));
    let client = Client::new();
    let github = Github::new(concat!("acrussell.com", "/", env!("CARGO_PKG_VERSION")),
                             &client,
                             Credentials::Token(String::from(dotenv!("GITHUB_TOKEN"))));
    parse_projects(&mut projects_file)
        .expect("problem parsing projects file")
        .iter()
        .map(|parsed_project| {
            let repo = {
                let components = parsed_project.repo.split("/").collect::<Vec<_>>();
                try!(Repository::new(&github, components[0], components[1]).get())
            };

            let name = &parsed_project.name;
            let owner = repo.owner.login.clone();
            let url = Url::parse(&repo.html_url).unwrap();
            let languages = try!(repo.languages(&github)).keys().cloned().collect();

            let description = {
                let description = &parsed_project.description;
                markdown::render_html(description.deref())
            };

            Ok(Project {
                name: name.to_owned(),
                owner: owner.to_owned(),
                description: description,
                languages: languages,
                url: url,
            })
        })
        .collect()
}

#[derive(Debug, Deserialize)]
struct ParsedProject {
    name: String,
    repo: String,
    description: Markdown,
}

fn parse_projects<R>(reader: &mut R) -> Result<Vec<ParsedProject>, ConfigError>
    where R: Read
{
    Ok(try!(serde_yaml::from_reader(reader)))
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    #[test]
    fn parse_all_projects() {
        let mut projects_file = File::open("projects.yaml").unwrap();
        super::parse_projects(&mut projects_file).unwrap();
    }
}
