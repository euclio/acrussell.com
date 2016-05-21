//! Contains the routing logic for the website.

// FIXME: maplit creates a *lot* of these warnings.
#![cfg_attr(feature="clippy", allow(used_underscore_binding))]

use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use hbs::Template;
use iron::prelude::*;
use iron::status;
use persistent::Read;
use router::{Router, NoRoute};

use blog;
use persistence::{self, Config, Projects};

/// The number of blog post summaries that should be displayed.
const NUM_SUMMARIES: usize = 3;

fn resume(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let data = btreemap!{
        "resume_link" => req.get::<Read<Config>>().unwrap().resume_link.to_owned(),
    };
    res.set_mut(Template::new("resume", data)).set_mut(status::Ok);
    Ok(res)
}

fn projects(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let projects = req.get::<Read<Projects>>().unwrap();
    let data = btreemap!{
        "projects" => projects,
    };
    res.set_mut(Template::new("projects", data)).set_mut(status::Ok);
    Ok(res)
}

fn blog_post(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let year = req.extensions.get::<Router>().unwrap().find("year").and_then(|y| y.parse().ok());
    let month = req.extensions.get::<Router>().unwrap().find("month").and_then(|m| m.parse().ok());
    let day = req.extensions.get::<Router>().unwrap().find("day").and_then(|d| d.parse().ok());
    let title = req.extensions.get::<Router>().unwrap().find("title");

    let date = match (year, month, day) {
        (Some(year), Some(month), Some(day)) => NaiveDate::from_ymd(year, month, day),
        _ => return Err(IronError::new(NoRoute, status::NotFound)),
    };

    let title = match title {
        Some(title) => title,
        None => return Err(IronError::new(NoRoute, status::NotFound)),
    };

    match blog::get_post(&date, &title) {
        Ok(ref post) => {
            res.set_mut(Template::new("blog_post", post)).set_mut(status::Ok);
            Ok(res)
        }
        Err(err) => {
            error!("Error retrieving blog post: {}", err);
            Err(IronError::new(NoRoute, status::NotFound))
        }
    }
}

fn blog(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let connection = persistence::get_db_connection();
    let data = btreemap!{
        "posts" => blog::get_summaries(&connection).unwrap(),
    };
    res.set_mut(Template::new("blog", data)).set_mut(status::Ok);
    Ok(res)
}

fn about(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let images = Path::new("static/images/slideshow");
    let image_urls = fs::read_dir(images)
        .unwrap()
        .into_iter()
        .map(|path| path.unwrap().path().to_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    let data = btreemap! {
        "image_urls" => image_urls,
    };
    res.set_mut(Template::new("about", data)).set_mut(status::Ok);
    Ok(res)
}

fn index(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let connection = persistence::get_db_connection();
    let posts = blog::get_summaries(&connection);

    let data = btreemap!{
        "posts" => posts.unwrap().into_iter().take(NUM_SUMMARIES).collect::<Vec<_>>(),
    };
    res.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(res)
}

/// Returns the router for the server.
pub fn get_router() -> Router {
    router!(
        get "/" => index,
        get "/about" => about,
        get "/blog" => blog,
        get "/blog/:year/:month/:day/:title" => blog_post,
        get "/projects" => projects,
        get "/resume" => resume,
    )
}
