use minigrep::{run, Config};
use std::env;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parse arguments: {err}");
        std::process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        std::process::exit(1);
    }
}
