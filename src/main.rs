use std::{env, process};

use mygrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = mygrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
