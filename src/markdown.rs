//! Utilities for rendering Markdown.

use std::ops::Deref;

use hoedown::renderer::html;
use hoedown::{self, Render};
use hoedown::{AUTOLINK, FENCED_CODE, TABLES};

use serde;

/// An owned string containing Markdown.
#[derive(Debug, Clone)]
pub struct Markdown(String);

impl Deref for Markdown {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for Markdown {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(Markdown(string))
    }
}

/// An owned string containing HTML.
#[derive(Debug, Clone)]
pub struct Html(String);

impl Deref for Html {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl serde::Serialize for Html {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
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
