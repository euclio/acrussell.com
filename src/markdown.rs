//! Utilities for rendering Markdown.

use std::ops::Deref;

use hoedown::{self, Render};
use hoedown::{AUTOLINK, FENCED_CODE, TABLES};
use hoedown::renderer::html;

use serde;

/// An owned string containing Markdown.
#[derive(Debug, Clone)]
pub struct Markdown(String);

impl Markdown {
    /// Wraps a `String` in a `Markdown` instance, indicating that the string contains markdown.
    pub fn new(inner: String) -> Markdown {
        Markdown(inner)
    }
}

impl Deref for Markdown {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}


impl serde::Deserialize for Markdown {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        let string = try!(String::deserialize(deserializer));
        Ok(Markdown(string))
    }
}

/// An owned string containing HTML.
#[derive(Debug, Clone)]
pub struct Html(String);

impl Html {
    /// Wraps a `String` in an `Html` instance, indicating that the string contains HTML..
    pub fn new(inner: String) -> Html {
        Html(inner)
    }
}

impl Deref for Html {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl serde::Serialize for Html {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(self.deref())
    }
}

/// Renders a markdown string into unescaped HTML.
pub fn render_html(markdown: &str) -> Html {
    let markdown = hoedown::Markdown::new(markdown).extensions(AUTOLINK | FENCED_CODE | TABLES);

    let mut html = hoedown::Html::new(html::Flags::empty(), 0);
    Html(html.render(&markdown).to_str().unwrap().to_owned())
}
