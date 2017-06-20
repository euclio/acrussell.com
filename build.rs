#![recursion_limit = "1024"]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[macro_use]
extern crate error_chain;

mod errors {
    use std::io;

    error_chain! {
        foreign_links {
            Io(io::Error);
        }
    }
}

use errors::*;

fn run() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let status = Command::new("cp")
        .args(&["-r", "static", out_dir.to_str().unwrap()])
        .stderr(Stdio::inherit())
        .status()
        .chain_err(|| "failed to run `cp`")?;
    if !status.success() {
        bail!("failed to copy static files to the target directory");
    }

    let scss_out = out_dir.join("styles.css");
    let status = Command::new("sass")
        .args(&["scss/main.scss", scss_out.to_str().unwrap()])
        .stderr(Stdio::inherit())
        .status()
        .chain_err(|| "failed to run `sass`")?;
    if !status.success() {
        bail!("SCSS compilation failed");
    }

    let css_out = out_dir.join("static/css/");
    fs::create_dir_all(&css_out)?;
    let status = Command::new("postcss")
        .args(&["--use", "autoprefixer"])
        .arg(scss_out.to_str().unwrap())
        .args(&["-d", css_out.to_str().unwrap()])
        .status()
        .chain_err(|| "failed to run `postcss`")?;
    if !status.success() {
        bail!("failed postprocessing CSS");
    }

    Ok(())
}

quick_main!(run);
