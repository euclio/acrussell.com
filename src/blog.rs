//! Static blog generation.

use std::fs::{self, File};
use std::ops::Deref;
use std::path::Path;
use std::io::prelude::*;

use ammonia::Ammonia;
use chrono::{NaiveDateTime, NaiveDate, Datelike};
use rusqlite::{self, Connection};
use serde::{self, Deserialize};
use serde_yaml;

use errors::{self, ResultExt, ErrorKind};
use markdown::{self, Html, Markdown};

/// The length of a blog post preview.
const SUMMARY_LENGTH: usize = 200;

/// The date format used in the blog posts' markdown files.
const ON_DISK_FORMAT: &'static str = "%l:%M%P %m/%d/%y";

/// The date format suitable for displaying on the website.
const HUMAN_READABLE_FORMAT: &'static str = "%B %e, %Y";

/// A struct representing a blog post.
#[derive(Debug, Clone, Serialize)]
pub struct Post {
    /// The title of the post.
    pub title: String,

    /// The time that the post was written.
    #[serde(serialize_with="HumanReadable::serialize_with")]
    pub date: NaiveDateTime,

    /// The post rendered as HTML.
    pub html: Html,

    /// The next post chronologically.
    pub next_post: Option<PostLink>,

    /// The previous post chronologically.
    pub prev_post: Option<PostLink>,
}

/// Information needed to construct a link to a post.
#[derive(Debug, Clone, Serialize)]
pub struct PostLink {
    pub title: String,
    pub url: String,
}

/// A brief summary of a blog post.
#[derive(Serialize, Debug)]
pub struct Summary {
    /// The title of the post.
    pub title: String,

    /// The date the post was written.
    #[serde(serialize_with="HumanReadable::serialize_with")]
    pub date: NaiveDateTime,

    /// A short preview of the post.
    pub summary: String,

    /// A URL to reach the full post.
    pub url: String,
}

