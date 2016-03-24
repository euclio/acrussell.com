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

use blog::{self, Post};
use persistence::{self, Config};

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

    let projects = &req.get::<Read<Config>>().unwrap().projects;

    let data = btreemap!{
        "projects" => projects,
    };
    res.set_mut(Template::new("projects", data)).set_mut(status::Ok);
    Ok(res)
}

fn blog_post(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let year = req.extensions.get::<Router>().unwrap().find("year");
    let month = req.extensions.get::<Router>().unwrap().find("month");
    let day = req.extensions.get::<Router>().unwrap().find("day");
    let title = req.extensions.get::<Router>().unwrap().find("title");

    let post = blog::parse_posts(Path::new("blog/"))
                   .unwrap()
                   .into_iter()
                   .find(|post| {
                       let year = year.and_then(|y| y.parse().ok());
                       let month = month.and_then(|m| m.parse().ok());
                       let day = day.and_then(|d| d.parse().ok());

                       if let (Some(year), Some(month), Some(day)) = (year, month, day) {
                           let title_match = match title {
                               Some(title) => post.title.to_lowercase().replace(" ", "-") == title,
                               None => false,
                           };

                           let requested_date = NaiveDate::from_ymd(year, month, day);
                           post.date.date() == requested_date && title_match
                       } else {
                           false
                       }
                   });

    match post {
        Some(ref post) => {
            let data = btreemap!{
                "title" => post.title.to_owned(),
                "date" => post.date.format(Post::DATE_FORMAT).to_string(),
                "content" => post.html.to_string(),
            };
            res.set_mut(Template::new("blog_post", data)).set_mut(status::Ok);
            Ok(res)
        }
        None => Err(IronError::new(NoRoute, status::NotFound)),
    }
}

fn blog(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    #[derive(Debug, Serialize)]
    struct Post {
        title: String,
        date: String,
        html: String,
    }

    let connection = persistence::get_db_connection();

    let mut posts_stmt = connection.prepare("SELECT title, date, html FROM posts").unwrap();

    let posts = posts_stmt.query_map(&[], |row| {
                              Post {
                                  title: row.get(0),
                                  date: row.get(1),
                                  html: row.get(2),
                              }
                          })
                          .unwrap()
                          .map(Result::unwrap)
                          .collect::<Vec<_>>();

    let data = btreemap!{
        "posts" => posts,
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

    #[derive(Debug, Serialize)]
    struct Summary {
        title: String,
        date: String,
        summary: String,
        url: String,
    }

    let connection = persistence::get_db_connection();
    let mut summaries_stmt = connection.prepare("SELECT title, date, summary, url FROM posts \
                                                 ORDER BY date desc LIMIT 3")
                                       .unwrap();
    let summaries = summaries_stmt.query_map(&[], |row| {
                                      Summary {
                                          title: row.get(0),
                                          date: row.get(1),
                                          summary: row.get(2),
                                          url: row.get(3),
                                      }
                                  })
                                  .unwrap()
                                  .map(Result::unwrap)
                                  .collect::<Vec<_>>();

    let data = btreemap!{
        "posts" => summaries,
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
