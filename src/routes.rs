//! Contains the routing logic for the website.

// FIXME: maplit creates a *lot* of these warnings.
#![allow(used_underscore_binding)]

use std::fs;
use std::path::Path;

use chrono::Datelike;
use hbs::Template;
use iron::prelude::*;
use iron::status;
use persistent::Read;
use router::Router;

use blog::{self, Post, Summary};
use persistence::Projects;

const RESUME_LINK: &'static str =
    r"https://github.com/euclio/resume/blob/master/resume.pdf?raw=true";

fn generate_blog_post_summaries() -> Vec<Summary> {
    let mut posts = blog::parse_posts(Path::new("blog/")).unwrap();
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts.into_iter()
         .map(|p| p.to_summary())
         .collect::<Vec<_>>()
}

fn resume(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let data = btreemap!{
        "resume_link" => RESUME_LINK,
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

    let year = req.extensions.get::<Router>().unwrap().find("year").unwrap();
    let month = req.extensions.get::<Router>().unwrap().find("month").unwrap();
    let day = req.extensions.get::<Router>().unwrap().find("day").unwrap();
    let title = req.extensions.get::<Router>().unwrap().find("title").unwrap();

    let post = blog::parse_posts(Path::new("blog/"))
                   .unwrap()
                   .iter()
                   .find(|post| {
                       post.date.year() == year.parse().unwrap() &&
                       post.date.month() == month.parse().unwrap() &&
                       post.date.day() == day.parse().unwrap() &&
                       post.title.to_lowercase().replace(" ", "-") == title
                   })
                   .unwrap()
                   .to_owned();

    let data = btreemap!{
        "title" => post.title.to_owned(),
        "date" => post.date.format(Post::DATE_FORMAT).to_string(),
        "content" => post.html.to_string(),
    };
    res.set_mut(Template::new("blog_post", data)).set_mut(status::Ok);

    Ok(res)
}

fn blog(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let data = btreemap!{
        "posts" => generate_blog_post_summaries(),
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

    let summaries = generate_blog_post_summaries();

    let data = btreemap!{
        "posts" => summaries.iter().take(3).collect::<Vec<_>>(),
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