/// Retrieves blog post content and metadata by parsing all markdown files in a given directory,
/// then persists the posts into the database.
pub fn load<P>(directory: P, conn: &Connection) -> errors::Result<()>
    where P: AsRef<Path>
{
    let posts = parse_posts(&directory)?;
    info!("parsed {} blog posts in {:?}",
          posts.len(),
          directory.as_ref());

    conn.execute(r#"CREATE VIRTUAL TABLE post_content USING fts4(content, title)"#,
                 &[])?;

    let mut stmt = conn.prepare(r#"
        INSERT INTO posts (title, date, html, summary, url)
        VALUES ($1, $2, $3, $4, $5)
    "#)?;

    for post in posts {
        let html = markdown::render_html(&post.content);
        let summary = create_summary(&html, &post.url());
        let docid = stmt.insert(&[&post.metadata.title,
                                  &post.metadata.date,
                                  &html.deref(),
                                  &summary.deref(),
                                  &post.url().to_string()])?;
        let mut stmt = conn.prepare(r#"
            INSERT INTO post_content (docid, title, content)
            VALUES ($1, $2, $3)
        "#)?;
        stmt.execute(&[&docid, &post.metadata.title, &post.content.deref()])?;
    }

    info!("optimizing blog post content index");
    conn.execute(r#"INSERT INTO post_content(post_content) VALUES ('optimize')"#,
                 &[])?;

    Ok(())
}

/// Searches post contents and titles with a text query.
///
/// Returns summaries of the posts that contain the query.
pub fn find_summaries(conn: &rusqlite::Connection, query: &str) -> errors::Result<Vec<Summary>> {
    let mut stmt = conn.prepare(r#"
        SELECT title, date, summary, url
        FROM posts
        WHERE rowid IN (
            SELECT docid
            FROM post_content
            WHERE post_content MATCH ?
        )
    "#)?;

    let rows = stmt.query_map(&[&query], |row| {
            Summary {
                title: row.get(0),
                date: row.get(1),
                summary: row.get(2),
                url: row.get(3),
            }
        })?;

    let mut summaries = vec![];

    for row in rows {
        summaries.push(row?);
    }

    Ok(summaries)
}

/// Retrieves a blog post from the database given the date it was posted and its title.
pub fn get_post(conn: &rusqlite::Connection,
                date: &NaiveDate,
                title: &str)
                -> errors::Result<Post> {
    let mut stmt = conn.prepare(r#"
        SELECT title, date, html
        FROM posts
        WHERE REPLACE(LOWER(title), " ", "-") = $1
            AND DATE(date) = $2
    "#)?;

    let mut post = stmt.query_row(&[&title, date], |row| {
            Post {
                title: row.get(0),
                date: row.get(1),
                html: Html::new(row.get(2)),
                next_post: None,
                prev_post: None,
            }
        })?;

    let mut stmt = conn.prepare(r#"
        SELECT title, url
        FROM posts
        WHERE DATE(date) > ?
        ORDER BY date ASC
        LIMIT 1
    "#)?;

    let next_post = stmt.query_row(&[&post.date.date()], |row| {
        PostLink {
            title: row.get(0),
            url: row.get(1),
        }
    });

    post.next_post = match next_post {
        Ok(post) => Some(post),
        Err(rusqlite::Error::QueryReturnedNoRows) => None,
        Err(e) => return Err(e.into()),
    };

    let mut stmt = conn.prepare(r#"
        SELECT title, url
        FROM posts
        WHERE DATE(date) < ?
        ORDER BY date DESC
        LIMIT 1
    "#)?;

    let prev_post = stmt.query_row(&[&post.date.date()], |row| {
        PostLink {
            title: row.get(0),
            url: row.get(1),
        }
    });

    post.prev_post = match prev_post {
        Ok(post) => Some(post),
        Err(rusqlite::Error::QueryReturnedNoRows) => None,
        Err(e) => return Err(e.into()),
    };

    Ok(post)
}

/// Retrieves blog post summaries from the database.
pub fn get_summaries(conn: &Connection) -> errors::Result<Vec<Summary>> {
    let mut stmt = conn.prepare(r#"
        SELECT title, date, summary, url
        FROM posts
        ORDER BY date DESC
    "#)?;

    let rows = stmt.query_map(&[], |row| {
            Summary {
                title: row.get(0),
                date: row.get(1),
                summary: row.get(2),
                url: row.get(3),
            }
        })?;

    let mut summaries = vec![];

    for row in rows {
        summaries.push(row?);
    }

    Ok(summaries)
}

#[derive(Debug)]
struct ParsedPost {
    metadata: Metadata,
    content: Markdown,
}

impl ParsedPost {
    /// Returns a relative URL to the given post.
    fn url(&self) -> String {
        let date = &self.metadata.date;

        // TODO: I'd like to return a String here, but url::Url doesn't allow non-relative URLs. We
        // could work around this if we knew the server name.
        format!("/blog/{}/{}/{}/{}",
                date.year(),
                date.month(),
                date.day(),
                self.escaped_title())
    }

    fn escaped_title(&self) -> String {
        self.metadata.title.to_lowercase().replace(" ", "-")
    }
}

#[derive(Debug, Deserialize)]
struct Metadata {
    title: String,
    #[serde(deserialize_with="OnDisk::deserialize_with")]
    date: NaiveDateTime,
    categories: Vec<String>,
    tags: Vec<String>,
}


fn parse_post<R>(reader: &mut R) -> errors::Result<ParsedPost>
    where R: Read
{
    let post = {
        let mut post = String::new();
        reader
            .read_to_string(&mut post)
            .chain_err(|| "could not read contents of post")?;
        post
    };

    // Read up to the first double newline to determine the end of metadata.
    //
    // TODO: This could probably be more robust. Investigate separating the metadata and
    // markdown as separate YAML documents.
    let contents = post.splitn(2, "\n\n").collect::<Vec<_>>();

    let metadata = serde_yaml::from_str(contents[0])?;

    Ok(ParsedPost {
           metadata: metadata,
           content: Markdown::new(contents[1].to_owned()),
       })
}

fn create_summary(html: &Html, url: &str) -> Html {
    let ammonia = Ammonia {
        url_relative: true,
        ..Default::default()
    };

    let summary_link = format!(r#"… <a href="{}">Continue&rarr;</a>"#, url);
    let summary = html.chars()
        .take(SUMMARY_LENGTH)
        .chain(summary_link.chars())
        .collect::<String>();

    // Sanitize the summary so that any unclosed tags are closed again.
    let summary = ammonia.clean(&summary);

    Html::new(summary)
}

fn parse_posts<P>(directory: P) -> errors::Result<Vec<ParsedPost>>
    where P: AsRef<Path>
{
    let entries = fs::read_dir(directory)
        .chain_err(|| "could not read blog posts directory")?
        .into_iter();

    entries
        .map(|entry| {
                 let entry = entry.unwrap();
                 let mut file = File::open(entry.path())
                     .chain_err(|| "error opening directory entry")?;
                 let post = parse_post(&mut file)
                     .chain_err(|| ErrorKind::PostParse(entry.path().to_owned()))?;
                 Ok(post)
             })
        .collect()
}

trait DateSerializeWith {
    const FORMAT: &'static str;

    fn serialize_with<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(&date.format(Self::FORMAT).to_string())
    }
}

trait DateDeserializeWith {
    const FORMAT: &'static str;

    fn deserialize_with<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
        where D: serde::Deserializer<'de>
    {
        let string = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&string, Self::FORMAT).or_else(|_| {
            let msg = format!("invalid date format: expected '{}'", Self::FORMAT);
            Err(serde::de::Error::custom(msg.as_str()))
        })
    }
}

struct HumanReadable;

impl DateSerializeWith for HumanReadable {
    const FORMAT: &'static str = HUMAN_READABLE_FORMAT;
}

struct OnDisk;

impl DateDeserializeWith for OnDisk {
    const FORMAT: &'static str = ON_DISK_FORMAT;
}

#[cfg(test)]
mod tests {
    use markdown::Html;

    #[test]
    fn parse_all_posts() {
        super::parse_posts("blog").unwrap();
    }

    #[test]
    fn summary_sanitization() {
        let text = "<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
                    eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim \
                    veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea \
                    commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit \
                    esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
                    cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est \
                    laborum.</p>";
        let html = Html::new(String::from(text));
        assert!(html.len() > super::SUMMARY_LENGTH);

        let summary = super::create_summary(&html, "http://google.com");

        assert!(summary.ends_with("</p>"));
    }
}
