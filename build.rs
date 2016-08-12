use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env!("OUT_DIR"));

    // Copy static assets to target directory.
    let copy_status = Command::new("cp")
        .arg("-r")
        .arg("static")
        .arg(out_dir.to_str().unwrap())
        .status()
        .unwrap();
    assert!(copy_status.success());

    // Compile SCSS
    let scss_out = &out_dir.join("styles.css");
    let sass_status = Command::new("sass")
        .arg("scss/main.scss")
        .arg(scss_out.to_str().unwrap())
        .status()
        .unwrap_or_else(|e| {
            match e.kind() {
                ErrorKind::NotFound => panic!("sass not installed"),
                _ => panic!(e),
            }
        });
    assert!(sass_status.success(), "there was a problem compiling scss");

    // Apply PostCSS
    let output = &out_dir.join("static/css/");
    fs::create_dir_all(&output).unwrap();
    let postcss_status = Command::new("postcss")
        .args(&["--use", "autoprefixer"])
        .arg(scss_out.to_str().unwrap())
        .args(&["-d", output.to_str().unwrap()])
        .status()
        .unwrap_or_else(|e| {
            match e.kind() {
                ErrorKind::NotFound => panic!("postcss not installed"),
                _ => panic!(e),
            }
        });
    assert!(postcss_status.success(),
            "there was a problem postprocessing css");
}
