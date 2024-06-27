use minigrep::{Config, run};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parse arguments: {err}");
        std::process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        std::process::exit(1);
    }
}
