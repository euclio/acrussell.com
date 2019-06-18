//! My personal website.

#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate dotenv_codegen;

mod config;
mod errors;
mod helpers;
mod markdown;
mod projects;

use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use handlebars::Handlebars;
use log::*;
use serde_json::json;

use crate::errors::*;

/// The entrypoint of the generator. Generates and writes static HTML based on the configuration.
pub fn generate() -> Result<()> {
    let config = config::load("config.yaml")?;
    let dist = PathBuf::from("dist");

    let mut handlebars = Handlebars::new();

    handlebars.register_helper("join", Box::new(crate::helpers::join));
    handlebars
        .register_templates_directory(".hbs", "templates")
        .unwrap();

    let image_urls = fs::read_dir("dist/images/slideshow")?
        .map(|entry| entry.map(|e| {
            let path = e.path();
            Path::new("/").join(path.strip_prefix("dist/").unwrap())
        }))
        .collect::<std::result::Result<Vec<_>, io::Error>>()?;

    let projects = projects::load("projects.yaml")?;

    let context = json!({
        "pages": config.pages,
        "image_urls": image_urls,
        "projects": projects,
        "resume_link": config.resume_link.as_str(),
    });

    for page in config.pages {
        let html_out = dist.join(page.html_path());
        if let Some(parent) = html_out.parent() {
            if !parent.exists() {
                fs::create_dir(parent)?;
            }
        }

        info!("writing {}", html_out.display());

        let mut out = File::create(html_out)?;
        handlebars.render_to_write(&page.template, &context, &mut out)?;
    }

    Ok(())
}
