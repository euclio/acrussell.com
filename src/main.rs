use std::process;

fn main() {
    env_logger::init();

    if let Err(ref e) = website::generate() {
        eprintln!("error: {}", e);

        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace: {:?}", backtrace);
        }

        process::exit(1);
    }
}
