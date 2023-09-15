use std::{env, process};

use mygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = mygrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
