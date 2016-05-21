//! Information about projects that I have worked on.

use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;

use hyper::Client;
use hyper::header::{Authorization, Bearer, Connection, UserAgent};
use rustc_serialize::json::Json;
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
    let github = Github::new(dotenv!("GITHUB_TOKEN"));
    parse_projects(&mut projects_file)
        .expect("problem parsing projects file")
        .iter()
        .map(|parsed_project| {
            let name = &parsed_project.name;
            let path = format!("/repos/{}", &parsed_project.repo);
            let repository = github.request(&path).unwrap();
            let owner = repository.find_path(&["owner", "login"])
                .and_then(Json::as_string)
                .unwrap();
            let url = Url::parse(repository.find("html_url").and_then(Json::as_string).unwrap())
                .unwrap();
            let languages = {
                let url = repository.find("languages_url")
                    .and_then(Json::as_string)
                    .unwrap();
                let response = github.request_url(&url).unwrap();
                response.as_object()
                    .and_then(|obj| {
                        Some(obj.keys()
                            .cloned()
                            .collect())
                    })
                    .unwrap()
            };

            let description = {
                let description = &parsed_project.description;
                markdown::render_html(&description.deref())
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

struct Github {
    token: String,
    client: Client,
}

impl Github {
    const ENDPOINT: &'static str = "https://api.github.com";

    fn new(token: &str) -> Self {
        Github {
            client: Client::new(),
            token: token.to_owned(),
        }
    }

    fn request(&self, endpoint: &str) -> Option<Json> {
        let url = Self::ENDPOINT.to_owned() + endpoint;
        self.request_url(&url)
    }

    fn request_url(&self, url: &str) -> Option<Json> {
        let mut body = String::new();

        let mut response = self.client
            .get(url)
            .header(UserAgent("acrussell.com".to_owned()))
            .header(Authorization(Bearer { token: self.token.to_owned() }))
            .header(Connection::close())
            .send()
            .unwrap();
        response.read_to_string(&mut body).unwrap();
        Some(Json::from_str(&body).unwrap())
    }
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
