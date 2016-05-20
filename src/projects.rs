//! Information about projects that I have worked on.

use std::io::prelude::*;

use hyper::Client;
use hyper::header::{Authorization, Bearer, Connection, UserAgent};
use rustc_serialize::json::Json;
use url::Url;
use yaml::Yaml;

use config::ConfigError;
use markdown::{self, Html};

/// Encapsulates a project that I have worked on.
#[derive(Debug, Serialize)]
pub struct Project {
    name: String,
    owner: String,
    languages: Vec<String>,
    description: Html,
    url: Url,
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

fn parse_project(project: &Yaml) -> Result<Project, ConfigError> {
    let github = Github::new(dotenv!("GITHUB_TOKEN"));

    let name = try!(project["name"]
        .as_str()
        .ok_or(ConfigError::Format("could not find key 'name'")));

    let repo = try!(project["repo"]
        .as_str()
        .ok_or(ConfigError::Format("could not find key 'repo'")));

    let path = format!("/repos/{}", repo);
    let repository = github.request(&path).unwrap();

    let owner = repository.find_path(&["owner", "login"])
        .and_then(|json| json.as_string())
        .unwrap();

    let url = Url::parse(repository.find("html_url").and_then(|json| json.as_string()).unwrap())
        .unwrap();

    let languages = {
        let url = repository.find("languages_url")
            .and_then(|json| json.as_string())
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
        let description = try!(project["description"]
            .as_str()
            .ok_or(ConfigError::Format("could not find key 'description'")));
        markdown::render_html(&description)
    };

    Ok(Project {
        name: name.to_owned(),
        owner: owner.to_owned(),
        description: description,
        languages: languages,
        url: url,
    })
}


/// Returns a list of projects parsed from a file.
pub fn projects(projects: &Yaml) -> Result<Vec<Project>, ConfigError> {
    let projects = try!(projects.as_vec()
        .ok_or(ConfigError::Format("expected vector in project configuration")));

    projects.iter().map(|project| parse_project(project)).collect()
}
