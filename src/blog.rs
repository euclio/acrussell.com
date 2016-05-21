//! Static blog generation.

use std::fs::{self, File};
use std::path::Path;
use std::io;
use std::io::prelude::*;

use ammonia::Ammonia;
use chrono::{self, NaiveDateTime, NaiveDate, Datelike};
use rusqlite::{self, Connection};
use serde;
use yaml::YamlLoader;
use yaml::ScanError;

use markdown::{self, Html};
use persistence;

/// The length of a blog post preview.
const SUMMARY_LENGTH: usize = 200;

/// A struct representing a blog post.
#[derive(Debug, Clone, Serialize)]
pub struct Post {
    /// The title of the post.
    pub title: String,

    /// The time that the post was written.
    #[serde(serialize_with="Self::serialize_date")]
    pub date: NaiveDateTime,

    /// The post rendered as HTML.
    pub html: Html,
}

impl Post {
    /// The human-readable format that the date should appear as when rendering the post.
    #[cfg_attr(rustfmt, rustfmt_skip)]
    // TODO: Remove the above attribute. For some reason rustfmt wants to remove the visibility.
    pub const DATE_FORMAT: &'static str = "%B %e, %Y";

    fn serialize_date<S>(date: &NaiveDateTime, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(&date.format(Self::DATE_FORMAT).to_string())
    }
}

/// A brief summary of a blog post.
#[derive(Serialize, Debug)]
pub struct Summary {
    /// The title of the post.
    pub title: String,

    /// The date the post was written.
    #[serde(serialize_with="Self::serialize_date")]
    pub date: NaiveDateTime,

    /// A short preview of the post.
    pub summary: String,

    /// A URL to reach the full post.
    pub url: String,
}

impl Summary {
    fn serialize_date<S>(date: &NaiveDateTime, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(&date.format(Post::DATE_FORMAT).to_string())
    }
}

#[derive(Debug)]
struct ParsedPost {
    title: String,
    date: NaiveDateTime,
    summary: String,
    html: Html,
    url: String,
}

quick_error! {
    /// Encapsulates errors that might occur while parsing a blog post.
    #[derive(Debug)]
    pub enum PostParseError {
        /// There was a problem reading the file.
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }

        /// There was a syntax error in the metadata.
        MetadataSyntax(err: String) {
            from()
            from(err: ScanError) -> (err.to_string())
            from(err: chrono::ParseError) -> (err.to_string())
            description("yaml parsing error")
            display("Error parsing metadata: {}", err)
        }

        /// Something went wrong with the database.
        Database(err: rusqlite::Error) {
            from()
            description("problem with the database")
            display("Error with the database: {}", err)
            cause(err)
        }
    }
}

fn parse_post(post: &str) -> Result<ParsedPost, PostParseError> {
    // Read up to the first double newline to determine the end of metadata.
    //
    // TODO: This could probably be more robust. Investigate separating the metadata and
    // markdown as separate YAML documents.
    let contents = post.splitn(2, "\n\n").collect::<Vec<_>>();
    let metadata = contents[0];
    let metadata = try!(YamlLoader::load_from_str(metadata))[0].to_owned();

    let title = try!(metadata["title"]
            .as_str()
            .ok_or(PostParseError::MetadataSyntax("could not find key 'title'".to_owned())))
        .to_owned();

    let date_string = try!(metadata["date"]
        .as_str()
        .ok_or(PostParseError::MetadataSyntax("could not find 'key'".to_owned())));
    let date = try!(NaiveDateTime::parse_from_str(date_string, "%l:%M%P %m/%d/%y"));

    let content = contents[1];
    let html_content = markdown::render_html(&content);

    let escaped_title = title.to_lowercase().replace(" ", "-");
    let url = format!("/blog/{}/{}/{}/{}",
                      date.year(),
                      date.month(),
                      date.day(),
                      escaped_title);

    let mut ammonia = Ammonia::default();

    ammonia.url_relative = true;

    // Add a link at the end of the preview, then sanitize the HTML and close any unclosed
    // tags.
    let summary_link = format!(r#"... <a href="{}">Continue&rarr;</a>"#, url);
    let summary = html_content.chars()
        .take(SUMMARY_LENGTH)
        .chain(summary_link.chars())
        .collect::<String>();

    Ok(ParsedPost {
        title: title,
        date: date,
        html: html_content,
        summary: ammonia.clean(&summary),
        url: url,
    })
}

/// Retrieves blog post content and metadata by parsing all markdown files in a given directory,
/// then persists the posts into the database.
pub fn parse_posts<P>(directory: P, connection: &Connection) -> Result<(), PostParseError>
    where P: AsRef<Path>
{
    let posts = try!(fs::read_dir(&directory))
        .into_iter()
        .map(|entry| {
            let mut file = try!(File::open(try!(entry).path()));

            let mut contents = String::new();
            try!(file.read_to_string(&mut contents));

            parse_post(&contents)
        })
        .collect::<Vec<_>>();

    info!("parsed {} blog posts in {:?}",
          posts.len(),
          directory.as_ref());

    for post in posts {
        let post = post.unwrap();
        connection.execute(r"INSERT INTO posts (title, date, html, summary, url)
                             VALUES ($1, $2, $3, $4, $5)",
                     &[&post.title,
                       &post.date.format(persistence::DATETIME_FORMAT).to_string(),
                       &post.html.as_str(),
                       &post.summary,
                       &post.url.to_string()])
            .unwrap();
    }

    Ok(())
}

/// Retrieves a blog post from the database given the date it was posted and its title.
pub fn get_post(date: &NaiveDate, title: &str) -> Result<Post, rusqlite::Error> {
    let connection = persistence::get_db_connection();
    connection.query_row(r#"SELECT title, date, html
                            FROM posts
                            WHERE REPLACE(LOWER(title), " ", "-") = $1
                              AND DATE(date) = DATE($2)"#,
                         &[&title, &date.format(persistence::DATETIME_FORMAT).to_string()],
                         |row| {
        let date = NaiveDateTime::parse_from_str(&row.get::<_, String>(1),
                                                 persistence::DATETIME_FORMAT)
            .unwrap();
        Post {
            title: row.get(0),
            date: date,
            html: Html(row.get(2)),
        }
    })
}

/// Retrieves blog post summaries from the database.
pub fn get_summaries(connection: &Connection) -> Result<Vec<Summary>, rusqlite::Error> {
    let mut stmt = connection.prepare(r"SELECT title, date, summary, url
                                   FROM posts
                                   ORDER BY date DESC")
        .unwrap();

    let summaries = stmt.query_map(&[], |row| {
            let date = row.get::<_, String>(1);

            Summary {
                title: row.get(0),
                date: NaiveDateTime::parse_from_str(&date, persistence::DATETIME_FORMAT).unwrap(),
                summary: row.get(2),
                url: row.get(3),
            }
        })
        .unwrap();

    summaries.collect()
}
