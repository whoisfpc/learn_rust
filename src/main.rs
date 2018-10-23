extern crate learn_rust;

use std::env;
use std::process;

use learn_rust::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = learn_rust::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
