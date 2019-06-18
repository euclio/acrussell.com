use std::error::Error;
use std::fs::{self, File};

use handlebars::Handlebars;
use pulldown_cmark::Parser;

fn main() -> Result<(), Box<dyn Error>>{
    let handlebars = Handlebars::new();

    let content = {
        let md = fs::read_to_string("README.md")?;
        let mut html = String::new();
        let parser = Parser::new(&md);
        pulldown_cmark::html::push_html(&mut html, parser);
        html
    };

    let mut template = File::open("index.hbs")?;
    let mut out = File::create("dist/index.html")?;
    handlebars.render_template_source_to_write(&mut template, &content, &mut out)?;

    Ok(())
}
