use chrono::NaiveDateTime;
use serde::Serialize;

use crate::schema::{post_content, posts};

use crate::blog::human_readable_format;

/// A post that has been parsed from markdown and is ready for insertion into the database.
#[derive(Debug, Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    /// The title of the post.
    pub title: &'a str,

    /// The date the post was written.
    pub date: NaiveDateTime,

    /// The markdown of the post rendered as HTML.
    pub html: String,

    /// A brief summary of the blog post.
    ///
    /// This is implemented as the first 300 lines of the blog post with HTML tags removed.
    pub summary: String,

    /// The URL to the blog post.
    pub url: String,

    /// Used to disambiguate the blog post from others written the same day in the url. Created by
    /// lower-casing the entire title and replacing spaces with dashes.
    pub slug: String,
}

/// A blog post queried from the database.
#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub html: String,
    pub date: NaiveDateTime,
    pub url: String,
}

/// Used for full-text-search queries.
#[derive(Debug, Queryable, Insertable)]
#[table_name = "post_content"]
pub struct PostContent {
    pub docid: i32,
    pub title: String,
    pub content: String,
}

/// A brief summary of a blog post.
#[derive(Debug, Serialize, Queryable)]
pub struct Summary {
    /// The title of the post.
    pub title: String,

    /// The date that the post was written.
    #[serde(with = "human_readable_format")]
    pub date: NaiveDateTime,

    /// A short preview of the post.
    pub summary: String,

    /// A URL to reach the full post.
    pub url: String,
}

/// Information needed to construct a link to a post.
#[derive(Debug, Serialize, Queryable)]
pub struct PostLink {
    /// The title of the linked post.
    pub title: String,

    /// The URL linking to the post.
    pub url: String,
}
