use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = Path::new(env!("OUT_DIR")).to_owned();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).to_owned();

    // Compile SCSS
    let scss_out = &out_dir.join("styles.css");
    Command::new("sass")
        .arg("scss/main.scss")
        .arg(scss_out.as_path().to_str().unwrap())
        .spawn()
        .unwrap_or_else(|e| {
            match e.kind() {
                ErrorKind::NotFound => panic!("sass not installed"),
                _ => panic!(e),
            }
        });

    // Apply PostCSS
    let output = &root.join("static/css/");
    fs::create_dir_all(&output).unwrap();
    Command::new("postcss")
        .args(&["--use", "autoprefixer"])
        .arg(scss_out.as_path().to_str().unwrap())
        .args(&["-d", output.as_path().to_str().unwrap()])
        .spawn()
        .unwrap_or_else(|e| {
            match e.kind() {
                ErrorKind::NotFound => panic!("postcss not installed"),
                _ => panic!(e),
            }
        });
}
