//! Static blog generation.

use std::fs::{self, File};
use std::path::Path;
use std::io;
use std::io::prelude::*;

use ammonia::Ammonia;
use chrono::{self, NaiveDateTime, Datelike};
use yaml::YamlLoader;
use yaml::ScanError;

use markdown::{self, Html};

/// The length of a blog post preview.
const SUMMARY_LENGTH: usize = 200;

/// A struct representing a blog post.
#[derive(Debug, Clone)]
pub struct Post {
    /// The title of the post.
    pub title: String,

    /// The time that the post was written.
    pub date: NaiveDateTime,

    /// The post rendered as HTML.
    pub html: Html,

    /// A short preview of the post.
    pub summary: String,

    /// A URL to reach the full post.
    pub url: String,
}

impl Post {
    /// The human-readable format that the date should appear as when rendering the post.
    pub const DATE_FORMAT: &'static str = "%B %e, %Y";
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
    }
}

fn parse_post(post: &str) -> Result<Post, PostParseError> {
    // Read up to the first double newline to determine the end of metadata.
    //
    // TODO: This could probably be more robust. Investigate separating the metadata and
    // markdown as separate YAML documents.
    let contents = post.splitn(2, "\n\n").collect::<Vec<_>>();
    let metadata = contents[0];
    let metadata = try!(YamlLoader::load_from_str(metadata))[0].to_owned();

    let title = try!(metadata["title"]
                         .as_str()
                         .ok_or(PostParseError::MetadataSyntax("could not find key 'title'"
                                                                   .to_owned())))
                    .to_owned();

    let date_string = try!(metadata["date"]
                               .as_str()
                               .ok_or(PostParseError::MetadataSyntax("could not find 'key'"
                                                                         .to_owned())));
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

    Ok(Post {
        title: title,
        date: date,
        html: html_content,
        summary: summary,
        url: url,
    })
}

/// Retrieves blog post content and metadata by parsing all markdown files in a given directory.
pub fn parse_posts(directory: &Path) -> Result<Vec<Post>, PostParseError> {
    try!(fs::read_dir(directory))
        .into_iter()
        .map(|entry| {
            let mut file = try!(File::open(try!(entry).path()));

            let mut contents = String::new();
            try!(file.read_to_string(&mut contents));

            parse_post(&contents)
        })
        .collect()
}
