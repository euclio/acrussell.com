//! Information about projects that I have worked on.

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use hyper::Client;
use hyper::header::{Authorization, Bearer, Connection, UserAgent};
use rustc_serialize::json::Json;
use url::Url;
use yaml::{Yaml, YamlLoader};

use markdown::{self, Html};

quick_error!{
    /// Encapsulates errors that might occur while parsing a project.
    #[derive(Debug)]
    pub enum ProjectParseError {
        /// There was a problem reading the file.
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }

        /// There was a syntax error in the YAML.
        YamlSyntax(err: ::yaml::ScanError) {
            from()
            description("YAML syntax error")
            display("YAML syntax error: {}", err)
            cause(err)
        }

        /// The project configuration file was formatted incorrectly.
        Format(err: &'static str) {
            from()
            description("the project configuration file was formatted incorrectly")
            display("Error parsing project configuration: {}", err)
        }
    }
}

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

fn parse_project(project: &Yaml) -> Result<Project, ProjectParseError> {
    let github = Github::new(dotenv!("GITHUB_TOKEN"));

    let name = try!(project["name"]
                        .as_str()
                        .ok_or(ProjectParseError::Format("could not find key 'name'")));

    let repo = try!(project["repo"]
                        .as_str()
                        .ok_or(ProjectParseError::Format("could not find key 'repo'")));

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
                                   .ok_or(ProjectParseError::Format("could not find key \
                                                                     'description'")));
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
pub fn projects<P>(path: P) -> Result<Vec<Project>, ProjectParseError>
    where P: AsRef<Path>
{
    let yaml = {
        let mut yaml_file = try!(File::open(path.as_ref()));
        let mut string = String::new();
        try!(yaml_file.read_to_string(&mut string));
        string
    };

    let doc = &try!(YamlLoader::load_from_str(&yaml))[0];

    let projects = try!(doc["projects"]
                            .as_vec()
                            .ok_or(ProjectParseError::Format("could not find key \"projects\"")));
    projects.iter().map(|project| parse_project(project)).collect()
}
