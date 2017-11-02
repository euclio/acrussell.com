//! Contains the routing logic for the website.

use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use chrono::NaiveDate;
use diesel;
use handlebars_iron::{DirectorySource, HandlebarsEngine, Template};
use iron::prelude::*;
use iron::status;
use iron::{AfterMiddleware, Handler, itry, iexpect};
use log::{log, error};
use mount::Mount;
use params::{Params, Value};
use persistent::{self, Read};
use router::{Router, NoRoute, router};
use serde_json::{self, json, json_internal};
use staticfile::Static;

#[cfg(feature = "watch")]
use handlebars_iron::Watchable;

use blog;
use config;
use errors::*;
use helpers;
use persistence::{DatabaseConnectionPool, ConnectionPool, Config, Projects};
use projects::Project;

/// The number of blog post summaries that should be displayed.
const NUM_SUMMARIES: usize = 3;

fn resume(req: &mut Request) -> IronResult<Response> {
    let data = json!({
        "resume_link": req.get::<Read<Config>>().unwrap().resume_link.to_string(),
    });
    Ok(Response::with((status::Ok, Template::new("resume", data))))
}

fn projects(req: &mut Request) -> IronResult<Response> {
    let projects = req.get::<Read<Projects>>().unwrap();
    let data = json!({
        "projects": *projects,
    });
    Ok(Response::with(
        (status::Ok, Template::new("projects", data)),
    ))
}

fn blog_post(req: &mut Request) -> IronResult<Response> {
    let connection = req.extensions
        .get::<Read<DatabaseConnectionPool>>()
        .unwrap()
        .get()
        .unwrap();
    let params = req.extensions.get::<Router>().unwrap();

    let year = iexpect!(params.find("year").and_then(|y| y.parse().ok()));
    let month = iexpect!(params.find("month").and_then(|m| m.parse().ok()));
    let day = iexpect!(params.find("day").and_then(|d| d.parse().ok()));
    let slug = iexpect!(req.extensions
        .get::<Router>()
        .unwrap()
        .find("slug"));

    let date = iexpect!(NaiveDate::from_ymd_opt(year, month, day));
    let post = match blog::get_post(&connection, &date, slug) {
        Ok(post) => post,
        Err(Error(ErrorKind::Sql(diesel::NotFound), _)) => {
            return Err(IronError::new(NoRoute, status::NotFound))
        }
        Err(e) => return Err(IronError::new(e, status::NotFound)),
    };

    Ok(Response::with(
        (status::Ok, Template::new("blog_post", post)),
    ))
}

fn blog(req: &mut Request) -> IronResult<Response> {
    let connection = req.get::<Read<DatabaseConnectionPool>>()
        .unwrap()
        .get()
        .unwrap();

    let query = match req.get_ref::<Params>().unwrap().find(&["q"]) {
        Some(&Value::String(ref query)) if !query.is_empty() => Some(query.to_owned()),
        _ => None,
    };

    let summaries = if let Some(ref query) = query {
        blog::find_summaries(&connection, query)
    } else {
        blog::get_summaries(&connection)
    };

    let mut data = json!({
        "posts": itry!(summaries)
    });

    if let Some(query) = query.and_then(|query| serde_json::to_value(query).ok()) {
        data["query"] = query;
    }

    Ok(Response::with((status::Ok, Template::new("blog", data))))
}

fn about(_: &mut Request) -> IronResult<Response> {
    let images = Path::new("static/images/slideshow");
    let image_urls = itry!(fs::read_dir(images))
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|path| path.path())
        .collect::<Vec<_>>();

    let data = json!({
        "image_urls": image_urls
    });
    Ok(Response::with((status::Ok, Template::new("about", data))))
}

fn index(req: &mut Request) -> IronResult<Response> {
    let connection = req.extensions
        .get::<Read<DatabaseConnectionPool>>()
        .unwrap()
        .get()
        .unwrap();
    let posts = itry!(blog::get_summaries(&connection));

    let data = json!({
        "posts": posts.into_iter().take(NUM_SUMMARIES).collect::<Vec<_>>()
    });
    Ok(Response::with((status::Ok, Template::new("index", data))))
}

/// Returns the router for the server.
fn get_router() -> Router {
    router!(
        index:      get "/" => index,
        about:      get "/about" => about,
        blog:       get "/blog" => blog,
        blog_post:  get "/blog/:year/:month/:day/:slug" => blog_post,
        projects:   get "/projects" => projects,
        resume:     get "/resume" => resume,

        favicon:    get "/favicon.ico" => Static::new(Path::new("static/images")),
        robots_txt: get "/robots.txt" => Static::new(Path::new("static")),
    )
}

#[cfg(feature = "watch")]
fn watch_templates(hbse: Arc<HandlebarsEngine>, path: &str) {
    hbse.watch(path);
}

#[cfg(not(feature = "watch"))]
fn watch_templates(_hbse: Arc<HandlebarsEngine>, _path: &str) {}

fn initialize_templates(folder: &str, extension: &str) -> Result<Arc<HandlebarsEngine>> {
    let hbse = {
        let mut hbse = HandlebarsEngine::new();
        hbse.add(Box::new(DirectorySource::new(folder, extension)));
        hbse.handlebars_mut().register_helper(
            "join",
            Box::new(helpers::join),
        );
        hbse.reload().chain_err(|| "could not reload templates")?;

        Arc::new(hbse)
    };

    watch_templates(hbse.clone(), "./templates");

    Ok(hbse)
}


