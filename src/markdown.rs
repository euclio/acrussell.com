//! Utilities for rendering Markdown.

use std::ops::Deref;

use hoedown::{self, Markdown, Render};
use hoedown::{AUTOLINK, FENCED_CODE, TABLES};
use hoedown::renderer::html;

/// An owned string containing HTML.
#[derive(Debug, Clone)]
pub struct Html(String);

impl Deref for Html {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::serde::Serialize for Html {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ::serde::Serializer,
    {
        serializer.serialize_str(self.deref())
    }
}

/// Renders a markdown string into unescaped HTML.
pub fn render_html(markdown: &str) -> Html {
    let markdown = Markdown::new(markdown).extensions(AUTOLINK | FENCED_CODE | TABLES);

    let mut html = hoedown::Html::new(html::Flags::empty(), 0);
    Html(html.render(&markdown).to_str().unwrap().to_owned())
}
