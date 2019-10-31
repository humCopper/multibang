use std::{env, process};

use multibang::Config;
fn main() {
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Parsing Error: {}", err);
            process::exit(1);
        });

    if let Err(e) = multibang::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}