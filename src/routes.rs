//! Contains the routing logic for the website.

use std::error::Error;
use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use mount::Mount;
use hbs::{self, DirectorySource, HandlebarsEngine, Template};
use iron::prelude::*;
use iron::{status, AfterMiddleware, Handler};
use params::{Params, Value};
use persistent::{self, Read};
use router::{Router, NoRoute};
use serde_json;
use staticfile::Static;

use blog;
use config;
use helpers;
use projects::Project;
use persistence::{DatabaseConnectionPool, ConnectionPool, Config, Projects};

/// The number of blog post summaries that should be displayed.
const NUM_SUMMARIES: usize = 3;

fn resume(req: &mut Request) -> IronResult<Response> {
    let data = btreemap!{
        "resume_link" => req.get::<Read<Config>>().unwrap().resume_link.to_owned(),
    };
    Ok(Response::with((status::Ok, Template::new("resume", data))))
}

fn projects(req: &mut Request) -> IronResult<Response> {
    let projects = req.get::<Read<Projects>>().unwrap();
    let data = btreemap!{
        "projects" => projects,
    };
    Ok(Response::with((status::Ok, Template::new("projects", data))))
}

fn blog_post(req: &mut Request) -> IronResult<Response> {
    let connection = req.extensions.get::<Read<DatabaseConnectionPool>>().unwrap().get().unwrap();
    let params = req.extensions.get::<Router>().unwrap();

    let year = params.find("year").and_then(|y| y.parse().ok());
    let month = params.find("month").and_then(|m| m.parse().ok());
    let day = params.find("day").and_then(|d| d.parse().ok());
    let title = try!(req.extensions
        .get::<Router>()
        .unwrap()
        .find("title")
        .ok_or(IronError::new(NoRoute, status::NotFound)));

    let date = match (year, month, day) {
        (Some(year), Some(month), Some(day)) => NaiveDate::from_ymd_opt(year, month, day),
        _ => None,
    };

    let date = try!(date.ok_or(IronError::new(NoRoute, status::NotFound)));

    match blog::get_post(&connection, &date, title) {
        Ok(ref post) => Ok(Response::with((status::Ok, Template::new("blog_post", post)))),
        Err(err) => {
            error!("Error retrieving blog post: {}", err);
            Err(IronError::new(NoRoute, status::NotFound))
        }
    }
}

fn blog(req: &mut Request) -> IronResult<Response> {
    let connection = req.get::<Read<DatabaseConnectionPool>>().unwrap().get().unwrap();

    let query = match req.get_ref::<Params>().unwrap().find(&["q"]) {
        Some(&Value::String(ref query)) if !query.is_empty() => Some(query.to_owned()),
        _ => None,
    };

    let summaries = if let Some(ref query) = query {
        blog::find_summaries(&connection, query)
    } else {
        blog::get_summaries(&connection)
    };

    let mut data = btreemap!{
        "posts" => serde_json::to_value(itry!(summaries)),
    };

    if let Some(ref query) = query {
        data.insert("query", serde_json::to_value(query));
    }

    Ok(Response::with((status::Ok, Template::new("blog", data))))
}

fn about(_: &mut Request) -> IronResult<Response> {
    let images = Path::new("static/images/slideshow");
    let image_urls = fs::read_dir(images)
        .unwrap()
        .into_iter()
        .map(|path| path.unwrap().path().to_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    let data = btreemap! {
        "image_urls" => image_urls,
    };
    Ok(Response::with((status::Ok, Template::new("about", data))))
}

fn index(req: &mut Request) -> IronResult<Response> {
    let connection = req.extensions.get::<Read<DatabaseConnectionPool>>().unwrap().get().unwrap();
    let posts = blog::get_summaries(&connection);

    let data = btreemap!{
        "posts" => posts.unwrap().into_iter().take(NUM_SUMMARIES).collect::<Vec<_>>(),
    };
    Ok(Response::with((status::Ok, Template::new("index", data))))
}

/// Returns the router for the server.
fn get_router() -> Router {
    router!(
        index:      get "/" => index,
        about:      get "/about" => about,
        blog:       get "/blog" => blog,
        blog_post:  get "/blog/:year/:month/:day/:title" => blog_post,
        projects:   get "/projects" => projects,
        resume:     get "/resume" => resume,

        favicon:    get "/favicon.ico" => Static::new(Path::new("static/images")),
        robots_txt: get "/robots.txt" => Static::new(Path::new("static")),
    )
}

fn initialize_templates(folder: &str,
                        extension: &str)
                        -> Result<HandlebarsEngine, hbs::SourceError> {
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new(folder, extension)));
    try!(hbse.reload());

    {
        let mut reg = hbse.registry.write().unwrap();
        reg.register_helper("join", Box::new(helpers::join));
    }

    Ok(hbse)
}


