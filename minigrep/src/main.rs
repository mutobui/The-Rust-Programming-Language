use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("there is something wrong when parsing: {:?}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("there is something wrong when reading the file: {:?}", err);
        process::exit(1);
    }
}


