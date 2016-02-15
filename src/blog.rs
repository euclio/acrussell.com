//! Static blog generation.

use std::fs::{self, File};
use std::ops::Deref;
use std::path::Path;
use std::io;
use std::io::prelude::*;

use ammonia::Ammonia;
use chrono::{self, Datelike, DateTime, TimeZone, UTC};
use hoedown::{self, Markdown};
use hoedown::renderer::html;
use hoedown::Render;
use yaml::YamlLoader;
use yaml::ScanError;

/// The length of a blog post preview.
const PREVIEW_LENGTH: usize = 200;

/// An owned string containing HTML.
#[derive(Debug, Clone)]
pub struct Html(String);

impl Deref for Html {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

/// A struct representing a blog post.
#[derive(Debug, Clone)]
pub struct Post {
    /// The title of the post.
    pub title: String,

    /// The time that the post was written.
    pub date: DateTime<UTC>,

    /// The post rendered as HTML.
    pub html: Html,
}

impl Post {
    /// The human-readable format that the date should appear as when rendering the post.
    pub const DATE_FORMAT: &'static str = "%B %e, %Y";

    /// Returns the URL to retrieve a blog post.
    pub fn url(&self) -> String {
        let escaped_title = self.title.to_lowercase().replace(" ", "-");
        format!("/blog/{}/{}/{}/{}",
                self.date.year(),
                self.date.month(),
                self.date.day(),
                escaped_title)
    }

    /// Creates a summarized version of the blog post.
    pub fn to_summary(&self) -> Summary {
        let mut ammonia = Ammonia::default();

        ammonia.url_relative = true;

        // Add a link at the end of the preview, then sanitize the HTML and close any unclosed
        // tags.
        let preview_link = format!(r#"... <a href="{}">Continue&rarr;</a>"#, self.url());
        let preview = &self.html
                           .chars()
                           .take(PREVIEW_LENGTH)
                           .chain(preview_link.chars())
                           .collect::<String>();
        Summary {
            title: self.title.to_owned(),
            date: self.date.format(Self::DATE_FORMAT).to_string(),
            preview: ammonia.clean(preview).to_owned(),
            url: self.url(),
        }
    }
}

/// A brief summary of a blog post.
#[derive(Serialize, Debug)]
pub struct Summary {
    /// The title of the post.
    pub title: String,

    /// The date the post was written.
    pub date: String,

    /// A short preview of the post.
    pub preview: String,

    /// A URL to reach the full post.
    pub url: String,
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
    use hoedown::{AUTOLINK, FENCED_CODE, TABLES};
    use self::PostParseError::*;

    // Read up to the first double newline to determine the end of metadata.
    //
    // TODO: This could probably be more robust. Investigate separating the metadata and
    // markdown as separate YAML documents.
    let contents = post.splitn(2, "\n\n").collect::<Vec<_>>();
    let metadata = contents[0];
    let metadata = try!(YamlLoader::load_from_str(metadata))[0].to_owned();

    let content = contents[1];
    let markdown = Markdown::new(content)
                       .extensions(AUTOLINK)
                       .extensions(FENCED_CODE)
                       .extensions(TABLES);

    let mut html = hoedown::Html::new(html::Flags::empty(), 0);
    let html_string = html.render(&markdown).to_str().unwrap().to_owned();

    let title = try!(metadata["title"]
                         .as_str()
                         .ok_or(PostParseError::MetadataSyntax("could not find key 'title'"
                                                                   .to_owned())))
                    .to_owned();

    let date_string = try!(metadata["date"]
                               .as_str()
                               .ok_or(PostParseError::MetadataSyntax("could not find 'key'"
                                                                         .to_owned())));
    let date = try!(UTC.datetime_from_str(date_string, "%l:%M%P %m/%d/%y"));

    Ok(Post {
        title: title,
        date: date,
        html: Html(html_string),
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