fn mount(chain: Chain) -> Mount {
    let mut mount = Mount::new();
    mount.mount("/", chain);
    mount.mount("/static",
                Static::new(Path::new(env!("OUT_DIR")).join("static")));
    mount
}

/// Returns the main route handler for the website.
///
/// This handler contains a number of middleware:
///   - Routing for all page requests
///   - Routing for static files
///   - Persistence for the website configuration
///   - Rendering handlebars templates
///   - Error reporting
///   - Error handling
pub fn handler(config: config::Config,
               projects: Vec<Project>,
               connection_pool: ConnectionPool)
               -> Box<Handler> {
    let mut chain = Chain::new(get_router());

    chain.link_before(persistent::Read::<Config>::one(config));
    chain.link_before(persistent::Read::<Projects>::one(projects));
    chain.link_before(persistent::Read::<DatabaseConnectionPool>::one(connection_pool));

    chain.link_after(ErrorHandler);
    chain.link_after(initialize_templates("./templates/", ".hbs").unwrap());
    chain.link_after(ErrorReporter);

    let mount = mount(chain);
    Box::new(mount)
}

struct ErrorReporter;

impl AfterMiddleware for ErrorReporter {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        error!("{}", err.description());
        Err(err)
    }
}

struct ErrorHandler;

impl AfterMiddleware for ErrorHandler {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, Template::new("not_found", ()))))
        } else {
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate iron;
    extern crate iron_test;
    extern crate tempfile;
    extern crate url;

    use std::fs::File;
    use std::io::prelude::*;

    use self::iron::{Handler, Headers};
    use self::iron_test::{request, response};
    use self::tempfile::NamedTempFile;
    use self::url::Url;

    use config::Config;
    use persistence;

    fn create_handler() -> Box<Handler> {
        let tempfile = NamedTempFile::new().unwrap();

        // Populate the database.
        //
        // FIXME: This should be handled by a server object, to avoid duplication between code and
        // tests.
        let uri = format!("file:{}", tempfile.path().to_str().unwrap());
        let pool = persistence::get_connection_pool(&uri);
        let connection = pool.get().unwrap();
        let schema = {
            let mut schema_file = File::open("schema.sql").unwrap();
            let mut schema = String::new();
            schema_file.read_to_string(&mut schema).unwrap();
            schema
        };

        connection.execute_batch(&schema).unwrap();

        super::handler(Config { resume_link: Url::parse("http://google.com").unwrap() },
                       vec![],
                       pool)
    }

    #[test]
    fn index() {
        let handler = create_handler();
        let response = request::get("http://localhost:3000/", Headers::new(), &handler).unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn blog() {
        let handler = create_handler();
        let response = request::get("http://localhost:3000/blog", Headers::new(), &handler)
            .unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn about() {
        let handler = create_handler();
        let response = request::get("http://localhost:3000/about", Headers::new(), &handler)
            .unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn projects() {
        let handler = create_handler();
        let response = request::get("http://localhost:3000/projects", Headers::new(), &handler)
            .unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn resume() {
        let handler = create_handler();
        let response = request::get("http://localhost:3000/resume", Headers::new(), &handler)
            .unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn post_dates() {
        let handler = create_handler();

        let response = request::get("http://localhost:3000/blog/2016/13/31/invalid-date",
                                    Headers::new(),
                                    &handler)
            .unwrap();
        assert!(response.status.unwrap().is_client_error());
    }

    #[test]
    fn static_files() {
        let handler = create_handler();
        let response = request::get("http://localhost:3000/favicon.ico",
                                    Headers::new(),
                                    &handler)
            .unwrap();
        assert!(response.status.unwrap().is_success());

        let response = request::get("http://localhost:3000/robots.txt", Headers::new(), &handler)
            .unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn not_found() {
        let handler = create_handler();
        let response = request::get("http://localhost:3000/this/path/does/not/exist",
                                    Headers::new(),
                                    &handler)
            .unwrap();
        let body = response::extract_body_to_string(response);
        assert!(body.contains("Page Not Found"));
    }

    #[test]
    fn css() {
        let handler = create_handler();
        let response = request::get("http://localhost:3000/static/css/styles.css",
                                    Headers::new(),
                                    &handler)
            .unwrap();
        assert!(response.status.unwrap().is_success());
    }
}
