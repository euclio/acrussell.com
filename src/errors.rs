//! Contains convenient type aliases and traits for error handling.

#![allow(deprecated)]

use std::io;
use std::path::PathBuf;

error_chain! {
    errors {
        PostParse(p: PathBuf){
            description("could not parse blog post"),
            display("could not parse blog post: {}", p.display())
        }
    }

    foreign_links {
        GitHub(hubcaps::Error);
        Io(io::Error);
        TemplateRender(handlebars::RenderError);
        UrlParse(url::ParseError);
        Yaml(serde_yaml::Error);
    }
}