fn mount(chain: Chain) -> Mount {
    let mut mount = Mount::new();
    mount.mount("/", chain);
    mount.mount(
        "/static",
        Static::new(Path::new(env!("OUT_DIR")).join("static")),
    );
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
pub fn handler(
    config: config::Config,
    projects: Vec<Project>,
    connection_pool: ConnectionPool,
) -> Result<Box<Handler>> {
    let mut chain = Chain::new(get_router());

    chain.link_before(persistent::Read::<Config>::one(config));
    chain.link_before(persistent::Read::<Projects>::one(projects));
    chain.link_before(persistent::Read::<DatabaseConnectionPool>::one(
        connection_pool,
    ));

    chain.link_after(ErrorHandler);
    chain.link_after(initialize_templates("./templates/", ".hbs")?);
    chain.link_after(ErrorReporter);

    let mount = mount(chain);
    Ok(Box::new(mount))
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
        if err.error.downcast::<NoRoute>().is_some() {
            Ok(Response::with(
                (status::NotFound, Template::new("not_found", ())),
            ))
        } else {
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate iron_test;
    extern crate tempfile;
    extern crate url;

    use std::fs::File;
    use std::io::prelude::*;

    use diesel::connection::SimpleConnection;
    use iron::{Handler, Headers};

    use self::iron_test::{request, response};
    use self::tempfile::NamedTempFile;
    use self::url::Url;

    use config::Config;
    use persistence;

    struct Server {
        pub handler: Box<Handler>,
        pub database: NamedTempFile,
    }

    /// Creates a handler for the website.
    ///
    /// Returns a pair containing the handler, and a tempfile containing a test sqlite database.
    fn create_server() -> Server {
        let tempfile = NamedTempFile::new().unwrap();

        // Populate the database.
        //
        // FIXME: This should be handled by a server object, to avoid duplication between code and
        // tests.
        let pool = persistence::get_connection_pool(tempfile.path().to_str().unwrap()).unwrap();
        let connection = pool.get().unwrap();
        let schema = {
            let mut schema_file = File::open("schema.sql").unwrap();
            let mut schema = String::new();
            schema_file.read_to_string(&mut schema).unwrap();
            schema
        };

        connection.batch_execute(&schema).unwrap();
        ::blog::create_fts_index(&connection).unwrap();

        let handler = super::handler(
            Config { resume_link: Url::parse("http://google.com").unwrap() },
            vec![],
            pool,
        ).unwrap();
        Server {
            handler,
            database: tempfile,
        }
    }

    #[test]
    fn index() {
        let server = create_server();
        let response = request::get("http://localhost:3000/", Headers::new(), &server.handler)
            .unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn blog() {
        let server = create_server();
        let response = request::get(
            "http://localhost:3000/blog",
            Headers::new(),
            &server.handler,
        ).unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn blog_search() {
        let server = create_server();
        let response = request::get(
            "http://localhost:3000/blog?q=nonsensequerythatwillreturnnovalues",
            Headers::new(),
            &server.handler,
        ).unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn about() {
        let server = create_server();
        let response = request::get(
            "http://localhost:3000/about",
            Headers::new(),
            &server.handler,
        ).unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn projects() {
        let server = create_server();
        let response = request::get(
            "http://localhost:3000/projects",
            Headers::new(),
            &server.handler,
        ).unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn resume() {
        let server = create_server();
        let response = request::get(
            "http://localhost:3000/resume",
            Headers::new(),
            &server.handler,
        ).unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn post_dates() {
        let server = create_server();
        let response = request::get(
            "http://localhost:3000/blog/2016/13/31/invalid-date",
            Headers::new(),
            &server.handler,
        ).unwrap();
        assert!(response.status.unwrap().is_client_error());
    }

    #[test]
    fn static_files() {
        let server = create_server();
        let response = request::get(
            "http://localhost:3000/favicon.ico",
            Headers::new(),
            &server.handler,
        ).unwrap();
        assert!(response.status.unwrap().is_success());

        let response = request::get(
            "http://localhost:3000/robots.txt",
            Headers::new(),
            &server.handler,
        ).unwrap();
        assert!(response.status.unwrap().is_success());
    }

    #[test]
    fn not_found() {
        let server = create_server();
        let response = request::get(
            "http://localhost:3000/this/path/does/not/exist",
            Headers::new(),
            &server.handler,
        ).unwrap();
        let body = response::extract_body_to_string(response);
        assert!(body.contains("Page Not Found"));

        let response = request::get(
            "http://localhost:3000/blog/1992/08/18/does-not-exist",
            Headers::new(),
            &server.handler,
        ).unwrap();
        let body = response::extract_body_to_string(response);
        assert!(body.contains("Page Not Found"));
    }

    #[test]
    fn css() {
        let server = create_server();
        let response = request::get(
            "http://localhost:3000/static/css/styles.css",
            Headers::new(),
            &server.handler,
        ).unwrap();
        assert!(response.status.unwrap().is_success());
    }
}
