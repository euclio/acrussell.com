//! Information about projects that I have worked on.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;

use hubcaps::{Credentials, Github};
use serde::{Deserialize, Serialize};
use serde_yaml;
use tokio::runtime::Runtime;
use url::Url;
use url_serde;

use errors::*;
use markdown::{self, Html, Markdown};

/// Encapsulates a project that I have worked on.
#[derive(Debug, Serialize)]
pub struct Project {
    name: String,
    owner: String,
    languages: Vec<String>,
    description: Html,
    #[serde(with = "url_serde")]
    url: Url,
}

/// Returns a list of projects parsed from a file.
pub fn load<P>(projects_path: P) -> Result<Vec<Project>>
where
    P: AsRef<Path>,
{
    let mut projects_file = File::open(projects_path).chain_err(
        || "could not open project file",
    )?;
    let github = Github::new(
        concat!("acrussell.com", "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(String::from(dotenv!("GITHUB_TOKEN"))),
    );

    let mut rt = Runtime::new()?;

    parse_projects(&mut projects_file)
        .chain_err(|| "problem parsing projects file")?
        .iter()
        .map(|parsed_project| {
            let repo = {
                let components = parsed_project.repo.split('/').collect::<Vec<_>>();
                github.repo(components[0], components[1])
            };

            let repo = rt.block_on(repo.get())?;

            let name = &parsed_project.name;
            let owner = repo.owner.login.clone();
            let url = Url::parse(&repo.html_url)?;

            // Sort languages by the amount of bytes in the repository.
            let languages = rt.block_on(repo.languages(github.clone()))?
                .into_iter()
                .map(|(k, v)| (v, k))
                .collect::<BTreeMap<_, _>>()
                .values()
                .cloned()
                .rev()
                .collect();

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

fn parse_projects<R>(reader: &mut R) -> Result<Vec<ParsedProject>>
where
    R: Read,
{
    Ok(serde_yaml::from_reader(reader)?)
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
