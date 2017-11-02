//! Static blog generation.

use std::fs::{self, File};
use std::path::Path;
use std::io::prelude::*;

use ammonia::{self, Ammonia};
use chrono::{NaiveDateTime, NaiveDate, Datelike};
use diesel::expression::{dsl, sql, AsExpression};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::types::Bool;
use diesel::{self, ExecuteDsl};
use log::{log, info};
use serde_derive::{Deserialize, Serialize};
use serde_yaml;

use errors::{self, ResultExt, ErrorKind};
use markdown::{self, Html, Markdown};
use models::{Summary, NewPost, PostLink, PostContent};

/// The length of a blog post preview.
const SUMMARY_LENGTH: usize = 200;

#[allow(missing_docs)]
mod infix {
    /// Diesel doesn't support FTS4 `MATCH` out of the box for SQLite, so we implement it ourselves.
    diesel_infix_operator!(Matches, " MATCH ");
}

fn fts_match<T, U>(left: T, right: U) -> infix::Matches<T, U::Expression>
where
    T: Expression,
    U: AsExpression<T::SqlType>,
{
    infix::Matches::new(left, right.as_expression())
}


/// A struct representing a blog post.
#[derive(Debug, Serialize)]
pub struct Post {
    /// The title of the post.
    pub title: String,

    /// The time that the post was written.
    #[serde(with = "human_readable_format")]
    pub date: NaiveDateTime,

    /// The post rendered as HTML.
    pub html: Html,

    /// The next post chronologically.
    pub next_post: Option<PostLink>,

    /// The previous post chronologically.
    pub prev_post: Option<PostLink>,
}

/// Retrieves blog post content and metadata by parsing all markdown files in a given directory,
/// then persists the posts into the database.
pub fn load<P>(directory: P, conn: &SqliteConnection) -> errors::Result<()>
where
    P: AsRef<Path>,
{
    use schema::posts::dsl::*;

    let parsed_posts = parse_posts(&directory)?;
    info!(
        "parsed {} blog posts in {:?}",
        parsed_posts.len(),
        directory.as_ref()
    );

    let new_posts = parsed_posts
        .iter()
        .map(|post| {
            let post_html = markdown::render_html(&post.content);
            let post_summary = create_summary(&post_html, &post.url());

            NewPost {
                title: &post.metadata.title,
                date: post.metadata.date,
                html: post_html.to_string(),
                summary: post_summary.to_string(),
                url: post.url().to_string(),
                slug: post.slug(),
            }
        })
        .collect::<Vec<_>>();

    diesel::insert(&new_posts).into(posts).execute(conn)?;

    create_fts_index(conn)?;

    Ok(())
}

/// Creates the full text search index for the blog posts.
pub fn create_fts_index(conn: &SqliteConnection) -> errors::Result<()> {
    use schema::posts::dsl::*;
    use schema::post_content;

    sql::<Bool>(
        r#"CREATE VIRTUAL TABLE post_content USING fts4(content, title)"#,
    ).execute(conn)?;

    // TODO: We should actually load the content here, not the HTML.
    let new_post_content = posts.select((id, title, html)).load::<PostContent>(conn)?;

    diesel::insert(&new_post_content)
        .into(post_content::table)
        .execute(conn)?;

    info!("optimizing blog post content index");
    sql::<Bool>(
        r#"INSERT INTO post_content(post_content) VALUES ('optimize')"#,
    ).execute(conn)?;

    Ok(())
}

/// Searches post contents and titles with a text query.
///
/// Returns summaries of the posts that contain the query.
pub fn find_summaries(conn: &SqliteConnection, query: &str) -> errors::Result<Vec<Summary>> {
    use schema::post_content;
    use schema::post_content::dsl as content_dsl;
    use schema::posts::dsl::*;

    let matching_ids = post_content::table.select(content_dsl::docid).filter(
        fts_match(
            content_dsl::content,
            query,
        ),
    );
    let summaries = posts
        .select((title, date, summary, url))
        .filter(id.eq_any(matching_ids))
        .load::<Summary>(conn)?;

    Ok(summaries)
}

/// Retrieves a blog post from the database given the date it was posted and its title.
pub fn get_post(
    conn: &SqliteConnection,
    post_date: &NaiveDate,
    post_slug: &str,
) -> errors::Result<Post> {
    use schema::posts::dsl::*;

    // TODO: We should be able to do this in a single query.

    let post = posts
        .select((id, title, html, date, url))
        .filter(slug.eq(post_slug))
        .filter(dsl::date(date).eq(post_date))
        .first::<::models::Post>(conn)?;

    let next_post = posts
        .select((title, url))
        .order(date.asc())
        .filter(dsl::date(date).gt(post_date))
        .first::<PostLink>(conn)
        .optional()?;

    let prev_post = posts
        .select((title, url))
        .order(date.desc())
        .filter(dsl::date(date).lt(post_date))
        .first::<PostLink>(conn)
        .optional()?;

    Ok(Post {
        title: String::from(post.title.as_str()),
        date: post.date.clone(),
        html: Html::new(post.html.to_string()),
        next_post: next_post,
        prev_post: prev_post,
    })
}

/// Retrieves blog post summaries from the database.
pub fn get_summaries(conn: &SqliteConnection) -> errors::Result<Vec<Summary>> {
    use schema::posts::dsl::*;

    Ok(posts
        .select((title, date, summary, url))
        .order(date.desc())
        .load::<Summary>(conn)?)
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
        format!(
            "/blog/{}/{}/{}/{}",
            date.year(),
            date.month(),
            date.day(),
            self.slug()
        )
    }

    /// Returns the escaped title of the post, for use in the URL.
    fn slug(&self) -> String {
        self.metadata.title.to_lowercase().replace(' ', "-")
    }
}

#[derive(Debug, Deserialize)]
struct Metadata {
    title: String,
    #[serde(with = "on_disk_format")]
    date: NaiveDateTime,
    categories: Vec<String>,
    tags: Vec<String>,
}


fn parse_post<R>(reader: &mut R) -> errors::Result<ParsedPost>
where
    R: Read,
{
    let post = {
        let mut post = String::new();
        reader.read_to_string(&mut post).chain_err(
            || "could not read contents of post",
        )?;
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
        url_relative: ammonia::UrlRelative::PassThrough,
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
where
    P: AsRef<Path>,
{
    let entries = fs::read_dir(directory)
        .chain_err(|| "could not read blog posts directory")?
        .into_iter();

    entries
        .map(|entry| {
            let entry = entry.unwrap();
            let mut file = File::open(entry.path()).chain_err(
                || "error opening directory entry",
            )?;
            let post = parse_post(&mut file).chain_err(|| {
                ErrorKind::PostParse(entry.path().to_owned())
            })?;
            Ok(post)
        })
        .collect()
}

mod on_disk_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer};

    /// The date format used in the blog posts' markdown files.
    const ON_DISK_FORMAT: &str = "%l:%M%P %m/%d/%y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, ON_DISK_FORMAT).map_err(serde::de::Error::custom)
    }
}

/// Module intended to be used by `serde(with)` to format human-readable dates for display.
pub mod human_readable_format {
    use chrono::NaiveDateTime;
    use serde::Serializer;

    /// The date format suitable for displaying on the website.
    pub const HUMAN_READABLE_FORMAT: &str = "%B %e, %Y";

    /// Serializes a date into a human-readable format.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format(HUMAN_READABLE_FORMAT).to_string())
    }
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
